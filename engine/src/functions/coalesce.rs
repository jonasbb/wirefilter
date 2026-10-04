use crate::{CompiledFunction, FunctionDefinition, FunctionParam, GetType, Type};
use std::iter;

/// Returns the first non-missing value from two or more arguments of the same type.
#[derive(Debug, Default)]
pub struct CoalesceFunction {}

impl FunctionDefinition for CoalesceFunction {
    fn check_param(
        &self,
        _: &crate::ParserSettings,
        params: &mut dyn ExactSizeIterator<Item = FunctionParam<'_>>,
        next_param: &FunctionParam<'_>,
        _: Option<&mut super::FunctionDefinitionContext>,
    ) -> Result<(), super::FunctionParamError> {
        if let Some(previous_param) = params.next() {
            next_param.expect_val_type(iter::once(previous_param.get_type().into()))?;
        }
        Ok(())
    }

    fn return_type(
        &self,
        params: &mut dyn ExactSizeIterator<Item = FunctionParam<'_>>,
        _: Option<&super::FunctionDefinitionContext>,
    ) -> Type {
        params.next().unwrap().get_type()
    }

    fn arg_count(&self) -> (usize, Option<usize>) {
        (2, None)
    }

    fn compile(
        &self,
        _: &mut dyn ExactSizeIterator<Item = FunctionParam<'_>>,
        _: Option<super::FunctionDefinitionContext>,
    ) -> CompiledFunction {
        unimplemented!()
    }
}
