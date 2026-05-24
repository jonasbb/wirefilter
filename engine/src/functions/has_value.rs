use crate::{
    ExpectedType, ExpectedTypeList, FunctionArgs, FunctionDefinition, GetType, LhsValue, Type,
    TypeMismatchError,
};
use std::iter;

/// `has_value(collection, value)` — return true if `value` exists in the
/// supplied `collection` (Map or Array).
#[derive(Debug, Default)]
pub struct HasValueFunction {}

impl HasValueFunction {
    /// Create a new `has_value` function definition.
    pub const fn new() -> Self {
        Self {}
    }
}

impl FunctionDefinition for HasValueFunction {
    fn check_param(
        &self,
        _: &crate::ParserSettings,
        params: &mut dyn ExactSizeIterator<Item = super::FunctionParam<'_>>,
        next_param: &super::FunctionParam<'_>,
        _: Option<&mut super::FunctionDefinitionContext>,
    ) -> Result<(), super::FunctionParamError> {
        match params.len() {
            0 => {
                // first argument must be a Map<T> or Array<T> field
                next_param
                    .arg_kind()
                    .expect(super::FunctionArgKind::Field)?;
                next_param.expect_val_type(
                    iter::once(ExpectedType::Map).chain(iter::once(ExpectedType::Array)),
                )?;
            }
            1 => {
                let collection_type = params.next().unwrap().get_type();
                let elem_ty = match collection_type {
                    Type::Map(value_type) | Type::Array(value_type) => value_type.into_type(),
                    _ => unreachable!(),
                };

                // Element type `T` must be a primitive type (Bytes, Int, Ip, Bool)
                match elem_ty {
                    Type::Bytes | Type::Int | Type::Ip | Type::Bool => {
                        next_param.expect_val_type(iter::once(elem_ty.into()))?;
                    }
                    other => {
                        // Construct an expected list of primitive types for a clear error
                        let mut expected = ExpectedTypeList::default();
                        expected.insert(ExpectedType::Type(Type::Bytes));
                        expected.insert(ExpectedType::Type(Type::Int));
                        expected.insert(ExpectedType::Type(Type::Ip));
                        expected.insert(ExpectedType::Type(Type::Bool));
                        return Err(super::FunctionParamError::TypeMismatch(TypeMismatchError {
                            expected,
                            actual: other,
                        }));
                    }
                }
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
        (2, Some(0))
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
