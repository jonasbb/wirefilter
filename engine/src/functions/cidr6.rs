use crate::{FunctionArgs, FunctionDefinition, LhsValue, Type};
use std::iter;

/// `cidr6(address, ipv6_network_bits)` — compute the IPv6 network address
/// for the provided `address` using `ipv6_network_bits` as the netmask length.
#[derive(Debug, Default)]
pub struct CIDR6Function {}

impl CIDR6Function {
    /// Create a new `cidr6` function definition.
    pub const fn new() -> Self {
        Self {}
    }
}

impl FunctionDefinition for CIDR6Function {
    fn check_param(
        &self,
        _: &crate::ParserSettings,
        params: &mut dyn ExactSizeIterator<Item = super::FunctionParam<'_>>,
        next_param: &super::FunctionParam<'_>,
        _: Option<&mut super::FunctionDefinitionContext>,
    ) -> Result<(), super::FunctionParamError> {
        match params.len() {
            0 => {
                // address must be a field of type Ip
                next_param
                    .arg_kind()
                    .expect(super::FunctionArgKind::Field)?;
                next_param.expect_val_type(iter::once(Type::Ip.into()))?;
            }
            1 => {
                // ipv6_network_bits: literal integer
                next_param
                    .arg_kind()
                    .expect(super::FunctionArgKind::Literal)?;
                next_param.expect_val_type(iter::once(Type::Int.into()))?;
                // optional: validate numeric range if constant provided
                next_param.expect_const_value::<&i64, _>(|ipv6_network_bits| {
                    if *ipv6_network_bits >= 1 && *ipv6_network_bits <= 128 {
                        Ok(())
                    } else {
                        Err("ipv6 network bits must be between 1 and 128".to_string())
                    }
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
        Type::Ip
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
