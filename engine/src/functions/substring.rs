use super::{FunctionArgKind, FunctionArgs, FunctionDefinition};
use crate::{LhsValue, Type};

/// Returns a substring (slice by byte index) of a String/Bytes field.
///
/// Usage:
///
/// substring(field, start, end?)
///
/// - `field` must be a non-literal field whose value is `String`/`Bytes` (for
///   example `http.request.body.raw`).
/// - `start` is an `Integer` byte index indicating the first byte to include.
/// - `end` is an optional `Integer` byte index indicating the first byte to
///   exclude. If omitted, the substring runs to the end of the field.
///
/// Index semantics:
/// - Indexing is by byte, not Unicode scalar; the first byte is index 0.
/// - Negative indexes count from the end of the value: an index of `-1` refers
///   to the last byte, `-2` to the penultimate byte, and so on.
/// - Out-of-range indexes are clamped to the bounds `[0, len]` where `len` is
///   the byte length of the field. If `end < start` after clamping, an empty
///   string is returned.
///
/// Examples:
///
/// If `http.request.body.raw` is `"asdfghjk"`:
///
/// substring(http.request.body.raw, 2, 5)   -> "dfg"
/// substring(http.request.body.raw, 2)      -> "dfghjk"
/// substring(http.request.body.raw, -2)     -> "jk"
/// substring(http.request.body.raw, 0, -2)  -> "asdfgh"
#[derive(Debug, Default)]
pub struct SubstringFunction {}

impl FunctionDefinition for SubstringFunction {
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
            1 => {
                next_param.arg_kind().expect(FunctionArgKind::Literal)?;
                next_param.expect_val_type(std::iter::once(Type::Int.into()))?;
            }
            2 => {
                next_param.arg_kind().expect(FunctionArgKind::Literal)?;
                next_param.expect_val_type(std::iter::once(Type::Int.into()))?;
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
        (2, Some(1))
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
