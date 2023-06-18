mod bind;
mod compile;
mod prep_src;
mod utils;

use bind::*;
use compile::*;
use prep_src::*;

/// The buildscript that builds the C++ code. Yay!
fn main() {
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
