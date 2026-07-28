use super::{FunctionArgKind, FunctionArgs, FunctionDefinition};
use crate::{LhsValue, Type};
use std::iter;

/// Returns the integer value associated with the supplied key in `field`.
///
/// The `field` must be a string containing a valid JSON document. Subsequent
/// arguments are literal keys that can be attribute names (strings) or
/// zero-based array positions (integers). Keys are applied in order to traverse
/// the JSON hierarchy. Only plain integers are returned (floats like `42.0`
/// are rejected).
#[derive(Debug, Default)]
pub struct JsonLookupIntegerFunction {}

impl FunctionDefinition for JsonLookupIntegerFunction {
    fn check_param(
        &self,
        _: &crate::ParserSettings,
        params: &mut dyn ExactSizeIterator<Item = super::FunctionParam<'_>>,
        next_param: &super::FunctionParam<'_>,
        _: Option<&mut super::FunctionDefinitionContext>,
    ) -> Result<(), super::FunctionParamError> {
        match params.len() {
            0 => {
                next_param.arg_kind().expect(FunctionArgKind::Field)?;
                next_param.expect_val_type(iter::once(Type::Bytes.into()))?;
            }
            _ => {
                next_param.arg_kind().expect(FunctionArgKind::Literal)?;
                next_param
                    .expect_val_type(vec![Type::Bytes.into(), Type::Int.into()].into_iter())?;
            }
        }

        Ok(())
    }

    fn return_type(
        &self,
        _: &mut dyn ExactSizeIterator<Item = super::FunctionParam<'_>>,
        _: Option<&super::FunctionDefinitionContext>,
    ) -> crate::Type {
        Type::Int
    }

    fn arg_count(&self) -> (usize, Option<usize>) {
        (2, None)
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
