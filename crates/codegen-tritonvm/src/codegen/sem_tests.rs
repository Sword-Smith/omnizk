//! Semantic equivalence tests for the TritonVM codegen.

#![allow(clippy::unwrap_used)]

mod add;
mod block;
mod fib;
mod func_call;
mod locals;

use std::collections::HashMap;

use c2zk_ir::pass::run_ir_passes;
use num_traits::Zero;
use triton_vm::op_stack::OpStack;
use triton_vm::stark::StarkHasher;
use triton_vm::BFieldElement;
use wasmtime::*;

use crate::compile_module;
use crate::TritonTargetConfig;

fn check_wasm(
    source: &[u8],
    input: Vec<u64>,
    secret_input: Vec<u64>,
    expected_output: Vec<u64>,
    expected_wat: expect_test::Expect,
    expected_triton: expect_test::Expect,
) {
    let wat = wasmprinter::print_bytes(source).unwrap();
    expected_wat.assert_eq(&wat);

    check_triton(
        source,
        input,
        secret_input,
        expected_output,
        expected_triton,
    );
}

fn check_triton(
    wasm: &[u8],
    input: Vec<u64>,
    secret_input: Vec<u64>,
    expected_output: Vec<u64>,
    expected_triton: expect_test::Expect,
) {
    use c2zk_frontend::translate;
    use c2zk_frontend::FrontendConfig;
    use c2zk_frontend::WasmFrontendConfig;

    let frontend = FrontendConfig::Wasm(WasmFrontendConfig::default());
    let triton_target_config = TritonTargetConfig::default();
    let mut ir_module = translate(wasm, frontend).unwrap();
    run_ir_passes(&mut ir_module, &triton_target_config.ir_passes);

    // print all functions
    // let mut func_idx: usize = ir_module.start_func_idx.into();
    // for func in ir_module.functions().iter() {
    //     let func_idx: usize = ir_module.function_idx_by_name(&func.name()).unwrap().into();
    //     println!("{func_idx}: {:#?}.", func);
    //     // func_idx += 1;
    // }

    let inst_buf = compile_module(ir_module, &triton_target_config).unwrap();
    let out_source = inst_buf.pretty_print();
    expected_triton.assert_eq(&out_source);
    let program = inst_buf.program();
    let input = input.into_iter().map(Into::into).collect();
    let secret_input = secret_input.into_iter().map(Into::into).collect();
    let execution_res = program.debug_terminal_state(input, secret_input, None, None);
    let last_state = match execution_res {
        Ok(vm_state) => vm_state,
        Err((err, vm_state)) => {
            panic!("VM execution failed with error \"{err}\". Last state was: {vm_state}")
        }
    };

    let out = last_state.public_output;
    assert_eq!(
        out.into_iter().map(|b| b.into()).collect::<Vec<u64>>(),
        expected_output
    );
    let mut expected_stack = vec![
        vec![BFieldElement::zero(); 11],
        program.hash::<StarkHasher>().values().to_vec(),
    ]
    .concat();
    expected_stack.reverse();
    assert_eq!(last_state.op_stack.stack, expected_stack);
}

fn _pp_trace(_trace: &[triton_vm::vm::VMState]) {
    // iterate over last n traces
    for state in _trace.iter() {
        //.rev().take(400).rev() {
        let s = format!(
            "{}: {}",
            &state.current_instruction().unwrap(),
            _pretty_print_vec_horiz(&_pretty_stack(&state.op_stack))
        );
        dbg!(s);
        let r = _pretty_print_ram_horiz(&state.ram);
        dbg!(r);
    }
}

fn check_wat(
    source: &str,
    input: Vec<u64>,
    secret_input: Vec<u64>,
    expected_output: Vec<u64>,
    expected_triton: expect_test::Expect,
) {
    struct Io {
        input: Vec<u64>,
        secret_input: Vec<u64>,
        output: Vec<u64>,
    }

    let mut store = Store::new(
        &Engine::default(),
        Io {
            input: input.clone().into_iter().rev().collect(),
            secret_input: secret_input.clone().into_iter().rev().collect(),
            output: Vec::new(),
        },
    );

    let wasm = wat::parse_str(source).unwrap();
    let module = Module::from_binary(store.engine(), &wasm).unwrap();

    let c2zk_stdlib_pub_input = Func::wrap(&mut store, |mut caller: Caller<'_, Io>| {
        caller.data_mut().input.pop().unwrap()
    });
    let c2zk_stdlib_pub_output =
        Func::wrap(&mut store, |mut caller: Caller<'_, Io>, output: i64| {
            caller.data_mut().output.push(output as u64);
        });
    let c2zk_stdlib_secret_input = Func::wrap(&mut store, |mut caller: Caller<'_, Io>| {
        caller.data_mut().secret_input.pop().unwrap()
    });
    let imports = [
        c2zk_stdlib_pub_input.into(),
        c2zk_stdlib_pub_output.into(),
        c2zk_stdlib_secret_input.into(),
    ];
    let _ = Instance::new(&mut store, &module, &imports).unwrap();

    assert_eq!(store.data().output, expected_output);
    check_triton(&wasm, input, secret_input, expected_output, expected_triton);
}

fn _pretty_print_ram_horiz(ram: &HashMap<BFieldElement, BFieldElement>) -> String {
    // TODO: sort by key (pointer)
    // ram.iter().map(|(k, v)| (k.into(), v.into())).collect()
    let mut s = String::new();
    for (k, v) in ram.iter() {
        s.push_str(&format!("{}:{} ", k.value(), v.value()));
    }
    s
}

fn _pretty_stack(stack: &OpStack) -> Vec<u64> {
    stack
        .stack
        .iter()
        .map(|b| b.value())
        // .filter(|v| *v != 0)
        .rev()
        .collect::<Vec<u64>>()
}

fn _pretty_print_vec_horiz<T: std::fmt::Display>(vec: &[T]) -> String {
    let mut s = String::new();
    for v in vec {
        s.push_str(&format!("{} ", v));
    }
    s
}
