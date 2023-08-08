# OmniZK Compiler Overview

## General
The function `wrap_main_with_io` can evaluate a rust function
with the correct signature such that it can be evaluated with
a public input, secret input, and (to be added) initial memory.
This is a very powerful tool for testing.

In rough terms, the compilation is Rust -> WASM -> IR -> TASM.

## Rust --> WASM
in the compilation process, a `.wasm` file is created
somewhere in your OS' temp-directory, which is `/tmp/`
on a Linux system. This `.wasm` file is a binary, but
it can be read with the command-line tool `wasm2wat`.

Internally in the compiler, `wat` can be generated
from wasm through 
`let wat = wasmprinter::print_bytes(source).unwrap();`
where `source` is the byte slice.

This wat code might look like this:
```wat
(module
  (type (;0;) (func (result i64)))
  (type (;1;) (func (param i64)))
  (type (;2;) (func))
  (type (;3;) (func (param i64 i64) (result i64)))
  (import "env" "c2zk_stdlib_pub_input" (func $c2zk_stdlib_pub_input (type 0)))
  (import "env" "c2zk_stdlib_pub_output" (func $c2zk_stdlib_pub_output (type 1)))
  (import "env" "c2zk_stdlib_secret_input" (func $c2zk_stdlib_secret_input (type 0)))
  (func $__main (type 2)
    call $main_add)
  (func $add (type 3) (param i64 i64) (result i64)
    local.get 1
    local.get 0
    i64.add)
  (func $main_add (type 2)
    call $pub_input
    call $pub_input
    call $add
    call $secret_input
    call $add
    call $pub_output)
  (func $pub_input (type 0) (result i64)
    call $c2zk_stdlib_pub_input)
  (func $pub_output (type 1) (param i64)
    local.get 0
    call $c2zk_stdlib_pub_output)
  (func $secret_input (type 0) (result i64)
    call $c2zk_stdlib_secret_input)
  (memory (;0;) 16)
  (global $__stack_pointer (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048576))
  (global (;2;) i32 (i32.const 1048576))
  (export "memory" (memory 0))
  (export "__main" (func $__main))
  (export "main_add" (func $main_add))
  (export "add" (func $add))
  (export "pub_input" (func $pub_input))
  (export "secret_input" (func $secret_input))
  (export "pub_output" (func $pub_output))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2)))
```

Internally, the compiled WASM code is represented as a slice of
bytes though.

## WASM --> IR
The function `translate` in `crates/frontend/src/translate.rs`
converts from WASM to the intermediate represenation, IR. This
seems to be just a list of functions.

A function called `run_ir_passes` is called on this IR.


## IR --> TASM
The IR to TASM conversion takes place in the `compile_module`
function in `crates/codegen-tritonvm/src/codegen.rs`.

The compilation from WASM to TASM takes place inside the
`check_triton` function in 
`crates/codegen-tritonvm/src/codegen/sem_tests.rs`. 

The `out_source` contains the TASM code.



#[allow(clippy::type_complexity)]
pub fn wrap_main_with_io(
    main_func: &'static dyn Fn(),
) -> Box<dyn Fn(Vec<u64>, Vec<u64>) -> Vec<u64>> {
    Box::new(|input: Vec<u64>, secret_input: Vec<u64>| {
        c2zk_stdlib::io_native::init_io(input, secret_input);
        main_func();
        c2zk_stdlib::io_native::get_pub_output()
    })
}

## Resources
### WASM
- https://v8.dev/blog/wasm-decompile

