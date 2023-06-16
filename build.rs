use glob::glob;
use std::borrow::Borrow;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;
use std::{env, io};

const TENSORFLOW_DIR_LOCATION: &str = "tflite-micro";

trait CompilationBuilder {
    fn flag(&mut self, s: &str) -> &mut Self;
    fn define(&mut self, var: &str, val: Option<&str>) -> &mut Self;

    /// Build flags for tensorflow micro sources
    fn tensorflow_build_setup(&mut self) -> &mut Self {
        let target = env::var("TARGET").unwrap_or_else(|_| "".to_string());

        let build = self
            .flag("-fno-rtti") // No Runtime type information
            .flag("-fmessage-length=0")
            .flag("-fno-exceptions")
            .flag("-fno-unwind-tables")
            .flag("-ffunction-sections")
            .flag("-fdata-sections")
            .flag("-funsigned-char")
            .flag("-MMD")
            .flag("-std=c++11")
            .flag("-fno-delete-null-pointer-checks")
            .flag("-fomit-frame-pointer")
            .flag("-fpermissive")
            .flag("-fno-use-cxa-atexit")
            // use a full word for enums, this should match clang's behaviour
            .flag("-fno-short-enums")
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

        if target.starts_with("thumb") {
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

fn run_command_or_fail<P, S>(dir: &str, cmd: P, args: &[S])
where
    P: AsRef<Path>,
    S: Borrow<str> + AsRef<OsStr>,
{
    let cmd = cmd.as_ref();
    let cmd = if cmd.components().count() > 1 && cmd.is_relative() {
        // If `cmd` is a relative path (and not a bare command that should be
        // looked up in PATH), absolutize it relative to `dir`, as otherwise the
        // behavior of std::process::Command is undefined.
        // https://github.com/rust-lang/rust/issues/37868
        PathBuf::from(dir)
            .join(cmd)
            .canonicalize()
            .expect("canonicalization failed")
    } else {
        PathBuf::from(cmd)
    };
    eprintln!(
        "Running command: \"{} {}\" in dir: {}",
        cmd.display(),
        args.join(" "),
        dir
    );
    let ret = Command::new(cmd).current_dir(dir).args(args).status();
    match ret.map(|status| (status.success(), status.code())) {
        Ok((true, _)) => {}
        Ok((false, Some(c))) => panic!("Command failed with error code {}", c),
        Ok((false, None)) => panic!("Command got killed"),
        Err(e) => panic!("Command failed with error: {}", e),
    }
}

/**
 * `git checkout` the TFLM library if it's not already there. Also download thrid-party tools
 * and/or install pip packages.
 */
fn prepare_tflm_dir() {
    if !Path::new("tflite-micro/LICENSE").exists() {
        eprintln!("Setting up TFLM git submodule...");
        run_command_or_fail(".", "git", &["submodule", "update", "--init"]);
    }

    if !Path::new(
        "tflite-micro/tensorflow/lite/micro/tools/make/downloads/flatbuffers/CONTRIBUTING.md",
    )
    .exists()
    {
        eprintln!("Building tensorflow micro example to fetch Tensorflow dependencies...");
        run_command_or_fail(
            TENSORFLOW_DIR_LOCATION,
            "make",
            &[
                "-f",
                "tensorflow/lite/micro/tools/make/Makefile",
                "test_micro_speech_test",
            ],
        );
    }

    run_command_or_fail(
        TENSORFLOW_DIR_LOCATION,
        "make",
        &[
            "-f",
            "tensorflow/lite/micro/tools/make/Makefile",
            "third_party_downloads",
        ],
    );
}

/**
 * Get the manifest directory.
 */
fn manifest_dir() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
}

/**
 * Get TFLM source directory.
 */

fn tflm_src_dir() -> PathBuf {
    manifest_dir().join("tflite-micro")
}

/**
 * Get the flatbuffers include directory.
 */
fn flatbuffers_include_dir() -> PathBuf {
    tflm_src_dir().join("tensorflow/lite/micro/tools/make/downloads/flatbuffers/include")
}

/**
 * Check if the build is cross-compiling.
 */
fn is_cross_compiling() -> bool {
    env::var("TARGET").unwrap() != env::var("HOST").unwrap() // TODO: remove `unwrap`s!
}

/**
 * Return a Vec of all *.cc files in `path`, excluding those that have a
 * name containing 'test.cc'
 */
fn get_files_glob(path: PathBuf) -> Vec<String> {
    let mut paths: Vec<String> = vec![];

    for entry in glob(&path.to_string_lossy()).unwrap() {
        let p: PathBuf = entry.unwrap();
        paths.push(p.to_string_lossy().to_string());
    }

    paths
        .into_iter()
        .filter(|p| !p.contains("test.cc"))
        .filter(|p| !p.contains("debug_log.cc"))
        .filter(|p| !p.contains("frontend_memmap"))
        .filter(|p| !p.contains("frontend_main"))
        .collect()
}

/**
 * Gets the result of a command. Used to query GCC for locations of libraries and such.
 */
fn get_command_result(command: &mut Command) -> io::Result<String> {
    command.output().map(|output| {
        if output.status.success() {
            String::from_utf8(output.stdout).expect("Output should be UTF-8!")
        } else {
            panic!("Couldn't read output from GCC.")
        }
    })
}

/**
 * Links the `libm` library. This is necessary because the `tflite-micro` library
 * depends on `libm`, but the `tflite-micro` build script doesn't link it.
 */
fn link_libm() {
    // If we're cross-compiling, we have to jump through a couple more hoops to include libm.
    if is_cross_compiling() {
        let mut gcc = cc::Build::new().get_compiler().to_command();

        // Find include directory used by the crosscompiler for libm.
        let libm_location = PathBuf::from(
            get_command_result(gcc.arg("--print-file-name=libm.a"))
                .expect("Error querying gcc for libm location"),
        );

        // Parent directory of the libm.a file is the include directory.
        let libm_path = libm_location.parent().unwrap();

        // Tell cargo to search for the libm.a file in the directory we found.
        println!(
            "cargo:rustc-link-search=native={}",
            libm_path.to_string_lossy()
        );
        // Tell cargo to link the libm.a file statically.
        println!("cargo:rustc-link-lib=static=m");
    }
    // Otherwise, we can just link it directly.
    else {
        // If we're not cross-compiling, we can just use the system libm.
        // Tell cargo to link the libm.a file.
        println!("cargo:rustc-link-lib=m");
    }
}

/**
 * Move the tensorflow source to the build directory so that we can build it
 * more cleanly. Returns the tensorflow directory within the source directory.
 */
fn prepare_tensorflow_source() -> PathBuf {
    println!("Moving TFLM source...");

    let start = Instant::now();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let tflm_out_dir = out_dir.join("tflite-micro");

    let copy_dir = fs_extra::dir::CopyOptions {
        content_only: false,
        overwrite: true,
        skip_exist: false,
        buffer_size: 65536,
        copy_inside: false,
        depth: 0,
    };

    if !tflm_out_dir.exists() || cfg!(feature = "build") {
        // Copy directory
        println!("Copying TF from {:?}", tflm_src_dir());
        println!("Copying TF to {:?}", out_dir);

        fs_extra::dir::copy(tflm_src_dir(), &out_dir, &copy_dir)
            .expect("Unable to copy tensorflow");
    }

    println!("Moving source took {:?}", start.elapsed());

    tflm_out_dir
}

/**
 * Build the tensorflow library.
 */
fn build_tflm() {
    // The path to the tensorflow directory in the source directory.
    let tflite = prepare_tensorflow_source().join("tensorflow");
    // The path to the output directory.
    let out_dir = env::var("OUT_DIR").unwrap();
    // The name of the final library to link.
    let tflm_lib_name = Path::new(&out_dir).join("tflm.a");

    // If we don't have the library, or we're building it, build and bind TFLM.
    if !tflm_lib_name.exists() || cfg!(feature = "build") {
        println!("Building TFLM...");

        let target: String = env::var("TARGET").unwrap_or_else(|_| "".to_string());
        let tfmicro_mdir = tflite.join("lite/micro/tools/make/");
        let start = Instant::now();

        let mut builder = cc::Build::new();
        let builder_ref = builder
            .cpp(true) // We're building with C++.
            .tensorflow_build_setup()
            .cpp_link_stdlib(None)
            //
            .include(tflite.parent().unwrap())
            .include(tfmicro_mdir.join("downloads"))
            .include(tfmicro_mdir.join("downloads/gemmlowp"))
            .include(tfmicro_mdir.join("downloads/flatbuffers/include"))
            .include(tfmicro_mdir.join("downloads/ruy"))
            //
            .files(get_files_glob(tflite.join("lite/micro/*.cc")))
            .files(get_files_glob(tflite.join("lite/micro/kernels/*.cc")))
            .files(get_files_glob(
                tflite.join("lite/micro/memory_planner/*.cc"),
            ))
            .files(get_files_glob(
                tflite.join("lite/experimental/microfrontend/lib/*.c"),
            ))
            .file(tflite.join("lite/c/common.c"))
            .file(tflite.join("lite/core/api/error_reporter.cc"))
            .file(tflite.join("lite/core/api/flatbuffer_conversions.cc"))
            .file(tflite.join("lite/core/api/op_resolver.cc"))
            .file(tflite.join("lite/core/api/tensor_utils.cc"))
            .file(tflite.join("lite/kernels/internal/quantization_util.cc"))
            .file(tflite.join("lite/kernels/kernel_util.cc"));

        // CMSIS-NN for ARM Cortex-M targets
        if target.starts_with("thumb") && target.contains("m-none-") && cfg!(feature = "cmsis-nn") {
            println!("Build includes CMSIS-NN.");
            let cmsis = tflite.join("lite/micro/tools/make/downloads/cmsis");

            builder_ref
                .files(get_files_glob(cmsis.join("CMSIS/NN/Source/*.c")))
                .include(cmsis.join("CMSIS/NN/Include"))
                .include(cmsis.join("CMSIS/DSP/Include"))
                .include(cmsis.join("CMSIS/Core/Include"));
        }

        // Micro frontend.
        builder_ref
            .include(tfmicro_mdir.join("downloads/kissfft"))
            .include(tfmicro_mdir.join("downloads/kissfft/tools"))
            .include(tflite.join("lite/experimental/microfrontend/lib"))
            .files(get_files_glob(
                tflite.join("lite/experimental/microfrontend/lib/*.cc"),
            ))
            .file(tfmicro_mdir.join("downloads/kissfft/kiss_fft.c"))
            .file(tfmicro_mdir.join("downloads/kissfft/tools/kiss_fftr.c"));

        // Compile!
        builder_ref.compile("tflm");

        println!(
            "Building tensorflow micro from source took {:?}",
            start.elapsed()
        );
    } else {
        println!("Didn't rebuild tensorflow micro, using {:?}", tflm_lib_name);

        println!("cargo:rustc-link-lib=static=tflm");
        println!("cargo:rustc-link-search=native={}", out_dir);
    }
}

/**
 * Build inline C++ code.
 */
fn build_inline_cpp() {
    println!("Building inline C++...");
    let start = Instant::now();

    cpp_build::Config::new()
        .include(tflm_src_dir())
        .include(flatbuffers_include_dir())
        .tensorflow_build_setup()
        .cpp_link_stdlib(None)
        //.flag("-std=c++14")
        .build("src/lib.rs");

    println!("Building inline C++ took {:?}", start.elapsed());
}

/**
 * The buildscript that builds the C++ code. Yay!
 */
fn main() {
    // // Tell cargo to look for shared libraries in the specified directory
    // println!("cargo:rustc-link-search=./tflite-micro");

    // // Tell cargo to invalidate the built crate whenever the wrapper changes
    // println!("cargo:rerun-if-changed=wrapper.h");

    // Begin the build process: checkout the TFLM library if it's not already there.
    prepare_tflm_dir();

    // Tell cargo to link `libm` as a dependency for tflite-micro.
    link_libm();

    // Build inline C++!
    build_inline_cpp();

    // Finally, build the tensorflow library.
    build_tflm();
}
