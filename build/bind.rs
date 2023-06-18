use crate::utils::*;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// This generates "tflite_types.rs" containing structs and enums which are
/// inter-operable with rust
pub fn bindgen_tflite_types(tensorflow_location: &Path) {
    use bindgen::*;

    let tflite_types_name = OUT_DIR.join("tflite_types.rs");

    if !tflite_types_name.exists() || cfg!(feature = "build") {
        println!("Running bindgen...");

        let start = Instant::now();

        let bindings = bindgen_cross_builder()
            // Configuration
            .allowlist_recursively(true) // Allow types under types.
            .prepend_enum_name(false) // Don't preprend the enum name to variants -- it's ugly!
            .impl_debug(true) // Let us debug the types.
            .with_codegen_config(CodegenConfig::TYPES) // TODO: figure out what this does.
            .layout_tests(false)
            .enable_cxx_namespaces()
            .derive_copy(true)
            .derive_default(true)
            .derive_partialeq(true)
            .derive_eq(true)
            .size_t_is_usize(true)
            .generate_inline_functions(true) // Generate inline functions.
            .use_core() // Use core instead of std.
            .ctypes_prefix("cty") // Use cty instead of std.
            .default_non_copy_union_style(NonCopyUnionStyle::ManuallyDrop) // Fix issue where `ManuallyDrop` is not wrapping values.
            .default_enum_style(EnumVariation::Rust {
                non_exhaustive: false,
            })
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
            // Headers
            .header(format!(
                "{}/tensorflow/lite/micro/kernels/micro_ops.h",
                tensorflow_location.to_string_lossy()
            ))
            .header(format!(
                "{}/tensorflow/lite/micro/micro_op_resolver.h",
                tensorflow_location.to_string_lossy()
            ))
            .header(format!(
                "{}/tensorflow/lite/micro/tflite_bridge/micro_error_reporter.h",
                tensorflow_location.to_string_lossy()
            ))
            .header(format!(
                "{}/tensorflow/lite/micro/micro_interpreter.h",
                tensorflow_location.to_string_lossy()
            ))
            // Inclusions
            .clang_arg(format!(
                "-include{}/tensorflow/lite/micro/micro_common.h",
                tensorflow_location.to_string_lossy()
            ))
            .clang_arg(format!("-I{}", tensorflow_location.to_string_lossy()))
            .clang_arg(format!(
                "-I{}/tensorflow",
                tensorflow_location.to_string_lossy()
            ))
            .clang_arg(format!(
                "-I{}/third_party/flatbuffers/include",
                tensorflow_location.to_string_lossy()
            )) // -> flatbuffers/flatbuffers.h
            .clang_arg(format!(
                "-I{}/tensorflow/lite",
                tensorflow_location.to_string_lossy()
            ))
            .clang_arg(format!(
                "-I{}/tensorflow/lite/micro",
                tensorflow_location.to_string_lossy()
            )) // -> micro/micro_common.h
            .clang_arg(format!(
                "-I{}/tensorflow/lite/c",
                tensorflow_location.to_string_lossy()
            )) // -> c/common.h
            // Others
            .clang_arg("-fretain-comments-from-system-headers") // Allow for parsing comments to create docs.
            .clang_arg("-DGEMMLOWP_ALLOW_SLOW_SCALAR_FALLBACK")
            .clang_arg("-xc++")
            .clang_arg("-std=c++17"); // C++17 is required for flatbuffers

        // Dump the preprocessed input for debugging. Stored in __bindgen.* files.
        bindings
            .dump_preprocessed_input()
            .expect("Unable to dump preprocessed input!");

        let bindings = bindings.generate().expect("Unable to generate bindings!");

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
