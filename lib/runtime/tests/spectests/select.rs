// Rust test file autogenerated with cargo build (build/spectests.rs).
// Please do NOT modify it by hand, as it will be reset on next build.
// Test based on spectests/select.wast
#![allow(
    warnings,
    dead_code
)]
use wabt::wat2wasm;
use std::{f32, f64};

use wasmer_runtime::types::Value;
use wasmer_runtime::{Instance, module::Module};
use wasmer_clif_backend::CraneliftCompiler;

use crate::spectests::_common::{
    spectest_importobject,
    NaNCheck,
};


// Line 1
fn create_module_1() -> Box<Instance> {
    let module_str = "(module
      (type (;0;) (func (param i32 i32 i32) (result i32)))
      (type (;1;) (func (param i64 i64 i32) (result i64)))
      (type (;2;) (func (param f32 f32 i32) (result f32)))
      (type (;3;) (func (param f64 f64 i32) (result f64)))
      (type (;4;) (func (param i32) (result i32)))
      (type (;5;) (func))
      (func (;0;) (type 0) (param i32 i32 i32) (result i32)
        get_local 0
        get_local 1
        get_local 2
        select)
      (func (;1;) (type 1) (param i64 i64 i32) (result i64)
        get_local 0
        get_local 1
        get_local 2
        select)
      (func (;2;) (type 2) (param f32 f32 i32) (result f32)
        get_local 0
        get_local 1
        get_local 2
        select)
      (func (;3;) (type 3) (param f64 f64 i32) (result f64)
        get_local 0
        get_local 1
        get_local 2
        select)
      (func (;4;) (type 4) (param i32) (result i32)
        unreachable
        i32.const 0
        get_local 0
        select)
      (func (;5;) (type 4) (param i32) (result i32)
        i32.const 0
        unreachable
        get_local 0
        select)
      (func (;6;) (type 5)
        unreachable
        select
        unreachable
        i32.const 0
        select
        unreachable
        i32.const 0
        i32.const 0
        select
        unreachable
        f32.const 0x0p+0 (;=0;)
        i32.const 0
        select
        unreachable)
      (export \"select_i32\" (func 0))
      (export \"select_i64\" (func 1))
      (export \"select_f32\" (func 2))
      (export \"select_f64\" (func 3))
      (export \"select_trap_l\" (func 4))
      (export \"select_trap_r\" (func 5))
      (export \"select_unreached\" (func 6)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new()).expect("WASM can't be compiled");
    module.instantiate(&spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_1(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 31
fn c1_l31_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c1_l31_action_invoke");
    let result = instance.call("select_i32", &[Value::I32(1 as i32), Value::I32(2 as i32), Value::I32(1 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(1 as i32))));
    result.map(|_| ())
}

// Line 32
fn c2_l32_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c2_l32_action_invoke");
    let result = instance.call("select_i64", &[Value::I64(2 as i64), Value::I64(1 as i64), Value::I32(1 as i32)]);
    assert_eq!(result, Ok(Some(Value::I64(2 as i64))));
    result.map(|_| ())
}

// Line 33
fn c3_l33_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c3_l33_action_invoke");
    let result = instance.call("select_f32", &[Value::F32((1.0f32)), Value::F32((2.0f32)), Value::I32(1 as i32)]);
    assert_eq!(result, Ok(Some(Value::F32((1.0f32)))));
    result.map(|_| ())
}

// Line 34
fn c4_l34_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c4_l34_action_invoke");
    let result = instance.call("select_f64", &[Value::F64((1.0f64)), Value::F64((2.0f64)), Value::I32(1 as i32)]);
    assert_eq!(result, Ok(Some(Value::F64((1.0f64)))));
    result.map(|_| ())
}

// Line 36
fn c5_l36_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c5_l36_action_invoke");
    let result = instance.call("select_i32", &[Value::I32(1 as i32), Value::I32(2 as i32), Value::I32(0 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(2 as i32))));
    result.map(|_| ())
}

// Line 37
fn c6_l37_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c6_l37_action_invoke");
    let result = instance.call("select_i32", &[Value::I32(2 as i32), Value::I32(1 as i32), Value::I32(0 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(1 as i32))));
    result.map(|_| ())
}

// Line 38
fn c7_l38_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c7_l38_action_invoke");
    let result = instance.call("select_i64", &[Value::I64(2 as i64), Value::I64(1 as i64), Value::I32(-1 as i32)]);
    assert_eq!(result, Ok(Some(Value::I64(2 as i64))));
    result.map(|_| ())
}

// Line 39
fn c8_l39_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c8_l39_action_invoke");
    let result = instance.call("select_i64", &[Value::I64(2 as i64), Value::I64(1 as i64), Value::I32(-252645136 as i32)]);
    assert_eq!(result, Ok(Some(Value::I64(2 as i64))));
    result.map(|_| ())
}

// Line 41
fn c9_l41_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c9_l41_action_invoke");
    let result = instance.call("select_f32", &[Value::F32(f32::from_bits(2143289344)), Value::F32((1.0f32)), Value::I32(1 as i32)]);
    let expected = f32::from_bits(2143289344);
                                if let Value::F32(result) = result.clone().unwrap().unwrap() {
                                assert!((result as f32).is_nan());
            assert_eq!((result as f32).is_sign_positive(), (expected as f32).is_sign_positive());
            } else {
              panic!("Unexpected result type {:?}", result);
            }
    result.map(|_| ())
}

// Line 42
fn c10_l42_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c10_l42_action_invoke");
    let result = instance.call("select_f32", &[Value::F32(f32::from_bits(2139226884)), Value::F32((1.0f32)), Value::I32(1 as i32)]);
    let expected = f32::from_bits(2139226884);
                                if let Value::F32(result) = result.clone().unwrap().unwrap() {
                                assert!((result as f32).is_nan());
            assert_eq!((result as f32).is_sign_positive(), (expected as f32).is_sign_positive());
            } else {
              panic!("Unexpected result type {:?}", result);
            }
    result.map(|_| ())
}

// Line 43
fn c11_l43_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c11_l43_action_invoke");
    let result = instance.call("select_f32", &[Value::F32(f32::from_bits(2143289344)), Value::F32((1.0f32)), Value::I32(0 as i32)]);
    assert_eq!(result, Ok(Some(Value::F32((1.0f32)))));
    result.map(|_| ())
}

// Line 44
fn c12_l44_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c12_l44_action_invoke");
    let result = instance.call("select_f32", &[Value::F32(f32::from_bits(2139226884)), Value::F32((1.0f32)), Value::I32(0 as i32)]);
    assert_eq!(result, Ok(Some(Value::F32((1.0f32)))));
    result.map(|_| ())
}

// Line 45
fn c13_l45_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c13_l45_action_invoke");
    let result = instance.call("select_f32", &[Value::F32((2.0f32)), Value::F32(f32::from_bits(2143289344)), Value::I32(1 as i32)]);
    assert_eq!(result, Ok(Some(Value::F32((2.0f32)))));
    result.map(|_| ())
}

// Line 46
fn c14_l46_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c14_l46_action_invoke");
    let result = instance.call("select_f32", &[Value::F32((2.0f32)), Value::F32(f32::from_bits(2139226884)), Value::I32(1 as i32)]);
    assert_eq!(result, Ok(Some(Value::F32((2.0f32)))));
    result.map(|_| ())
}

// Line 47
fn c15_l47_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c15_l47_action_invoke");
    let result = instance.call("select_f32", &[Value::F32((2.0f32)), Value::F32(f32::from_bits(2143289344)), Value::I32(0 as i32)]);
    let expected = f32::from_bits(2143289344);
                                if let Value::F32(result) = result.clone().unwrap().unwrap() {
                                assert!((result as f32).is_nan());
            assert_eq!((result as f32).is_sign_positive(), (expected as f32).is_sign_positive());
            } else {
              panic!("Unexpected result type {:?}", result);
            }
    result.map(|_| ())
}

// Line 48
fn c16_l48_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c16_l48_action_invoke");
    let result = instance.call("select_f32", &[Value::F32((2.0f32)), Value::F32(f32::from_bits(2139226884)), Value::I32(0 as i32)]);
    let expected = f32::from_bits(2139226884);
                                if let Value::F32(result) = result.clone().unwrap().unwrap() {
                                assert!((result as f32).is_nan());
            assert_eq!((result as f32).is_sign_positive(), (expected as f32).is_sign_positive());
            } else {
              panic!("Unexpected result type {:?}", result);
            }
    result.map(|_| ())
}

// Line 50
fn c17_l50_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c17_l50_action_invoke");
    let result = instance.call("select_f64", &[Value::F64(f64::from_bits(9221120237041090560)), Value::F64((1.0f64)), Value::I32(1 as i32)]);
    let expected = f64::from_bits(9221120237041090560);
                                if let Value::F64(result) = result.clone().unwrap().unwrap() {
                                assert!((result as f64).is_nan());
            assert_eq!((result as f64).is_sign_positive(), (expected as f64).is_sign_positive());
            } else {
              panic!("Unexpected result type {:?}", result);
            }
    result.map(|_| ())
}

// Line 51
fn c18_l51_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c18_l51_action_invoke");
    let result = instance.call("select_f64", &[Value::F64(f64::from_bits(9218868437227537156)), Value::F64((1.0f64)), Value::I32(1 as i32)]);
    let expected = f64::from_bits(9218868437227537156);
                                if let Value::F64(result) = result.clone().unwrap().unwrap() {
                                assert!((result as f64).is_nan());
            assert_eq!((result as f64).is_sign_positive(), (expected as f64).is_sign_positive());
            } else {
              panic!("Unexpected result type {:?}", result);
            }
    result.map(|_| ())
}

// Line 52
fn c19_l52_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c19_l52_action_invoke");
    let result = instance.call("select_f64", &[Value::F64(f64::from_bits(9221120237041090560)), Value::F64((1.0f64)), Value::I32(0 as i32)]);
    assert_eq!(result, Ok(Some(Value::F64((1.0f64)))));
    result.map(|_| ())
}

// Line 53
fn c20_l53_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c20_l53_action_invoke");
    let result = instance.call("select_f64", &[Value::F64(f64::from_bits(9218868437227537156)), Value::F64((1.0f64)), Value::I32(0 as i32)]);
    assert_eq!(result, Ok(Some(Value::F64((1.0f64)))));
    result.map(|_| ())
}

// Line 54
fn c21_l54_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c21_l54_action_invoke");
    let result = instance.call("select_f64", &[Value::F64((2.0f64)), Value::F64(f64::from_bits(9221120237041090560)), Value::I32(1 as i32)]);
    assert_eq!(result, Ok(Some(Value::F64((2.0f64)))));
    result.map(|_| ())
}

// Line 55
fn c22_l55_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c22_l55_action_invoke");
    let result = instance.call("select_f64", &[Value::F64((2.0f64)), Value::F64(f64::from_bits(9218868437227537156)), Value::I32(1 as i32)]);
    assert_eq!(result, Ok(Some(Value::F64((2.0f64)))));
    result.map(|_| ())
}

// Line 56
fn c23_l56_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c23_l56_action_invoke");
    let result = instance.call("select_f64", &[Value::F64((2.0f64)), Value::F64(f64::from_bits(9221120237041090560)), Value::I32(0 as i32)]);
    let expected = f64::from_bits(9221120237041090560);
                                if let Value::F64(result) = result.clone().unwrap().unwrap() {
                                assert!((result as f64).is_nan());
            assert_eq!((result as f64).is_sign_positive(), (expected as f64).is_sign_positive());
            } else {
              panic!("Unexpected result type {:?}", result);
            }
    result.map(|_| ())
}

// Line 57
fn c24_l57_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c24_l57_action_invoke");
    let result = instance.call("select_f64", &[Value::F64((2.0f64)), Value::F64(f64::from_bits(9218868437227537156)), Value::I32(0 as i32)]);
    let expected = f64::from_bits(9218868437227537156);
                                if let Value::F64(result) = result.clone().unwrap().unwrap() {
                                assert!((result as f64).is_nan());
            assert_eq!((result as f64).is_sign_positive(), (expected as f64).is_sign_positive());
            } else {
              panic!("Unexpected result type {:?}", result);
            }
    result.map(|_| ())
}

// Line 59
fn c25_l59_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c25_l59_action_invoke");
    let result = instance.call("select_trap_l", &[Value::I32(1 as i32)]);
    
    result.map(|_| ())
}

#[test]
fn c25_l59_assert_trap() {
    let mut instance = create_module_1();
    let result = c25_l59_action_invoke(&mut*instance);
    assert!(result.is_err());
}

// Line 60
fn c26_l60_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c26_l60_action_invoke");
    let result = instance.call("select_trap_l", &[Value::I32(0 as i32)]);
    
    result.map(|_| ())
}

#[test]
fn c26_l60_assert_trap() {
    let mut instance = create_module_1();
    let result = c26_l60_action_invoke(&mut*instance);
    assert!(result.is_err());
}

// Line 61
fn c27_l61_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c27_l61_action_invoke");
    let result = instance.call("select_trap_r", &[Value::I32(1 as i32)]);
    
    result.map(|_| ())
}

#[test]
fn c27_l61_assert_trap() {
    let mut instance = create_module_1();
    let result = c27_l61_action_invoke(&mut*instance);
    assert!(result.is_err());
}

// Line 62
fn c28_l62_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c28_l62_action_invoke");
    let result = instance.call("select_trap_r", &[Value::I32(0 as i32)]);
    
    result.map(|_| ())
}

#[test]
fn c28_l62_assert_trap() {
    let mut instance = create_module_1();
    let result = c28_l62_action_invoke(&mut*instance);
    assert!(result.is_err());
}

// Line 65
#[test]
fn c29_l65_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 9, 1, 7, 0, 1, 1, 65, 1, 27, 11];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

#[test]
fn test_module_1() {
    let mut instance = create_module_1();
    // We group the calls together
    start_module_1(&mut instance);
    c1_l31_action_invoke(&mut instance);
    c2_l32_action_invoke(&mut instance);
    c3_l33_action_invoke(&mut instance);
    c4_l34_action_invoke(&mut instance);
    c5_l36_action_invoke(&mut instance);
    c6_l37_action_invoke(&mut instance);
    c7_l38_action_invoke(&mut instance);
    c8_l39_action_invoke(&mut instance);
    c9_l41_action_invoke(&mut instance);
    c10_l42_action_invoke(&mut instance);
    c11_l43_action_invoke(&mut instance);
    c12_l44_action_invoke(&mut instance);
    c13_l45_action_invoke(&mut instance);
    c14_l46_action_invoke(&mut instance);
    c15_l47_action_invoke(&mut instance);
    c16_l48_action_invoke(&mut instance);
    c17_l50_action_invoke(&mut instance);
    c18_l51_action_invoke(&mut instance);
    c19_l52_action_invoke(&mut instance);
    c20_l53_action_invoke(&mut instance);
    c21_l54_action_invoke(&mut instance);
    c22_l55_action_invoke(&mut instance);
    c23_l56_action_invoke(&mut instance);
    c24_l57_action_invoke(&mut instance);
}
