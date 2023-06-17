use glob::glob;
use lazy_static::lazy_static;
use std::borrow::Borrow;
use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fmt};

lazy_static! {
    /// The location of the TFLM source directory.
    pub static ref TENSORFLOW_LOCATION: PathBuf = PathBuf::from("c/tflite-micro");

    /// The target triple of the current build.
    pub static ref TARGET: String = env::var("TARGET").expect("Could not get target triple!");

    /// The host triple of the current build.
    pub static ref HOST: String = env::var("HOST").expect("Could not get host triple!");

    /// The output directory of the current build.
    pub static ref OUT_DIR: PathBuf = PathBuf::from(env::var("OUT_DIR").expect("Could not get output directory!"));
}

impl fmt::Display for TENSORFLOW_LOCATION {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", TENSORFLOW_LOCATION.display())
    }
}

impl fmt::Display for TARGET {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", TARGET)
    }
}

impl AsRef<Path> for TENSORFLOW_LOCATION {
    fn as_ref(&self) -> &Path {
        self.borrow()
    }
}

pub trait CompilationBuilder {
    fn flag(&mut self, s: &str) -> &mut Self;
    fn define(&mut self, var: &str, val: Option<&str>) -> &mut Self;

    /// Build flags for tensorflow micro sources
    fn tensorflow_build_setup(&mut self) -> &mut Self {
        let build = self
            .flag("-fno-common") // Separate sections for each global variable -- keep no_std happy
            .flag("-fno-rtti") // No runtime type information -- not necessary.
            .flag("-fmessage-length=0")
            .flag("-fno-exceptions") // Remove exception handling because this is an embedded environment.
            .flag("-fno-unwind-tables")
            .flag("-ffunction-sections")
            .flag("-fdata-sections")
            .flag("-funsigned-char")
            .flag("-MMD")
            .flag("-std=c++17") // TODO: This should be c++11, but the compiler throws a hissy fit.
            .flag("-fno-delete-null-pointer-checks")
            .flag("-fomit-frame-pointer")
            .flag("-fpermissive")
            .flag("-fno-use-cxa-atexit")
            .flag("-fno-short-enums") // Use a full word for enums, this should match clang's behaviour.
            .define("TF_LITE_STATIC_MEMORY", None)
            .define("TF_LITE_MCU_DEBUG_LOG", None)
            .define("GEMMLOWP_ALLOW_SLOW_SCALAR_FALLBACK", None);

        // warnings on by default
        let build = if cfg!(feature = "no-c-warnings") {
            build.flag("-w")
        } else {
            build
                .flag("-Wvla")
                .flag("-Wall")
                .flag("-Wextra")
                .flag("-Wno-unused-parameter")
                .flag("-Wno-missing-field-initializers")
                .flag("-Wno-write-strings")
                .flag("-Wno-sign-compare")
                .flag("-Wunused-function")
        };

        if TARGET.starts_with("thumb") {
            // unaligned accesses are usually a poor idea on ARM cortex-m
            build.flag("-mno-unaligned-access")
        } else {
            build
        }
    }
}

impl CompilationBuilder for cpp_build::Config {
    fn flag(&mut self, s: &str) -> &mut Self {
        self.flag(s)
    }
    fn define(&mut self, var: &str, val: Option<&str>) -> &mut Self {
        self.define(var, val)
    }
}

impl CompilationBuilder for cc::Build {
    fn flag(&mut self, s: &str) -> &mut Self {
        self.flag(s)
    }
    fn define(&mut self, var: &str, val: Option<&str>) -> &mut Self {
        self.define(var, val)
    }
}

/// Get the flatbuffers source directory.
pub fn flatbuffers_include_dir() -> PathBuf {
    TENSORFLOW_LOCATION.join("tensorflow/lite/micro/tools/make/downloads/flatbuffers/include")
}

/// Check if the build is cross-compiling.
pub fn is_cross_compiling() -> bool {
    TARGET.to_string() != HOST.to_string()
}

pub fn run_command_or_fail<P1, P2, S>(dir: P1, cmd: P2, args: &[S])
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
    S: Borrow<str> + AsRef<OsStr>,
{
    let cmd = cmd.as_ref();
    let dir = dir.as_ref();

    let cmd = if cmd.components().count() > 1 && cmd.is_relative() {
        // If `cmd` is a relative path (and not a bare command that should be
        // looked up in PATH), absolutize it relative to `dir`, as otherwise the
        // behavior of std::process::Command is undefined.
        // https://github.com/rust-lang/rust/issues/37868
        dir.join(cmd)
            .canonicalize()
            .expect("canonicalization failed")
    } else {
        PathBuf::from(cmd)
    };
    eprintln!(
        "Running command: \"{} {}\" in dir: {}",
        cmd.display(),
        args.join(" "),
        dir.display()
    );
    let ret = Command::new(cmd).current_dir(dir).args(args).status();
    match ret.map(|status| (status.success(), status.code())) {
        Ok((true, _)) => {}
        Ok((false, Some(c))) => panic!("Command failed with error code {}", c),
        Ok((false, None)) => panic!("Command got killed"),
        Err(e) => panic!("Command failed with error: {}", e),
    }
}

/// Gets the result of a command. Used to query GCC for locations of libraries and such.
pub fn get_command_result(command: &mut Command) -> io::Result<String> {
    command.output().map(|output| {
        if output.status.success() {
            String::from_utf8(output.stdout).expect("Output should be UTF-8!")
        } else {
            panic!("Couldn't read output from GCC.")
        }
    })
}

/// Return a Vec of all *.cc files in `path`, excluding those that have a
/// name containing 'test.cc'
pub fn get_files_glob(path: PathBuf) -> Vec<String> {
    let mut paths: Vec<String> = vec![];

    for entry in glob(&path.to_string_lossy()).unwrap() {
        let p: PathBuf = entry.unwrap();
        paths.push(p.to_string_lossy().to_string());
    }

    paths
        .into_iter()
        .filter(|p| !p.contains("test.cc")) // Get rid of test files.
        .filter(|p| !p.contains("debug_log.cc")) // Get rid of debugging.
        .filter(|p| !p.contains("frontend_memmap")) // TODO: figure this out!
        .filter(|p| !p.contains("frontend_main")) // TODO: figure this out!
        .collect()
}
