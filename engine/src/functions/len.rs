use crate::{ExpectedType, FunctionArgs, FunctionDefinition, LhsValue, Type};

/// Returns the byte length of a String or Bytes value, or the number of elements in an array.
///
/// This function is part of the Cloudflare Ruleset Engine.
///
/// # Arguments
///
/// * `value` - A `String`, `Bytes`, or `Array` type.
///
/// # Return Value
///
/// An `Integer` representing the length.
///
///
/// # Panics
///
/// This function will panic if:
/// - No arguments are provided.
/// - More than one argument is provided.
/// - The provided argument is not of type `String`, `Bytes`, or `Array`.
///
/// # Internal Implementation Details
///
/// The `LenFunction` struct implements the `FunctionDefinition` trait,
/// providing the necessary checks for parameters, return type, and
/// compilation to the underlying `len_impl` function.
///
/// The `len_impl` function handles the core logic of calculating the length
/// based on the `LhsValue` type:
/// - For `LhsValue::Array`, it returns the number of elements.
/// - For `LhsValue::Bytes` (which includes String values), it returns the byte length.
/// - It returns `None` if the expected types (`Array` or `Bytes`) are not found,
///   simulating a missing field.
#[derive(Debug, Default)]
pub struct LenFunction {}

impl FunctionDefinition for LenFunction {
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
                next_param.expect_val_type(
                    [ExpectedType::Type(Type::Bytes), ExpectedType::Array]
                        .iter()
                        .cloned(),
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
    ) -> Type {
        Type::Int
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
