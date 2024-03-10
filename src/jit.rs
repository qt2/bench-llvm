use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::{IntPredicate, OptimizationLevel};

use std::error::Error;
/// Convenience type alias for the `sum` function.
///
/// Calling this is innately `unsafe` because there's no guarantee it doesn't
/// do `unsafe` operations internally.
type SumFunc = unsafe extern "C" fn(u64, u64, u64) -> u64;
type IterFunc = unsafe extern "C" fn() -> i64;

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    fn jit_compile_sum(&self) -> Option<JitFunction<SumFunc>> {
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);
        let function = self.module.add_function("sum", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        let x = function.get_nth_param(0)?.into_int_value();
        let y = function.get_nth_param(1)?.into_int_value();
        let z = function.get_nth_param(2)?.into_int_value();

        let sum = self.builder.build_int_add(x, y, "sum").unwrap();
        let sum = self.builder.build_int_add(sum, z, "sum").unwrap();

        self.builder.build_return(Some(&sum)).unwrap();

        unsafe { self.execution_engine.get_function("sum").ok() }
    }

    pub fn jit_compile_iter(&self) -> JitFunction<IterFunc> {
        // let void_type = self.context.void_type();
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[], false);
        let function = self.module.add_function("iter", fn_type, None);
        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);

        let a_ptr = self.builder.build_alloca(i64_type, "a").unwrap();
        self.builder
            .build_store(a_ptr, self.context.i64_type().const_int(1, false))
            .unwrap();
        let i_ptr = self.builder.build_alloca(i64_type, "i").unwrap();
        self.builder
            .build_store(i_ptr, self.context.i64_type().const_int(1, false))
            .unwrap();

        let loop_block = self.context.append_basic_block(function, "loop");
        self.builder.build_unconditional_branch(loop_block).unwrap();
        self.builder.position_at_end(loop_block);

        let cond = self.builder.build_load(i64_type, i_ptr, "i").unwrap();
        let cond = self
            .builder
            .build_int_compare(
                IntPredicate::SLT,
                cond.into_int_value(),
                self.context.i64_type().const_int(1000000, false),
                "loopcond",
            )
            .unwrap();

        let body_block = self.context.append_basic_block(function, "loopbody");
        let after_block = self.context.append_basic_block(function, "afterloop");

        self.builder
            .build_conditional_branch(cond, body_block, after_block)
            .unwrap();
        self.builder.position_at_end(body_block);

        let va = self.builder.build_load(i64_type, a_ptr, "a").unwrap();
        let vi = self.builder.build_load(i64_type, i_ptr, "i").unwrap();
        let v0 = self
            .builder
            .build_int_mul(va.into_int_value(), vi.into_int_value(), "0")
            .unwrap();
        let v1 = self
            .builder
            .build_int_signed_rem(v0, self.context.i64_type().const_int(100000007, false), "1")
            .unwrap();
        self.builder.build_store(a_ptr, v1).unwrap();

        let next = self
            .builder
            .build_int_add(
                vi.into_int_value(),
                self.context.i64_type().const_int(1, false),
                "next",
            )
            .unwrap();
        self.builder.build_store(i_ptr, next).unwrap();

        self.builder.build_unconditional_branch(loop_block).unwrap();

        self.builder.position_at_end(after_block);

        let va = self.builder.build_load(i64_type, a_ptr, "a").unwrap();
        self.builder.build_return(Some(&va)).unwrap();

        unsafe {
            self.execution_engine
                .get_function("iter")
                .expect("failed to compile iter")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() -> Result<(), Box<dyn Error>> {
        let context = Context::create();
        let module = context.create_module("sum");
        let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;
        let codegen = CodeGen {
            context: &context,
            module,
            builder: context.create_builder(),
            execution_engine,
        };

        let sum = codegen
            .jit_compile_sum()
            .ok_or("Unable to JIT compile `sum`")?;

        let x = 1u64;
        let y = 2u64;
        let z = 3u64;

        unsafe {
            println!("{} + {} + {} = {}", x, y, z, sum.call(x, y, z));
            assert_eq!(sum.call(x, y, z), x + y + z);
        }

        Ok(())
    }

    #[test]
    fn test_iter() -> Result<(), Box<dyn Error>> {
        let context = Context::create();
        let module = context.create_module("iter");
        let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;
        let codegen = CodeGen {
            context: &context,
            module,
            builder: context.create_builder(),
            execution_engine,
        };

        let iter = codegen.jit_compile_iter();

        unsafe {
            println!("{}", iter.call());
            // assert_eq!(sum.call(x, y, z), x + y + z);
        }

        Ok(())
    }
}
