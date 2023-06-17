use crate::utils::*;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// Links the `libm` library. This is necessary because the `tflite-micro` library
/// depends on `libm`, but the `tflite-micro` build script doesn't link it.
pub fn link_libm() {
    // If we're cross-compiling, we have to jump through a couple more hoops to include libm.
    if is_cross_compiling() {
        println!("b");

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

/// Build inline C++ code.
pub fn build_inline_cpp(tensorflow_location: &Path) {
    println!("Building inline C++...");
    let start = Instant::now();

    cpp_build::Config::new()
        .include(tensorflow_location)
        .include(tensorflow_location.join("third_party/flatbuffers/include"))
        .include(tensorflow_location.join("third_party/gemmlowp"))
        .tensorflow_build_setup()
        .cpp_link_stdlib(None)
        //.flag("-std=c++14")
        .build("src/lib.rs");

    println!("Building inline C++ took {:?}", start.elapsed());
}

/// Build the tensorflow library.
pub fn build_tflm(tensorflow_location: &Path) {
    // The path to the tensorflow directory in the source directory.
    let tflite: PathBuf = tensorflow_location.join("tensorflow");
    // The path to the third party directory where helper libraries exist.
    let tf_third_party_dir = tensorflow_location.join("third_party");
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
            .include(tensorflow_location)
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
