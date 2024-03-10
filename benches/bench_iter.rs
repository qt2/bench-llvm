#![feature(test)]
extern crate test;

use bench_llvm::{CodeGen, VM};
use test::Bencher;

const MOD: i64 = 100000007;
const N: i64 = 1000000;
const ANS: i64 = 59273026;

#[bench]
fn bench_native(b: &mut Bencher) {
    b.iter(|| {
        let mut a = 1;
        for i in 1..N {
            a = (a * i) % MOD;
        }
        assert_eq!(a, ANS);
    })
}

#[bench]
fn bench_jit(b: &mut Bencher) {
    use inkwell::{context::Context, OptimizationLevel};

    let context = Context::create();
    let module = context.create_module("iter");
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let codegen = CodeGen {
        context: &context,
        module,
        builder: context.create_builder(),
        execution_engine,
    };

    let iter = codegen.jit_compile_iter();

    b.iter(|| {
        unsafe {
            assert_eq!(ANS, iter.call());
            // assert_eq!(sum.call(x, y, z), x + y + z);
        }
    })
}

#[bench]
fn bench_vm(b: &mut Bencher) {
    b.iter(|| {
        let mut vm = VM::new();
        let bytecode = vec![
            1, // Int
            1, 0, 0, 0, 0, 0, 0, 0, // 1
            2, // Set
            0, // 0
            1, // Int
            1, 0, 0, 0, 0, 0, 0, 0, // 1
            2, // Set
            1, // 1
            3, // Get @22
            1, // 1
            1, // Int
            0x40, 0x42, 0x0f, 0, 0, 0, 0, 0,  // 1000000
            8,  // LT
            5,  // JumpIfFalse @34
            60, // to 60
            3,  // Get
            0,  // 0
            3,  // Get
            1,  // 1
            6,  // Mul
            1,  // Int
            0x07, 0xe1, 0xf5, 0x05, 0, 0, 0, 0,  // 10000007
            7,  // Mod
            2,  // Set
            0,  // 0
            3,  // Get
            1,  // 1
            9,  // Increment
            2,  // Set
            1,  // 1
            4,  // Jump
            22, // to 22
            0,  // EOF @60
        ];
        vm.eval(&bytecode);
    })
}
