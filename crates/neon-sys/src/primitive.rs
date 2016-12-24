//! Facilities for working with primitive values.

use raw::{Local, Isolate};

extern "C" {

    /// Mutates the `out` argument provided to refer to the `v8::Undefined` object.
    #[link_name = "NeonSys_Primitive_Undefined"]
    pub fn undefined(out: &mut Local);

    /// Mutates the `out` argument provided to refer to the `v8::Null` object.
    #[link_name = "NeonSys_Primitive_Null"]
    pub fn null(out: &mut Local);

    /// Mutates the `out` argument provided to refer to the `v8::Boolean` object.
    #[link_name = "NeonSys_Primitive_Boolean"]
    pub fn boolean(out: &mut Local, b: bool);

    /// Gets the underlying value of a `v8::Boolean` object.
    #[link_name = "NeonSys_Primitive_BooleanValue"]
    pub fn boolean_value(p: Local) -> bool;

    /// Mutates the `out` argument provided to refer to a newly created `v8::Integer` object.
    #[link_name = "NeonSys_Primitive_Integer"]
    pub fn integer(out: &mut Local, isolate: *mut Isolate, x: i32);

    /// Indicates if the value is a 32-bit unsigned integer.
    #[link_name = "NeonSys_Primitive_IsUint32"]
    pub fn is_u32(p: Local) -> bool;

    /// Indicates if the value is a 32-bit signed integer.
    #[link_name = "NeonSys_Primitive_IsInt32"]
    pub fn is_i32(p: Local) -> bool;

    /// Gets the underlying value of a `v8::Integer` object.
    #[link_name = "NeonSys_Primitive_IntegerValue"]
    pub fn integer_value(p: Local) -> i64;

    /// Mutates the `out` argument provided to refer to a newly created `v8::Number` object.
    #[link_name = "NeonSys_Primitive_Number"]
    pub fn number(out: &mut Local, isolate: *mut Isolate, v: f64);

    /// Gets the underlying value of a `v8::Number` object.
    #[link_name = "NeonSys_Primitive_NumberValue"]
    pub fn number_value(p: Local) -> f64;
}
