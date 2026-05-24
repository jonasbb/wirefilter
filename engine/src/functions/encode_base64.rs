use crate::{FunctionArgs, FunctionDefinition, LhsValue, Type};
use std::iter;

/// Encodes an input String/Bytes to Base64. Only parsing metadata is
/// implemented here (checks for argument kinds/types). Runtime `compile`
/// is left unimplemented intentionally.
/// `encode_base64(input[, flags])` — encode input bytes/string to Base64.
#[derive(Debug, Default)]
pub struct EncodeBase64Function {}

impl EncodeBase64Function {
    /// Create a new `encode_base64` function definition.
    pub const fn new() -> Self {
        Self {}
    }
}

impl FunctionDefinition for EncodeBase64Function {
    fn check_param(
        &self,
        _: &crate::ParserSettings,
        params: &mut dyn ExactSizeIterator<Item = super::FunctionParam<'_>>,
        next_param: &super::FunctionParam<'_>,
        _: Option<&mut super::FunctionDefinitionContext>,
    ) -> Result<(), super::FunctionParamError> {
        match params.len() {
            0 => {
                // input: String | Bytes (accept either literal or field)
                next_param.expect_val_type(iter::once(Type::Bytes.into()))?;
            }
            1 => {
                // flags: optional literal string
                next_param
                    .arg_kind()
                    .expect(super::FunctionArgKind::Literal)?;
                next_param.expect_val_type(iter::once(Type::Bytes.into()))?;
                next_param.expect_const_value::<&crate::BytesExpr, _>(|flags| {
                    for &c in flags {
                        // Acceptable flags are
                        // * 'u' (URL-safe)
                        // * 'p' (pad with '=')
                        if c == b'u' || c == b'p' {
                        } else {
                            return Err(format!(
                                "Invalid flag `{}`. Expected 'u' or 'p'.",
                                c as char
                            ));
                        }
                    }
                    Ok(())
                })?;
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
