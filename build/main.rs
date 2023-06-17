mod utils;

use std::path::PathBuf;
use std::time::Instant;
use utils::*;

/// `git checkout` the TFLM library if it's not already there. Also download thrid-party tools
/// and/or install pip packages.
fn update_tflm_repo() {
    // Check if the TFLM library is already checked out. If not, download it.
    if TENSORFLOW_LOCATION.join("LICENSE").exists() {
        eprintln!("Setting up TFLM git submodule...");
        run_command_or_fail(".", "git", &["submodule", "update", "--remote"]);
    }

    // if !TENSORFLOW_LOCATION
    //     .join("tensorflow/lite/micro/tools/make/downloads/flatbuffers/CONTRIBUTING.md")
    //     .exists()
    // {
    //     eprintln!("Building tensorflow micro example to fetch Tensorflow dependencies...");
    //     run_command_or_fail(
    //         TENSORFLOW_LOCATION,
    //         "make",
    //         &[
    //             "-f",
    //             "tensorflow/lite/micro/tools/make/Makefile",
    //             "test_micro_speech_test",
    //         ],
    //     );
    // }

    // See #199: https://github.com/tensorflow/tflite-micro/issues/199
    run_command_or_fail(
        &TENSORFLOW_LOCATION,
        "make",
        &[
            "-f",
            "tensorflow/lite/micro/tools/make/Makefile",
            "third_party_downloads",
        ],
    );
}

/// Links the `libm` library. This is necessary because the `tflite-micro` library
/// depends on `libm`, but the `tflite-micro` build script doesn't link it.
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
/// Configure bindgen for cross-compiling
fn bindgen_cross_builder() -> bindgen::Builder {
    let builder = bindgen::Builder::default().clang_arg("--verbose");

    if is_cross_compiling() {
        // Setup target triple
        let builder = builder.clang_arg(format!("--target={}", TARGET));
        println!("Setting bindgen to cross compile to {}", TARGET);

        // Find the sysroot used by the crosscompiler, and pass this to clang
        let mut gcc = cc::Build::new().get_compiler().to_command();
        let path =
            get_command_result(gcc.arg("--print-sysroot")).expect("Error querying gcc for sysroot");
        let builder = builder.clang_arg(format!("--sysroot={}", path.trim()));

        // Add a path to the system headers for the target
        // compiler. Possibly we end up using a gcc header with clang
        // frontend, which is sketchy.
        let search_paths = cc::Build::new()
            .cpp(true)
            .get_compiler()
            .to_command()
            .arg("-E")
            .arg("-Wp,-v")
            .arg("-xc++")
            .arg(".")
            .output()
            .map(|output| {
                // We have to scrape the gcc console output to find where
                // the c++ headers are. If we only needed the c headers we
                // could use `--print-file-name=include` but that's not
                // possible.
                let gcc_out = String::from_utf8(output.stderr).expect("Error parsing gcc output");

                // Scrape the search paths
                let search_start = gcc_out.find("search starts here").unwrap();
                let search_paths: Vec<PathBuf> = gcc_out[search_start..]
                    .split('\n')
                    .map(|p| PathBuf::from(p.trim()))
                    .filter(|path| path.exists())
                    .collect();

                search_paths
            })
            .expect("Error querying gcc for include paths");

        // Add scraped paths to builder
        let mut builder = builder.detect_include_paths(false);
        for path in search_paths {
            builder = builder.clang_arg(format!("-I{}", path.to_string_lossy()));
        }
        builder
    } else {
        builder
    }
}

/// This generates "tflite_types.rs" containing structs and enums which are
/// inter-operable with rust
fn bindgen_tflite_types() {
    use bindgen::*;

    // let submodules = submodules();
    // let submodules_str = submodules.to_string_lossy();
    let tflite_types_name = OUT_DIR.join("tflite_types.rs");

    if !tflite_types_name.exists() || cfg!(feature = "build") {
        println!("Running bindgen");
        let start = Instant::now();

        let bindings = bindgen_cross_builder()
            .allowlist_recursively(true)
            .prepend_enum_name(false)
            .impl_debug(true)
            .with_codegen_config(CodegenConfig::TYPES)
            .layout_tests(false)
            .enable_cxx_namespaces()
            .derive_default(true)
            .size_t_is_usize(true)
            .use_core()
            .ctypes_prefix("cty")
            // Types
            .allowlist_type("tflite::MicroErrorReporter")
            .opaque_type("tflite::MicroErrorReporter")
            .allowlist_type("tflite::Model")
            .opaque_type("tflite::Model")
            .allowlist_type("tflite::MicroInterpreter")
            .opaque_type("tflite::MicroInterpreter")
            .allowlist_type("tflite::ops::micro::AllOpsResolver")
            .opaque_type("tflite::ops::micro::AllOpsResolver")
            .allowlist_type("TfLiteTensor")
            .allowlist_type("FrontendState")
            .allowlist_type("FrontendConfig")
            .allowlist_type("FrontendOutput")
            // Types - blocklist
            .blocklist_type("std")
            .blocklist_type("tflite::Interpreter_TfLiteDelegatePtr")
            .blocklist_type("tflite::Interpreter_State")
            .default_enum_style(EnumVariation::Rust {
                non_exhaustive: false,
            })
            .derive_partialeq(true)
            .derive_eq(true)
            .header("c/wrapper.h")
            .clang_arg(format!(
                "-I{}/tensorflow",
                TENSORFLOW_LOCATION.to_string_lossy()
            ))
            .clang_arg(format!(
                // -> flatbuffers/flatbuffers.h
                "-I{}",
                flatbuffers_include_dir().to_string_lossy()
            ))
            .clang_arg("-DGEMMLOWP_ALLOW_SLOW_SCALAR_FALLBACK")
            .clang_arg("-xc++")
            .clang_arg("-std=c++17"); // C++17 is required for flatbuffers

        let bindings = bindings.generate().expect("Unable to generate bindings");

        // Write the bindings to $OUT_DIR/tflite_types.rs
        let out_path = OUT_DIR.join("tflite_types.rs");
        bindings
            .write_to_file(out_path)
            .expect("Couldn't write bindings!");

        println!("Running bindgen took {:?}", start.elapsed());
    } else {
        println!("Didn't regenerate bindings");
    }
}

/// Build inline C++ code.
fn build_inline_cpp() {
    println!("Building inline C++...");
    let start = Instant::now();

    cpp_build::Config::new()
        .include(&TENSORFLOW_LOCATION)
        .include(
            TENSORFLOW_LOCATION
                .join("tensorflow/lite/micro/tools/make/downloads/flatbuffers/include"),
        )
        .tensorflow_build_setup()
        .cpp_link_stdlib(None)
        //.flag("-std=c++14")
        .build("src/lib.rs");

    println!("Building inline C++ took {:?}", start.elapsed());
}

/// Move the tensorflow source to the build directory so that we can build it
/// more cleanly. Returns the tensorflow directory within the source directory.
fn prepare_tensorflow_source() -> PathBuf {
    println!("Preparing TFLM source...");

    let start: Instant = Instant::now();
    let tflm_out_dir = OUT_DIR.join("tflite-micro");

    // let copy_dir = fs_extra::dir::CopyOptions {
    //     content_only: false,
    //     overwrite: true,
    //     skip_exist: false,
    //     buffer_size: 65536,
    //     copy_inside: false,
    //     depth: 0,
    // };

    // if !tflm_out_dir.exists() || cfg!(feature = "build") {
    //     // Copy directory
    //     println!("Copying TF from {:?}", tensorflow_src_dir());
    //     println!("Copying TF to {:?}", OUT_DIR);

    //     fs_extra::dir::copy(tensorflow_src_dir(), &OUT_DIR, &copy_dir)
    //         .expect("Unable to copy tensorflow");
    // }

    if !tflm_out_dir.exists() || cfg!(feature = "build") {
        // Thanks to @trylaarsdam for this tip to make this compile:
        run_command_or_fail(
            &TENSORFLOW_LOCATION,
            "python3",
            &[
                "tensorflow/lite/micro/tools/project_generation/create_tflm_tree.py",
                &tflm_out_dir.to_string_lossy(),
            ],
        );
    }

    println!("Preparing source took {:?}", start.elapsed());

    tflm_out_dir
}

/// Build the tensorflow library.
fn build_tflm() {
    // The path to the build directory.
    let tflite_parent = prepare_tensorflow_source();
    // The path to the tensorflow directory in the source directory.
    let tflite: PathBuf = tflite_parent.join("tensorflow");
    // The path to the third party directory where helper libraries exist.
    let tf_third_party_dir = tflite_parent.join("third_party");
    // The name of the final library to link.
    let tflm_lib_name = OUT_DIR.join("tflm.a");

    // If we don't have the library, or we're building it, build and bind TFLM.
    if !tflm_lib_name.exists() || cfg!(feature = "build") {
        println!("Building TFLM...");

        // TODO: See below!
        // let target: String = env::var("TARGET").unwrap_or_else(|_| "".to_string());
        let start = Instant::now();

        let mut builder = cc::Build::new();

        let builder_ref = builder
            .cpp(true) // We're building with C++.
            .tensorflow_build_setup()
            .cpp_link_stdlib(None)
            // Include helper libraries.
            .include(&tflite_parent)
            .include(&tf_third_party_dir)
            .include(tf_third_party_dir.join("gemmlowp"))
            .include(tf_third_party_dir.join("flatbuffers/include"))
            .include(tf_third_party_dir.join("ruy"))
            // Compile core TFLM files.
            .files(get_files_glob(tflite.join("lite/micro/*.cc")))
            .files(get_files_glob(tflite.join("lite/micro/kernels/*.cc")))
            .files(get_files_glob(
                tflite.join("lite/micro/memory_planner/*.cc"),
            ))
            .files(get_files_glob(
                tflite.join("lite/experimental/microfrontend/lib/*.c"),
            ))
            .file(tflite.join("lite/core/c/common.cc"))
            .file(tflite.join("lite/core/api/error_reporter.cc"))
            .file(tflite.join("lite/core/api/flatbuffer_conversions.cc"))
            .file(tflite.join("lite/core/api/op_resolver.cc"))
            .file(tflite.join("lite/core/api/tensor_utils.cc"))
            .file(tflite.join("lite/kernels/internal/quantization_util.cc"))
            .file(tflite.join("lite/kernels/kernel_util.cc"));

        // TODO: Add this back!
        // // CMSIS-NN for ARM Cortex-M targets
        // if target.starts_with("thumb") && target.contains("m-none-") && cfg!(feature = "cmsis-nn") {
        //     println!("Build includes CMSIS-NN.");
        //     let cmsis = tflite.join("lite/micro/tools/make/downloads/cmsis");

        //     builder_ref
        //         .files(get_files_glob(cmsis.join("CMSIS/NN/Source/*.c")))
        //         .include(cmsis.join("CMSIS/NN/Include"))
        //         .include(cmsis.join("CMSIS/DSP/Include"))
        //         .include(cmsis.join("CMSIS/Core/Include"));
        // }

        // Micro frontend.
        builder_ref
            .include(tf_third_party_dir.join("kissfft"))
            .include(tf_third_party_dir.join("kissfft/tools"))
            .include(tflite.join("lite/experimental/microfrontend/lib"))
            .file(tf_third_party_dir.join("kissfft/kiss_fft.c"))
            .file(tf_third_party_dir.join("kissfft/tools/kiss_fftr.c"))
            .files(get_files_glob(
                tflite.join("lite/experimental/microfrontend/lib/*.cc"),
            ));

        // Compile!
        builder_ref.compile("tflm");

        println!("Building TFLM from source took {:?}", start.elapsed());
    } else {
        println!("Not rebuilding TFLM, using {:?}", tflm_lib_name);

        // Tell cargo to link the tflm library.
        println!("cargo:rustc-link-lib=static=tflm");
        println!("cargo:rustc-link-search=native={}", OUT_DIR.display());
    }
}

/// The buildscript that builds the C++ code. Yay!
fn main() {
    // Begin the build process: checkout the TFLM library if it's not already there.
    update_tflm_repo();

    // Tell cargo to link `libm` as a dependency for tflite-micro.
    link_libm();

    // Build the bindings.
    bindgen_tflite_types();

    // Build inline C++.
    build_inline_cpp();

    // Build the tensorflow library.
    build_tflm();
}
