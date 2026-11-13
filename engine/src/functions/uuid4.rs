use super::{FunctionArgs, FunctionDefinition};
use crate::{LhsValue, Type};
use std::iter;

/// Defines Cloudflare's `uuidv4` function.
/// To obtain an array of random bytes, use the cf.random_seed field.
/// For example, uuidv4(cf.random_seed) will return a UUIDv4 similar to 49887398-6bcf-485f-8899-f15dbef4d1d5.
#[derive(Debug, Default)]
pub struct UUID4Function {}

impl FunctionDefinition for UUID4Function {
    fn check_param(
        &self,
        _: &crate::ParserSettings,
        params: &mut dyn ExactSizeIterator<Item = super::FunctionParam<'_>>,
        next_param: &super::FunctionParam<'_>,
        _: Option<&mut super::FunctionDefinitionContext>,
    ) -> Result<(), super::FunctionParamError> {
        match params.len() {
            // `source` must be a FIELD byte string
            // Constraints are not mentioned in the online documentation, but experimentally verified.
            0 => {
                next_param
                    .arg_kind()
                    .expect(super::FunctionArgKind::Field)?;
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
