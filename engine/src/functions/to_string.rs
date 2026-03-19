use super::{FunctionArgKind, FunctionArgs, FunctionDefinition};
use crate::{LhsValue, Type};

/// Convert an Integer, Boolean, or IP LHS value into its textual representation.
///
/// Usage:
///
/// to_string(field)
///
/// - `field` must be a non-literal field whose value is `Integer`, `Boolean`,
///   or `IP`. If the field is missing (type mismatch at runtime), the
///   function evaluates to `None` (propagates the missing field).
/// - The function returns the UTF-8 bytes of the textual representation of
///   the value (for example `5` -> "5", `true` -> "true", `1.2.3.4` ->
///   "1.2.3.4").
///
/// Examples:
///
/// Given a field `http.request.status_code` with integer value `200`:
///
/// ```text
/// any(to_string(http.request.status_code)[*] eq "200")
/// ```
///
/// If the field is missing or has the wrong type at evaluation time the
/// function returns `None` and the surrounding expression will behave
/// accordingly.
#[derive(Debug, Default)]
pub struct ToStringFunction {}

impl FunctionDefinition for ToStringFunction {
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
                next_param.expect_val_type(
                    [Type::Int.into(), Type::Bool.into(), Type::Ip.into()].into_iter(),
                )?;
            }
            _ => unreachable!(),
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
        (1, Some(0))
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
