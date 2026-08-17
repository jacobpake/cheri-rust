pub const SOFTFLOAT_LIBS: &[&str] = &[
    "softfloat",
    "softfloat3264convert",
    "softfloat32add",
    "softfloat32compare",
    "softfloat32convert",
    "softfloat64add",
    "softfloat64mul",
    "softfloat32div",
    "softfloat32mul",
    "softfloat32neg",
    "softfloat32sub",
    "softfloat64compare",
    "softfloat64convert",
    "softfloat64div",
    "softfloat64neg",
    "softfloat64sub",
    "softfloat3216convert",
    "softfloat32pow",
    "softfloat64pow",
];

pub const MATH_LIBS: &[&str] = &["stubs"];

pub const CORE_LIBS: &[&str] = &[
    "cheriot.token_library",
    "compartment_helpers",
    "atomic",
    "atomic1",
    "atomic2",
    "atomic4",
    "atomic8",
    "atomiccap",
    "locks",
    "crt",
    "debug",
    "freestanding",
];

pub const TEST_RUNNER_COMPARTMENT: &str = "test_runner";
pub const TEST_RUNNER_FIRMWARE: &str = "test_fw";
pub const TEST_RUNNER_CPP_WRAPPER: &str = "runner.cc";

pub const COMPARTMENT_LDSCRIPT: &str = "cheriot-rtos/sdk/compartment.ldscript";
