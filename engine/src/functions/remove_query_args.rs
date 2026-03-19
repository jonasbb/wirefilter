use super::{FunctionArgKind, FunctionArgs, FunctionDefinition};
use crate::{LhsValue, Type};

/// Removes one or more query string parameters from a URI query string.
///
/// The first argument must be a field (for example `http.request.uri.query`),
/// and the remaining arguments must be literal byte strings naming the
/// parameters to remove. The function removes all occurrences of the named
/// parameters and preserves the order of unaffected parameters. If the result
/// is empty, an empty string is returned.
#[derive(Debug, Default)]
pub struct RemoveQueryArgsFunction {}

impl FunctionDefinition for RemoveQueryArgsFunction {
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
                next_param.expect_val_type(std::iter::once(Type::Bytes.into()))?;
            }
            _ => {
                next_param.arg_kind().expect(FunctionArgKind::Literal)?;
                next_param.expect_val_type(std::iter::once(Type::Bytes.into()))?;
            }
        }

        Ok(())
    }

    fn return_type(
        &self,
        _: &mut dyn ExactSizeIterator<Item = super::FunctionParam<'_>>,
        _: Option<&super::FunctionDefinitionContext>,
    ) -> crate::Type {
        Type::Bytes
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
