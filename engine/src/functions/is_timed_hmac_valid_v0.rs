use crate::{FunctionArgs, FunctionDefinition, LhsValue, Type};
use std::iter;

/// `is_timed_hmac_valid_v0(key, messageMAC, ttl, currentTimeStamp, lengthOfSeparator?, flags?)`
/// — validate a time-limited HMAC token using the provided parameters.
#[derive(Debug, Default)]
pub struct IsTimedHmacValidV0Function {}

impl IsTimedHmacValidV0Function {
    /// Create a new `is_timed_hmac_valid_v0` function definition.
    pub const fn new() -> Self {
        Self {}
    }
}

impl FunctionDefinition for IsTimedHmacValidV0Function {
    fn check_param(
        &self,
        _: &crate::ParserSettings,
        params: &mut dyn ExactSizeIterator<Item = super::FunctionParam<'_>>,
        next_param: &super::FunctionParam<'_>,
        _: Option<&mut super::FunctionDefinitionContext>,
    ) -> Result<(), super::FunctionParamError> {
        match params.len() {
            0 => {
                // Key: literal string
                next_param
                    .arg_kind()
                    .expect(super::FunctionArgKind::Literal)?;
                next_param.expect_val_type(iter::once(Type::Bytes.into()))?;
            }
            1 => {
                // MessageMAC: field string
                next_param
                    .arg_kind()
                    .expect(super::FunctionArgKind::Field)?;
                next_param.expect_val_type(iter::once(Type::Bytes.into()))?;
            }
            2 => {
                // ttl: literal integer
                next_param
                    .arg_kind()
                    .expect(super::FunctionArgKind::Literal)?;
                next_param.expect_val_type(iter::once(Type::Int.into()))?;
            }
            3 => {
                // currentTimeStamp: integer
                // Pass the http.request.timestamp.sec field as an approximate value to this argument.
                next_param
                    .arg_kind()
                    .expect(super::FunctionArgKind::Field)?;
                next_param.expect_val_type(iter::once(Type::Int.into()))?;
            }
            4 => {
                // optional lengthOfSeparator: literal integer
                next_param
                    .arg_kind()
                    .expect(super::FunctionArgKind::Literal)?;
                next_param.expect_val_type(iter::once(Type::Int.into()))?;
            }
            5 => {
                // optional flags: literal string
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
        Type::Bool
    }

    fn arg_count(&self) -> (usize, Option<usize>) {
        (4, Some(2))
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
