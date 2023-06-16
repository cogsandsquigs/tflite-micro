# tflite-micro

Rust bindings to TensorFlow Lite for Microcontrollers, but updated!

Much of this code was copied or inspired by [Recognition2/tfmicro](https://github.com/Recognition2/tfmicro), so kudos to them for figuring out how to build the library.

## Building

**Prerequisites:** See [`bindgen`'s requirements](https://rust-lang.github.io/rust-bindgen/requirements.html) for the required dependencies to compile bindings for the library. Also see [the TensorFlow Lite for Microcontrollers documentation](https://www.tensorflow.org/lite/microcontrollers) for the required dependencies to compile the library itself.

Also, for some reason, the tensorflow build process requires certain python packages to build. All of these are listed in the `requirements.txt` file in the root of this repository. You can install them with `pip install -r requirements.txt`.
