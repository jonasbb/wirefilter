use crate::{CompiledFunction, ExpectedType, FunctionDefinition, FunctionParam, Type};
use std::iter;

/// Hashes one or more integer, byte-string, or IP fields into the requested range.
#[derive(Debug, Default)]
pub struct HashInRangeFunction {}

impl FunctionDefinition for HashInRangeFunction {
    fn check_param(
        &self,
        _: &crate::ParserSettings,
        params: &mut dyn ExactSizeIterator<Item = FunctionParam<'_>>,
        next_param: &FunctionParam<'_>,
        _: Option<&mut super::FunctionDefinitionContext>,
    ) -> Result<(), super::FunctionParamError> {
        match params.len() {
            0 => {
                next_param
                    .arg_kind()
                    .expect(super::FunctionArgKind::Literal)?;
                next_param.expect_val_type(iter::once(Type::Int.into()))?;
            }
            1 => {
                next_param
                    .arg_kind()
                    .expect(super::FunctionArgKind::Literal)?;
                next_param.expect_val_type(iter::once(Type::Int.into()))?;
                let min: &i64 = params.next().unwrap().as_constant()?.try_into()?;
                next_param.expect_const_value::<&i64, _>(|max| {
                    if *max > *min {
                        Ok(())
                    } else {
                        Err("max must be greater than min".to_string())
                    }
                })?;
            }
            _ => {
                next_param
                    .arg_kind()
                    .expect(super::FunctionArgKind::Field)?;
                next_param.expect_val_type(
                    [
                        ExpectedType::Type(Type::Int),
                        ExpectedType::Type(Type::Bytes),
                        ExpectedType::Type(Type::Ip),
                    ]
                    .into_iter(),
                )?;
            }
        }
        Ok(())
    }

    fn return_type(
        &self,
        _: &mut dyn ExactSizeIterator<Item = FunctionParam<'_>>,
        _: Option<&super::FunctionDefinitionContext>,
    ) -> Type {
        Type::Int
    }

    fn arg_count(&self) -> (usize, Option<usize>) {
        (3, None)
    }

    fn compile(
        &self,
        _: &mut dyn ExactSizeIterator<Item = FunctionParam<'_>>,
        _: Option<super::FunctionDefinitionContext>,
    ) -> CompiledFunction {
        unimplemented!()
    }
}
