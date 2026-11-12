use crate::{FunctionArgs, FunctionDefinition, LhsValue, Type};
use std::iter;

/// Removes ASCII whitespace from both ends of a string field.
///
/// An optional literal string specifies the bytes to remove instead of ASCII
/// whitespace.
#[derive(Debug, Default)]
pub struct TrimFunction {}

impl FunctionDefinition for TrimFunction {
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
                super::expect_unique_bytes(next_param)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_duplicate_trim_bytes() {
        let trim_set = super::super::LiteralValue::Bytes(crate::BytesExpr::new(
            "aa".as_bytes(),
            crate::BytesFormat::Quoted,
        ));
        let param = super::super::FunctionParam::Constant(&trim_set);
        let source = super::super::FunctionParam::Variable(Type::Bytes);
        let mut params = [source].into_iter();

        assert!(
            TrimFunction {}
                .check_param(&crate::ParserSettings::default(), &mut params, &param, None,)
                .is_err()
        );
    }

    #[test]
    fn accepts_unique_trim_bytes() {
        let trim_set = super::super::LiteralValue::Bytes(crate::BytesExpr::new(
            "ab".as_bytes(),
            crate::BytesFormat::Quoted,
        ));
        let param = super::super::FunctionParam::Constant(&trim_set);
        let source = super::super::FunctionParam::Variable(Type::Bytes);
        let mut params = [source].into_iter();

        TrimFunction {}
            .check_param(&crate::ParserSettings::default(), &mut params, &param, None)
            .unwrap();
    }
}
