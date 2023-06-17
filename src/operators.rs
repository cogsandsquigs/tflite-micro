/// Operators for Tensorflow micro
///
/// See lite/micro/kernels/all_ops_resolver.cc
use crate::micro_op_resolver::MutableOpResolver;

cpp! {{
    #include "tensorflow/lite/micro/kernels/micro_ops.h"
    #include "tensorflow/lite/micro/micro_mutable_op_resolver.h"
    #include "tensorflow/lite/mutable_op_resolver.h"
    #include "tensorflow/lite/c/common.h"
    #include "tensorflow/lite/core/api/flatbuffer_conversions.h"
    #include "tensorflow/lite/kernels/internal/compatibility.h"
    #include "tensorflow/lite/kernels/op_macros.h"
    #include "tensorflow/lite/micro/compatibility.h"
    #include "tensorflow/lite/micro/kernels/add.h"
    #include "tensorflow/lite/micro/kernels/conv.h"
    #include "tensorflow/lite/micro/kernels/depthwise_conv.h"
    #include "tensorflow/lite/micro/kernels/ethosu.h"
    #include "tensorflow/lite/micro/kernels/fully_connected.h"
    #include "tensorflow/lite/micro/kernels/micro_ops.h"
    #include "tensorflow/lite/micro/kernels/pooling.h"
    #include "tensorflow/lite/micro/kernels/reduce.h"
    #include "tensorflow/lite/micro/kernels/softmax.h"
    #include "tensorflow/lite/micro/micro_log.h"
    #include "tensorflow/lite/micro/micro_op_resolver.h"
    #include "tensorflow/lite/schema/schema_generated.h"
}}

impl MutableOpResolver {
    /// Use the FULLY_CONNECTED operator in this op resolver
    pub fn fully_connected(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_FULLY_CONNECTED,
                tflite::Register_FULLY_CONNECTED()
            );
        });

        self
    }

    /// Use the MAX_POOL_2D operator in this op resolver
    pub fn max_pool_2d(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_MAX_POOL_2D,
                tflite::Register_MAX_POOL_2D()
            );
        });

        self
    }

    /// Use the SOFTMAX operator in this op resolver
    pub fn softmax(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_SOFTMAX,
                tflite::Register_SOFTMAX()
            );
        });

        self
    }

    /// Use the LOGISTIC operator in this op resolver
    pub fn logistic(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_LOGISTIC,
                tflite::Register_LOGISTIC()
            );
        });

        self
    }

    /// Use the SVDF operator in this op resolver
    pub fn svdf(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_SVDF,
                tflite::Register_SVDF()
            );
        });

        self
    }

    /// Use the CONV_2D operator in this op resolver
    pub fn conv_2d(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_CONV_2D,
                tflite::Register_CONV_2D()
            );
        });

        self
    }

    /// Use the CONCATENATION operator in this op resolver
    pub fn concatenation(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_CONCATENATION,
                tflite::Register_CONCATENATION()
            );
        });

        self
    }

    /// Use the DEPTHWISE_CONV_2D operator in this op resolver
    pub fn depthwise_conv_2d(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_DEPTHWISE_CONV_2D,
                tflite::Register_DEPTHWISE_CONV_2D()
            );
        });

        self
    }

    /// Use the AVERAGE_POOL_2D operator in this op resolver
    pub fn average_pool_2d(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_AVERAGE_POOL_2D,
                tflite::Register_AVERAGE_POOL_2D()
            );
        });

        self
    }

    /// Use the ABS operator in this op resolver
    pub fn abs(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_ABS,
                tflite::Register_ABS()
            );
        });

        self
    }

    /// Use the SIN operator in this op resolver
    pub fn sin(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_SIN,
                tflite::Register_SIN()
            );
        });

        self
    }

    /// Use the COS operator in this op resolver
    pub fn cos(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_COS,
                tflite::Register_COS()
            );
        });

        self
    }

    /// Use the LOG operator in this op resolver
    pub fn log(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_LOG,
                tflite::Register_LOG()
            );
        });

        self
    }

    /// Use the SQRT operator in this op resolver
    pub fn sqrt(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_SQRT,
                tflite::Register_SQRT()
            );
        });

        self
    }

    /// Use the RSQRT operator in this op resolver
    pub fn rsqrt(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_RSQRT,
                tflite::Register_RSQRT()
            );
        });

        self
    }

    /// Use the SQUARE operator in this op resolver
    pub fn square(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_SQUARE,
                tflite::Register_SQUARE()
            );
        });

        self
    }

    /// Use the PRELU operator in this op resolver
    pub fn prelu(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_PRELU,
                tflite::Register_PRELU()
            );
        });

        self
    }

    /// Use the FLOOR operator in this op resolver
    pub fn floor(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_FLOOR,
                tflite::Register_FLOOR()
            );
        });

        self
    }

    /// Use the MAXIMUM operator in this op resolver
    pub fn maximum(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_MAXIMUM,
                tflite::Register_MAXIMUM()
            );
        });

        self
    }

    /// Use the MINIMUM operator in this op resolver
    pub fn minimum(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_MINIMUM,
                tflite::Register_MINIMUM()
            );
        });

        self
    }

    /// Use the ARG_MAX operator in this op resolver
    pub fn arg_max(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_ARG_MAX,
                tflite::Register_ARG_MAX()
            );
        });

        self
    }

    /// Use the ARG_MIN operator in this op resolver
    pub fn arg_min(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_ARG_MIN,
                tflite::Register_ARG_MIN()
            );
        });

        self
    }

    /// Use the LOGICAL_OR operator in this op resolver
    pub fn logical_or(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_LOGICAL_OR,
                tflite::Register_LOGICAL_OR()
            );
        });

        self
    }

    /// Use the LOGICAL_AND operator in this op resolver
    pub fn logical_and(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_LOGICAL_AND,
                tflite::Register_LOGICAL_AND()
            );
        });

        self
    }

    /// Use the LOGICAL_NOT operator in this op resolver
    pub fn logical_not(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_LOGICAL_NOT,
                tflite::Register_LOGICAL_NOT()
            );
        });

        self
    }

    /// Use the RESHAPE operator in this op resolver
    pub fn reshape(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_RESHAPE,
                tflite::Register_RESHAPE()
            );
        });

        self
    }

    /// Use the EQUAL operator in this op resolver
    pub fn equal(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_EQUAL,
                tflite::Register_EQUAL()
            );
        });

        self
    }

    /// Use the NOT_EQUAL operator in this op resolver
    pub fn not_equal(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_NOT_EQUAL,
                tflite::Register_NOT_EQUAL()
            );
        });

        self
    }

    /// Use the GREATER operator in this op resolver
    pub fn greater(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_GREATER,
                tflite::Register_GREATER()
            );
        });

        self
    }

    /// Use the GREATER_EQUAL operator in this op resolver
    pub fn greater_equal(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_GREATER_EQUAL,
                tflite::Register_GREATER_EQUAL()
            );
        });

        self
    }

    /// Use the LESS operator in this op resolver
    pub fn less(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_LESS,
                tflite::Register_LESS()
            );
        });

        self
    }

    /// Use the LESS_EQUAL operator in this op resolver
    pub fn less_equal(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_LESS_EQUAL,
                tflite::Register_LESS_EQUAL()
            );
        });

        self
    }

    /// Use the CEIL operator in this op resolver
    pub fn ceil(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_CEIL,
                tflite::Register_CEIL()
            );
        });

        self
    }

    /// Use the ROUND operator in this op resolver
    pub fn round(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_ROUND,
                tflite::Register_ROUND()
            );
        });

        self
    }

    /// Use the STRIDED_SLICE operator in this op resolver
    pub fn strided_slice(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_STRIDED_SLICE,
                tflite::Register_STRIDED_SLICE()
            );
        });

        self
    }

    /// Use the PACK operator in this op resolver
    pub fn pack(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_PACK,
                tflite::Register_PACK()
            );
        });

        self
    }

    /// Use the PAD operator in this op resolver
    pub fn pad(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_PAD,
                tflite::Register_PAD()
            );
        });

        self
    }

    /// Use the PADV2 operator in this op resolver
    pub fn padv2(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_PADV2,
                tflite::Register_PADV2()
            );
        });

        self
    }

    /// Use the SPLIT operator in this op resolver
    pub fn split(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_SPLIT,
                tflite::Register_SPLIT()
            );
        });

        self
    }

    /// Use the UNPACK operator in this op resolver
    pub fn unpack(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_UNPACK,
                tflite::Register_UNPACK()
            );
        });

        self
    }
    #[allow(clippy::should_implement_trait)]
    /// Use the NEG operator in this op resolver
    pub fn neg(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_NEG,
                tflite::Register_NEG()
            );
        });

        self
    }

    /// Use the ADD operator in this op resolver
    pub fn add(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_ADD,
                tflite::Register_ADD()
            );
        });

        self
    }

    /// Use the MUL operator in this op resolver
    pub fn mul(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_MUL,
                tflite::Register_MUL()
            );
        });

        self
    }

    /// Use the SUB operator in this op resolver
    pub fn sub(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_SUB,
                tflite::Register_SUB()
            );
        });

        self
    }

    /// Use the QUANTIZE operator in this op resolver
    pub fn quantize(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_QUANTIZE,
                tflite::Register_QUANTIZE()
            );
        });

        self
    }

    /// Use the DEQUANTIZE operator in this op resolver
    pub fn dequantize(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_DEQUANTIZE,
                tflite::Register_DEQUANTIZE()
            );
        });

        self
    }

    /// Use the RELU operator in this op resolver
    pub fn relu(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_RELU,
                tflite::Register_RELU()
            );
        });

        self
    }

    /// Use the RELU6 operator in this op resolver
    pub fn relu6(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_RELU6,
                tflite::Register_RELU6()
            );
        });

        self
    }

    /// Use the MEAN operator in this op resolver
    pub fn mean(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_MEAN,
                tflite::Register_MEAN()
            );
        });

        self
    }

    /// Use the RESIZE_NEAREST_NEIGHBOR operator in this op resolver
    pub fn resize_nearest_neighbor(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_RESIZE_NEAREST_NEIGHBOR,
                tflite::Register_RESIZE_NEAREST_NEIGHBOR()
            );
        });

        self
    }

    /// Use the L2_NORMALIZATION operator in this op resolver
    pub fn l2_normalization(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_L2_NORMALIZATION,
                tflite::Register_L2_NORMALIZATION()
            );
        });

        self
    }

    /// Use the TANH operator in this op resolver
    pub fn tanh(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBuiltin(
                tflite::BuiltinOperator_TANH,
                tflite::Register_TANH(),
                0
            );
        });

        self
    }
}
