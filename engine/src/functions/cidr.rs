use crate::{FunctionArgs, FunctionDefinition, LhsValue, Type};
use std::iter;

/// `cidr` Function (Cloudflare Ruleset Engine)
///
/// This documentation describes the behavior and usage of the `cidr` function
/// within Cloudflare's Ruleset Engine. It is not a native Rust function,
/// but rather a built-in function available for use in Cloudflare rule expressions.
///
/// The `cidr` function returns the network address corresponding to a given IP address
/// (IPv4 or IPv6), based on the specified network bit length (prefix length).
/// It is instrumental in creating rules that match traffic based on network segments
/// rather than individual IP addresses.
///
/// # Syntax in Ruleset Engine Expressions
///
/// `cidr(address, ipv4_network_bits, ipv6_network_bits)`
///
/// # Arguments
///
/// * `address`: An IP address (IPv4 or IPv6) that needs to be truncated to its network address.
///   - **Type:** IP address field (e.g., `ip.src`, `ip.dst`).
///   - **Constraint:** This parameter **must** be a field reference and **cannot** be a literal string.
///     The engine dynamically evaluates the IP address from the request's context.
///
/// * `ipv4_network_bits`: An integer specifying the number of leading bits that represent the network
///   portion for an **IPv4** address. This value defines the equivalent of an IPv4 subnet mask.
///   - **Type:** `Integer`
///   - **Constraint:** Must be between `1` and `32`.
///
/// * `ipv6_network_bits`: An integer specifying the number of leading bits that represent the network
///   portion for an **IPv6** address. This value defines the equivalent of an IPv6 prefix length.
///   - **Type:** `Integer`
///   - **Constraint:** Must be between `1` and `128`.
///
/// # Returns
///
/// * **Type:** IP address (IPv4 or IPv6)
/// * **Description:** The calculated network address (network ID) corresponding to the input `address`
///   and the relevant network bit length. The host portion of the IP address is "zeroed out".
///
/// # How it Works
///
/// The `cidr` function intelligently processes the `address` parameter based on its type:
/// - If `address` resolves to an IPv4 address, the `ipv4_network_bits` parameter is used
///   to determine the network portion, and `ipv6_network_bits` is ignored.
/// - If `address` resolves to an IPv6 address, the `ipv6_network_bits` parameter is used
///   to determine the network portion, and `ipv4_network_bits` is ignored.
///
/// # Examples for Cloudflare Ruleset Engine Expressions
///
/// Below are examples of how `cidr` is used within actual Cloudflare Ruleset Engine expressions.
/// These are typically part of a larger rule definition.
///
/// **1. Matching IPv4 traffic from the `113.10.0.0/24` network:**
///
/// ```text
/// (cidr(ip.src, 24, 64) eq 113.10.0.0)
/// ```
/// *Explanation:* This expression checks if the source IP address (`ip.src`), when its network
/// portion is truncated to 24 bits (for IPv4), matches `113.10.0.0`. The `64` for
/// `ipv6_network_bits` is a placeholder and would be ignored if `ip.src` is IPv4.
///
/// **2. Matching IPv6 traffic from the `2001:0:0:0::/24` network:**
///
/// ```text
/// (cidr(ip.src, 32, 24) eq 2001:0000:0000:0000:0000:0000:0000:0000)
/// ```
/// *Explanation:* This expression checks if the source IP address (`ip.src`), when its network
/// portion is truncated to 24 bits (for IPv6), matches `2001:0000:0000:0000:0000:0000:0000:0000`.
/// The `32` for `ipv4_network_bits` is a placeholder and would be ignored if `ip.src` is IPv6.
///
/// **3. Blocking all traffic originating from a specific IPv4 subnet:**
///
/// ```text
/// (ip.src in { "192.168.1.0/24" }) or (cidr(ip.src, 24, 0) eq 10.0.0.0)
/// ```
/// *Explanation:* This example shows how to combine `in` operator with `cidr`. It would block
/// traffic from the `192.168.1.0/24` subnet directly or if the source IP address, when truncated
/// to a `/24`, matches `10.0.0.0`. Note that for `cidr` on IPv4, `ipv6_network_bits` can be
/// set to `0` as it will be ignored by the engine.
#[derive(Debug, Default)]
pub struct CIDRFunction {}

impl FunctionDefinition for CIDRFunction {
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
                next_param.expect_val_type(iter::once(Type::Ip.into()))?;
            }
            1 => {
                next_param
                    .arg_kind()
                    .expect(super::FunctionArgKind::Literal)?;
                next_param.expect_val_type(iter::once(Type::Int.into()))?;
            }
            2 => {
                next_param
                    .arg_kind()
                    .expect(super::FunctionArgKind::Literal)?;
                next_param.expect_val_type(iter::once(Type::Int.into()))?;
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
        (3, Some(0))
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
