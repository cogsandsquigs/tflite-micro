#![cfg_attr(not(feature = "std"), no_std)]

// #[macro_use]
// extern crate log;
// #[macro_use]
// extern crate cpp;

pub mod bindings;
// mod frontend;
// mod interop;
// mod micro_error_reporter;
// mod micro_interpreter;
// mod micro_op_resolver;
// mod model;
// mod operators;
// mod tensor;
// pub use frontend::Frontend;
// pub use micro_interpreter::MicroInterpreter;
// pub use micro_op_resolver::{AllOpResolver, MutableOpResolver};
// pub use model::Model;

// /// Error type for tfmicro
// #[derive(Clone, Copy, PartialEq, Debug)]
// pub enum Error {
//     /// The model failed verification checks
//     InvalidModel,
//     /// An error occoured when instantiating the interpreter
//     InterpreterInitError,
//     /// An error occoured when allocating tensors in the tensor arena
//     AllocateTensorsError,
//     /// The length of the supplied slice was different to expect
//     InputDataLenMismatch,
//     /// The element type of the underlying data is not implemented by this crate
//     ElementTypeUnimplemented,
//     /// An error occoured converting some raw string to UTF8
//     Utf8Error,
// }

// /// The status resulting from a TensorFlow operation
// #[derive(Clone, Copy, PartialEq, Debug)]
// pub enum Status {
//     Ok,
//     Error,
//     DelegateError,
//     ApplicationError,
//     DelegateDataNotFound,
//     DelegateDataWriteError,
//     DelegateDataReadError,
//     UnresolvedOps,
//     Cancelled,
// }

// impl From<bindings::TfLiteStatus> for Status {
//     fn from(status: bindings::TfLiteStatus) -> Self {
//         use Status::*;

//         match status {
//             bindings::TfLiteStatus::kTfLiteOk => Ok,
//             bindings::TfLiteStatus::kTfLiteError => Error,
//             bindings::TfLiteStatus::kTfLiteDelegateError => DelegateError,
//             bindings::TfLiteStatus::kTfLiteApplicationError => ApplicationError,
//             bindings::TfLiteStatus::kTfLiteDelegateDataNotFound => DelegateDataNotFound,
//             bindings::TfLiteStatus::kTfLiteDelegateDataWriteError => DelegateDataWriteError,
//             bindings::TfLiteStatus::kTfLiteDelegateDataReadError => DelegateDataReadError,
//             bindings::TfLiteStatus::kTfLiteUnresolvedOps => UnresolvedOps,
//             bindings::TfLiteStatus::kTfLiteCancelled => Cancelled,
//         }
//     }
// }
