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

We're going to use this code as the canonical Rust source code:
```rust
use c2zk_stdlib::*;

#[inline(never)]
#[no_mangle]
fn add(a: u64, b: u64) -> u64 {
    a + b
}

#[no_mangle]
pub fn main_add() {
    let a = pub_input();
    let b = pub_input();
    let r = add(a, b);
    let c = secret_input();
    let r2 = add(r, c);
    pub_output(r2);
}
```

This translates the following WASM (here presented as WebAssembly Text (WAT)) code:
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

A function called `run_ir_passes` is called on this IR. This processes the IR somehow.
The passes that are the default values for `TritonTargetConfig` are:

Box::<AndMinus8Pass>::default(),
Box::<LocalsToMemPass>::default(),
Box::<GlobalsToMemPass>::default(),
Box::<BlocksToFuncPass>::default(),
Box::<GlobalsToMemPass>::default(),
Box::<PseudoOpSubPass>::default(),

IR defines the following instruction set:
```Rust
pub enum Inst {
    Unreachable,
    Nop,
    Call {
        func_idx: FuncIndex,
    },
    End,
    Return,
    Loop {
        block_type: BlockType,
    },
    Block {
        blockty: BlockType,
    },
    BrIf {
        relative_depth: u32,
    }, // branch out of the current block if the top of the stack is not zero
    Br {
        relative_depth: u32,
    },
    I32Const {
        value: i32,
    },
    I64Const {
        value: i64,
    },
    GlobalGet {
        global_idx: GlobalIndex,
    },
    GlobalSet {
        global_idx: GlobalIndex,
    },
    LocalGet {
        local_idx: u32,
    },
    LocalTee {
        local_idx: u32,
    },
    LocalSet {
        local_idx: u32,
    },
    I32Load {
        offset: u32,
    },
    I32Store {
        offset: u32,
    },
    I32Add,
    I32Sub,
    I32Mul,
    I32Eqz,
    I32WrapI64,
    I32And,
    I32GeU,
    I64Add,
    I64Mul,
    I64Eqz,
    I64And,
    I64GeU,
    I64Ne,
    I64Eq,
    I64ExtendI32U,
    PubInputRead,
    PubOutputWrite,
    SecretInputRead,
    // Extra (besides the wasm instructions)
    // -------------------------------------
    /// 0..=15, swap the top of stack with the idx-th element from the top of stack
    Swap {
        idx: u8,
    },
    /// 0..=15, copy the idx-th element to the top of the stack
    Dup {
        idx: u8,
    },
    // Extention instructions for target arch
    Ext(Ext),
}
```

After all IR passes, the `add` function 
from the canonical example looks like this in the IR
```omnizk-IR
        Func {
            name: "add",
            sig: FuncType {
                params: [
                    I64,
                    I64,
                ],
                results: [
                    I64,
                ],
            },
            locals: [],
            ins: [
                I32Const {
                    value: -1,
                },
                Call {
                    func_idx: FuncIndex(
                        10,
                    ),
                },
                Dup {
                    idx: 0,
                },
                Swap {
                    idx: 2,
                },
                I32Store {
                    offset: 0,
                },
                I32Const {
                    value: -4,
                },
                I32Add,
                Dup {
                    idx: 0,
                },
                Swap {
                    idx: 2,
                },
                I32Store {
                    offset: 0,
                },
                I32Const {
                    value: -4,
                },
                I32Add,
                I32Const {
                    value: -1,
                },
                Call {
                    func_idx: FuncIndex(
                        11,
                    ),
                },
                I32Const {
                    value: -1,
                },
                Call {
                    func_idx: FuncIndex(
                        10,
                    ),
                },
                I32Load {
                    offset: 4,
                },
                I32Const {
                    value: -1,
                },
                Call {
                    func_idx: FuncIndex(
                        10,
                    ),
                },
                I32Load {
                    offset: 8,
                },
                I32Add,
                I32Const {
                    value: -1,
                },
                Call {
                    func_idx: FuncIndex(
                        10,
                    ),
                },
                I32Const {
                    value: 8,
                },
                I32Add,
                I32Const {
                    value: -1,
                },
                Call {
                    func_idx: FuncIndex(
                        11,
                    ),
                },
                End,
            ],
            comments: {},
        },
       
```

Problem: How does the `FuncIndex` translate to function name?
Answer: Can be read from the `ir_module` though its `function_idx_by_name` method:
```rust
let func_idx: usize = ir_module.function_idx_by_name(&func.name()).unwrap().into();
```

### `module_translator`
Loops over the linear WASM data structure and produces a linear IR data structure for a `module`.

Uses a `ModuleBuilder` to construct the IR.

```rust
pub struct FuncIndex(u32);
pub struct TypeIndex(u32);
pub struct GlobalIndex(u32);
struct ModuleBuilder {
    types: Vec<FuncType>,
    start_func_idx: Option<FuncIndex>,
    functions: Vec<FuncBuilder>,
    import_functions: Vec<FuncBuilder>,
    import_func_body: ImportFuncBody,
    func_names: HashMap<FuncIndex, String>,
    func_types: HashMap<FuncIndex, TypeIndex>,
}
```

Problem: `parse_function_section` does not set the `func_name` value.

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

## IR Language
The IR 

## Resources
### WASM
- https://v8.dev/blog/wasm-decompile

