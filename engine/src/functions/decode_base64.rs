use crate::{FunctionArgs, FunctionDefinition, LhsValue, Type};

/// Decodes a Base64-encoded string specified in `source`.
///
/// The `source` must be a field (not a literal). The function decodes using
/// the standard Base64 alphabet (RFC 4648) and returns the decoded bytes.
///
/// Example:
///
/// Given an HTTP header: `client_id: MTIzYWJj`
///
/// ```text
/// any(decode_base64(http.request.headers["client_id"][*])[*] eq "123abc")
/// ```
///
/// The above evaluates to true because `MTIzYWJj` decodes to `"123abc"`.
#[derive(Default, Debug)]
pub struct DecodeBase64Function {}

impl FunctionDefinition for DecodeBase64Function {
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
                next_param.expect_val_type(std::iter::once(Type::Bytes.into()))?;
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
