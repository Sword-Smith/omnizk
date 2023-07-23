use expect_test::expect;

use crate::codegen::sem_tests::check_wasm;

// #[ignore = "reason"]
#[test]
fn test_fib() {
    let input = vec![25];
    let secret_input = vec![];
    let expected_output = vec![75025];
    let native_output = ozk_rust_wasm_tests_helper::wrap_main_with_io(
        &ozk_rust_wasm_tests_fib::fib::fib_seq,
    )(input.clone(), secret_input.clone());
    assert_eq!(native_output, expected_output);
    let wasm_bytes = ozk_rust_wasm_tests_helper::compile_rust_wasm_tests("fib-bin", "fib");
    check_wasm(
        &wasm_bytes,
        input,
        secret_input,
        expected_output,
        expect![[r#"
            (module
              (type (;0;) (func (result i64)))
              (type (;1;) (func (param i64)))
              (type (;2;) (func))
              (import "env" "ozk_stdlib_pub_input" (func $ozk_stdlib_pub_input (;0;) (type 0)))
              (import "env" "ozk_stdlib_pub_output" (func $ozk_stdlib_pub_output (;1;) (type 1)))
              (import "env" "ozk_stdlib_secret_input" (func $ozk_stdlib_secret_input (;2;) (type 0)))
              (func $__main (;3;) (type 2)
                call $fib_seq
              )
              (func $fib_seq (;4;) (type 2)
                (local i32 i32 i32 i32)
                block ;; label = @1
                  block ;; label = @2
                    call $pub_input
                    i32.wrap_i64
                    local.tee 0
                    br_if 0 (;@2;)
                    i32.const 0
                    local.set 1
                    br 1 (;@1;)
                  end
                  local.get 0
                  i32.const 7
                  i32.and
                  local.set 2
                  block ;; label = @2
                    block ;; label = @3
                      local.get 0
                      i32.const -1
                      i32.add
                      i32.const 7
                      i32.ge_u
                      br_if 0 (;@3;)
                      i32.const 1
                      local.set 0
                      i32.const 0
                      local.set 1
                      br 1 (;@2;)
                    end
                    local.get 0
                    i32.const -8
                    i32.and
                    local.set 3
                    i32.const 1
                    local.set 0
                    i32.const 0
                    local.set 1
                    loop ;; label = @3
                      local.get 0
                      local.get 1
                      i32.add
                      local.tee 1
                      local.get 0
                      i32.add
                      local.tee 0
                      local.get 1
                      i32.add
                      local.tee 1
                      local.get 0
                      i32.add
                      local.tee 0
                      local.get 1
                      i32.add
                      local.tee 1
                      local.get 0
                      i32.add
                      local.tee 0
                      local.get 1
                      i32.add
                      local.tee 1
                      local.get 0
                      i32.add
                      local.set 0
                      local.get 3
                      i32.const -8
                      i32.add
                      local.tee 3
                      br_if 0 (;@3;)
                    end
                  end
                  local.get 2
                  i32.eqz
                  br_if 0 (;@1;)
                  local.get 1
                  local.set 3
                  loop ;; label = @2
                    local.get 0
                    local.tee 1
                    local.get 3
                    i32.add
                    local.set 0
                    local.get 1
                    local.set 3
                    local.get 2
                    i32.const -1
                    i32.add
                    local.tee 2
                    br_if 0 (;@2;)
                  end
                end
                local.get 1
                i64.extend_i32_u
                call $pub_output
              )
              (func $pub_input (;5;) (type 0) (result i64)
                call $ozk_stdlib_pub_input
              )
              (func $pub_output (;6;) (type 1) (param i64)
                local.get 0
                call $ozk_stdlib_pub_output
              )
              (func $secret_input (;7;) (type 0) (result i64)
                call $ozk_stdlib_secret_input
              )
              (table (;0;) 1 1 funcref)
              (memory (;0;) 16)
              (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
              (global (;1;) i32 i32.const 1048576)
              (global (;2;) i32 i32.const 1048576)
              (export "memory" (memory 0))
              (export "__main" (func $__main))
              (export "fib_seq" (func $fib_seq))
              (export "main" (func $__main))
              (export "pub_input" (func $pub_input))
              (export "pub_output" (func $pub_output))
              (export "secret_input" (func $secret_input))
              (export "__data_end" (global 1))
              (export "__heap_base" (global 2))
            )"#]],
        expect![[r#"
            call __main
            halt
            ozk_stdlib_pub_input:
            read_io
            return
            ozk_stdlib_pub_output:
            push 0
            call globals_get
            push -4
            add
            push 0
            call globals_set
            push 0
            call globals_get
            swap 1
            write_mem
            pop
            push 0
            call globals_get
            read_mem
            swap 1
            pop
            write_io
            push 0
            call globals_get
            push 4
            add
            push 0
            call globals_set
            return
            ozk_stdlib_secret_input:
            divine
            return
            __main:
            call init_mem_for_locals
            call _ZN28ozk_rust_wasm_tests_bundle13fib7fib_seq17h2de4f42386e38ca7E
            return
            _ZN28ozk_rust_wasm_tests_bundle13fib7fib_seq17h2de4f42386e38ca7E:
            push -1
            call globals_get
            push -16
            add
            push 0
            call globals_set
            call _ZN28ozk_rust_wasm_tests_bundle13fib7fib_seq17h2de4f42386e38ca7E_l0_b0
            push -1
            call globals_get
            push 8
            add
            read_mem
            swap 1
            pop
            call _ZN11ozk_stdlib10pub_output17h88275f26ad69ad8bE
            push -1
            call globals_get
            push 16
            add
            push 0
            call globals_set
            return
            _ZN11ozk_stdlib9pub_input17had951df5cd6b60c2E:
            call ozk_stdlib_pub_input
            return
            _ZN11ozk_stdlib10pub_output17h88275f26ad69ad8bE:
            push -1
            call globals_get
            push -4
            add
            push 0
            call globals_set
            push 0
            call globals_get
            swap 1
            write_mem
            pop
            push 0
            call globals_get
            read_mem
            swap 1
            pop
            call ozk_stdlib_pub_output
            push 0
            call globals_get
            push 4
            add
            push 0
            call globals_set
            return
            secret_input:
            call ozk_stdlib_secret_input
            return
            init_mem_for_locals:
            push 00000000002147483647
            push 0
            call globals_set
            return
            globals_set:
            push 4
            mul
            push 00000000002147483647
            add
            swap1
            write_mem
            pop
            pop
            return
            _ZN28ozk_rust_wasm_tests_bundle13fib7fib_seq17h2de4f42386e38ca7E_l0_b0:
            call _ZN28ozk_rust_wasm_tests_bundle13fib7fib_seq17h2de4f42386e38ca7E_l0_b0_l1_b0
            call next_br_propagation
            skiz
            return
            push 0
            call globals_get
            push 12
            add
            read_mem
            swap 1
            pop
            push 7
            and
            push 0
            call globals_get
            swap 1
            swap 1
            push 4
            add
            swap 1
            write_mem
            pop
            pop
            call _ZN28ozk_rust_wasm_tests_bundle13fib7fib_seq17h2de4f42386e38ca7E_l0_b0_l1_b1
            call next_br_propagation
            skiz
            return
            push 0
            call globals_get
            push 4
            add
            read_mem
            swap 1
            pop
            push 0
            eq
            skiz
            return
            push 0
            call globals_get
            push 8
            add
            read_mem
            swap 1
            pop
            push 0
            call globals_get
            swap 1
            write_mem
            pop
            pop
            call _ZN28ozk_rust_wasm_tests_bundle13fib7fib_seq17h2de4f42386e38ca7E_l0_b0_l1_b2
            call next_br_propagation
            skiz
            return
            return
            _ZN28ozk_rust_wasm_tests_bundle13fib7fib_seq17h2de4f42386e38ca7E_l0_b0_l1_b0:
            call _ZN11ozk_stdlib9pub_input17had951df5cd6b60c2E
            dup0
            push -1
            call globals_get
            swap 1
            swap 1
            push 12
            add
            swap 1
            write_mem
            pop
            skiz
            return
            push 0
            push 0
            call globals_get
            swap 1
            swap 1
            push 8
            add
            swap 1
            write_mem
            pop
            push 2
            push 1
            call globals_set
            return
            return
            next_br_propagation:
            push 1
            call globals_get
            dup 0
            push 0
            eq
            skiz
            return
            push -1
            add
            dup 0
            push 1
            call globals_set
            return
            _ZN28ozk_rust_wasm_tests_bundle13fib7fib_seq17h2de4f42386e38ca7E_l0_b0_l1_b1:
            call _ZN28ozk_rust_wasm_tests_bundle13fib7fib_seq17h2de4f42386e38ca7E_l0_b0_l1_b1_l2_b0
            call next_br_propagation
            skiz
            return
            push 0
            call globals_get
            push 12
            add
            read_mem
            swap 1
            pop
            push 00000000002147483640
            and
            push 0
            call globals_get
            swap 1
            write_mem
            pop
            push 1
            push 0
            call globals_get
            swap 1
            swap 1
            push 12
            add
            swap 1
            write_mem
            pop
            push 0
            push 0
            call globals_get
            swap 1
            swap 1
            push 8
            add
            swap 1
            write_mem
            pop
            pop
            call _ZN28ozk_rust_wasm_tests_bundle13fib7fib_seq17h2de4f42386e38ca7E_l0_b0_l1_b1_l2_b1
            call next_br_propagation
            skiz
            return
            return
            _ZN28ozk_rust_wasm_tests_bundle13fib7fib_seq17h2de4f42386e38ca7E_l0_b0_l1_b1_l2_b0:
            push -1
            call globals_get
            push 12
            add
            read_mem
            swap 1
            pop
            push -1
            add
            push 7
            swap 1
            dup 1
            dup 1
            lt
            push 0
            eq
            swap 2
            swap 1
            eq
            add
            push 1
            eq
            skiz
            return
            push 1
            push 0
            call globals_get
            swap 1
            swap 1
            push 12
            add
            swap 1
            write_mem
            pop
            push 0
            push 0
            call globals_get
            swap 1
            swap 1
            push 8
            add
            swap 1
            write_mem
            pop
            push 2
            push 1
            call globals_set
            return
            return
            _ZN28ozk_rust_wasm_tests_bundle13fib7fib_seq17h2de4f42386e38ca7E_l0_b0_l1_b1_l2_b1:
            push -1
            call globals_get
            push 16
            add
            push 0
            call globals_get
            push 12
            add
            read_mem
            swap 1
            pop
            push 0
            call globals_get
            push 8
            add
            read_mem
            swap 1
            pop
            add
            dup 0
            push 0
            call globals_get
            swap 1
            swap 1
            push 8
            add
            swap 1
            write_mem
            pop
            push 0
            call globals_get
            push 12
            add
            read_mem
            swap 1
            pop
            add
            dup 0
            push 0
            call globals_get
            swap 1
            swap 1
            push 12
            add
            swap 1
            write_mem
            pop
            push 0
            call globals_get
            push 8
            add
            read_mem
            swap 1
            pop
            add
            dup 0
            push 0
            call globals_get
            swap 1
            swap 1
            push 8
            add
            swap 1
            write_mem
            pop
            push 0
            call globals_get
            push 12
            add
            read_mem
            swap 1
            pop
            add
            dup 0
            push 0
            call globals_get
            swap 1
            swap 1
            push 12
            add
            swap 1
            write_mem
            pop
            push 0
            call globals_get
            push 8
            add
            read_mem
            swap 1
            pop
            add
            dup 0
            push 0
            call globals_get
            swap 1
            swap 1
            push 8
            add
            swap 1
            write_mem
            pop
            push 0
            call globals_get
            push 12
            add
            read_mem
            swap 1
            pop
            add
            dup 0
            push 0
            call globals_get
            swap 1
            swap 1
            push 12
            add
            swap 1
            write_mem
            pop
            push 0
            call globals_get
            push 8
            add
            read_mem
            swap 1
            pop
            add
            dup 0
            push 0
            call globals_get
            swap 1
            swap 1
            push 8
            add
            swap 1
            write_mem
            pop
            push 0
            call globals_get
            push 12
            add
            read_mem
            swap 1
            pop
            add
            push 0
            call globals_get
            swap 1
            swap 1
            push 12
            add
            swap 1
            write_mem
            pop
            push 0
            call globals_get
            read_mem
            swap 1
            pop
            push -8
            add
            dup 0
            push 0
            call globals_get
            swap 1
            write_mem
            pop
            skiz
            recurse
            return
            fib_seq_l0_b0_l1_b2:
            push -1
            call globals_get
            push 12
            add
            read_mem
            swap 1
            pop
            dup 0
            push 0
            call globals_get
            swap 1
            swap 1
            push 8
            add
            swap 1
            write_mem
            pop
            push 0
            call globals_get
            read_mem
            swap 1
            pop
            add
            push 0
            call globals_get
            swap 1
            swap 1
            push 12
            add
            swap 1
            write_mem
            pop
            push 0
            call globals_get
            push 8
            add
            read_mem
            swap 1
            pop
            push 0
            call globals_get
            swap 1
            write_mem
            pop
            push 0
            call globals_get
            push 4
            add
            read_mem
            swap 1
            pop
            push -1
            add
            dup 0
            push 0
            call globals_get
            swap 1
            swap 1
            push 4
            add
            swap 1
            write_mem
            pop
            skiz
            recurse
            return
            globals_get:
            push -4
            mul
            push 00000000002147482623
            add
            read_mem
            swap 1
            pop
            return
            globals_set:
            push -4
            mul
            push 00000000002147482623
            add
            swap 1
            write_mem
            pop
            return"#]],
    )
}
