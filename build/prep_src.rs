use super::utils::*;
use std::path::PathBuf;
use std::time::Instant;

/// Move the tensorflow source to the build directory so that we can build it
/// more cleanly. Returns the tensorflow directory within the source directory.
pub fn prepare_tensorflow_source() -> PathBuf {
    println!("Preparing TFLM source...");

    let start: Instant = Instant::now();
    let tflm_out_dir = OUT_DIR.join("tflite-micro");

    // Begin the build process: checkout the TFLM library git repo if it's not
    // already there and update it if it is.
    checkout_tflm_repo();

    // If the folder is already there *AND* we're building it, delete it.
    // Note that below, we only check !tflm_out_dir.exists() because if we're
    // building it from scratch, we delete the folder anyways.
    if tflm_out_dir.exists() && cfg!(feature = "build") {
        println!("Deleting old TFLM source...");
        fs_extra::dir::remove(tflm_out_dir.as_path()).expect("Unable to delete old TFLM source");
    }

    // // If the old source is not there (either we're building it or it's not there to begin with),
    // // copy the source to the build directory.
    // if !tflm_out_dir.exists() {
    //     let copy_dir = fs_extra::dir::CopyOptions {
    //         content_only: false,
    //         overwrite: true,
    //         skip_exist: false,
    //         buffer_size: 65536,
    //         copy_inside: false,
    //         depth: 0,
    //     };

    //     // Copy directory
    //     println!("Copying TF from {:?}", TENSORFLOW_LOCATION.display());
    //     println!("Copying TF to {:?}", OUT_DIR.display());

    //     fs_extra::dir::copy(TENSORFLOW_LOCATION.as_path(), OUT_DIR.as_path(), &copy_dir)
    //         .expect("Unable to copy tensorflow");
    // }

    // Thanks to @trylaarsdam for this tip to make this compile:
    // If we're building the library, we need to run the `create_tflm_tree.py` script to
    // generate the source files -- this script integrates TFLM into a project (apparently).
    // If we're not building the library, we don't need to do this.
    if !tflm_out_dir.exists() {
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

/// `git checkout` the TFLM library if it's not already there. Also download thrid-party tools
/// and/or install pip packages.
fn checkout_tflm_repo() {
    // Check if the TFLM library is already checked out. If not, download it.
    if !TENSORFLOW_LOCATION.join("LICENSE").exists() {
        eprintln!("Setting up TFLM git submodule...");
        run_command_or_fail(".", "git", &["submodule", "update", "--init"]);
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
