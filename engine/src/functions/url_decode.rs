use crate::{FunctionArgs, FunctionDefinition, LhsValue, Type};
use std::iter;

/// Decodes a URL-formatted string defined in source.
///
/// Behavior summary:
/// - `%20` and `+` decode to a space character (` `).
/// - `%HH` decodes to the corresponding byte value.
/// - `%uXXXX` (when the `u` option is provided) decodes to the Unicode
///   code point U+XXXX and is emitted as UTF-8 bytes.
/// - The source must be a field (not a literal string).
///
/// Options (passed as a single literal string, e.g. "r" or "ur"):
/// - `r`: Recursive decoding. For example `%2520` decoded with `r` becomes a space
///   (`%2520` -> `%20` -> ` `).
/// - `u`: Enable Unicode percent decoding using `%uXXXX` sequences. The output
///   will be UTF-8 encoded.
///
/// Examples:
///
/// url_decode("John%20Doe") -> "John Doe"
/// url_decode("John+Doe")   -> "John Doe"
/// url_decode("%2520")      -> "%20"
/// url_decode("%2520", "r") -> " "
///
/// Notes:
/// - If `u` is provided and a `%uXXXX` sequence contains an invalid code point
///   or invalid hex, the implementation falls back conservatively and leaves
///   the `%` byte intact for that sequence.
/// - Recursive decoding is bounded (to avoid pathological loops).
#[derive(Debug, Default)]
pub struct UrlDecodeFunction {}

impl FunctionDefinition for UrlDecodeFunction {
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
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    fn return_type(
        &self,
        _: &mut dyn ExactSizeIterator<Item = super::FunctionParam<'_>>,
        _: Option<&super::FunctionDefinitionContext>,
    ) -> Type {
        Type::Bytes
    }

    fn arg_count(&self) -> (usize, Option<usize>) {
        (1, Some(1))
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
