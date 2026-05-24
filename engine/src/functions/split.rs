use crate::{FunctionArgs, FunctionDefinition, LhsValue, Type};
use std::iter;

/// `split(input, separator, limit)` — split `input` string by `separator` into
/// an array of strings with at most `limit` elements.
#[derive(Debug, Default)]
pub struct SplitFunction {}

impl SplitFunction {
    /// Create a new `split` function definition.
    pub const fn new() -> Self {
        Self {}
    }
}

impl FunctionDefinition for SplitFunction {
    fn check_param(
        &self,
        _: &crate::ParserSettings,
        params: &mut dyn ExactSizeIterator<Item = super::FunctionParam<'_>>,
        next_param: &super::FunctionParam<'_>,
        _: Option<&mut super::FunctionDefinitionContext>,
    ) -> Result<(), super::FunctionParamError> {
        match params.len() {
            0 => {
                next_param
                    .arg_kind()
                    .expect(super::FunctionArgKind::Field)?;
                next_param.expect_val_type(iter::once(Type::Bytes.into()))?;
            }
            1 => {
                next_param
                    .arg_kind()
                    .expect(super::FunctionArgKind::Literal)?;
                next_param.expect_val_type(iter::once(Type::Bytes.into()))?;
                next_param.expect_const_value::<&crate::BytesExpr, _>(|flags| {
                    if flags.is_empty() {
                        return Err("separator cannot be empty".to_string());
                    }
                    Ok(())
                })?;
            }
            2 => {
                next_param
                    .arg_kind()
                    .expect(super::FunctionArgKind::Literal)?;
                next_param.expect_val_type(iter::once(Type::Int.into()))?;
                next_param.expect_const_value::<&i64, _>(|v| {
                    if *v >= 1 && *v <= 128 {
                        Ok(())
                    } else {
                        Err("limit must be between 1 and 128".to_string())
                    }
                })?;
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    fn return_type(
        &self,
        _params: &mut dyn ExactSizeIterator<Item = super::FunctionParam<'_>>,
        _: Option<&super::FunctionDefinitionContext>,
    ) -> Type {
        // element type is Bytes
        Type::Array(Type::Bytes.into())
    }

    fn arg_count(&self) -> (usize, Option<usize>) {
        (3, Some(0))
    }

    fn compile(
        &self,
        _: &mut dyn ExactSizeIterator<Item = super::FunctionParam<'_>>,
        _: Option<super::FunctionDefinitionContext>,
    ) -> Box<dyn for<'i, 'a> Fn(FunctionArgs<'i, 'a>) -> Option<LhsValue<'a>> + Sync + Send + 'static>
    {
        unimplemented!()
    }
}
