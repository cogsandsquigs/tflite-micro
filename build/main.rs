mod bind;
mod c;
mod utils;

use bind::*;
use c::*;
use std::path::PathBuf;
use std::time::Instant;
use utils::*;

/// `git checkout` the TFLM library if it's not already there. Also download thrid-party tools
/// and/or install pip packages.
fn update_tflm_repo() {
    // Check if the TFLM library is already checked out. If not, download it.
    if !TENSORFLOW_LOCATION.join("LICENSE").exists() {
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

/// The buildscript that builds the C++ code. Yay!
fn main() {
    // Begin the build process: checkout the TFLM library if it's not already there.
    update_tflm_repo();

    let tensorflow_location = prepare_tensorflow_source();

    // Tell cargo to link `libm` as a dependency for tflite-micro.
    link_libm();

    // Build the bindings.
    bindgen_tflite_types(&tensorflow_location);

    // Build inline C++.
    build_inline_cpp(&tensorflow_location);

    // Build the tensorflow library.
    build_tflm(&tensorflow_location);
}
