use crate::{FunctionArgs, FunctionDefinition, LhsValue, Type};
use std::iter;

/// Mimics Cloudflare's `wildcard_replace` function for byte slice inputs and output.
///
/// This function replaces a `source` byte slice, matched by a `wildcard_pattern`
/// (a byte slice containing `*` wildcard metacharacters), with a `replacement`
/// byte slice. The `replacement` can contain references to wildcard capture groups
/// (e.g., `$1`, `$2`), up to eight such references.
///
/// **Important Note on UTF-8 Validity:**
/// The `wildcard_pattern` and `replacement` byte slices are interpreted as UTF-8
/// strings internally to process the wildcard (`*`) and capture group (`$1`) syntax.
/// Therefore, these two parameters **must be valid UTF-8**. If they are not,
/// the function will panic. The `source` parameter does not need to be valid UTF-8.
/// The output `Vec<u8>` will be valid UTF-8 if the `source` was valid UTF-8 and
/// the replacement logic preserves that validity; otherwise, it will contain raw bytes.
///
/// # Arguments
///
/// * `source` - The input byte slice (`&[u8]`) on which to perform the replacement.
///   In Cloudflare's engine, this must be a field value (e.g., `http.request.full_uri`).
///   The entire `source` value must match the `wildcard_pattern` (partial matches are ignored).
///
/// * `wildcard_pattern` - A byte slice (`&[u8]`) defining the pattern to match in `source`.
///   It can contain the following metacharacters:
///     * `*`: Matches zero or more of any character. This is a "lazy" match,
///       meaning it tries to match the shortest possible string. Each `*`
///       creates a capture group that can be referenced in `replacement`.
///     * `\*`: To match a literal asterisk (`*`), it must be escaped with a backslash.
///     * `\\`: To match a literal backslash (`\`), it must be escaped with another backslash.
///
///   **Invalid Patterns:** Two unescaped `*` characters in a row (`**`) are considered
///   invalid and cannot be used, aligning with Cloudflare's rules.
///
/// * `replacement` - A byte slice (`&[u8]`) that will replace the matched pattern.
///   It can contain references to wildcard capture groups in the format `$N`
///   (e.g., `$1`, `$2`), where `N` corresponds to the N-th `*` in the
///   `wildcard_pattern`. Up to eight replacement references (`$1` through `$8`)
///   are supported.
///     * `$$`: To include a literal dollar sign (`$`), it must be escaped with
///       another dollar sign.
///
/// * `flags` - An optional byte slice (`&[u8]`) for specifying additional behavior.
///   If `flags` is `b"s"`, the wildcard matching will be case-sensitive.
///   Any other value or an empty slice indicates case-insensitive matching.
///
/// # Returns
///
/// Returns a `Vec<u8>` containing the result of the replacement.
/// If no match is found according to the `wildcard_pattern`, the `source`
/// byte slice is returned unchanged (cloned into a new `Vec<u8>`).
///
/// # Panics
///
/// * If `wildcard_pattern` is not valid UTF-8.
/// * If `replacement` is not valid UTF-8.
/// * If the `wildcard_pattern` results in an invalid regular expression (e.g., `**`).
#[derive(Debug, Default)]
pub struct WildcardReplaceFunction {}

impl FunctionDefinition for WildcardReplaceFunction {
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
            2 => {
                next_param
                    .arg_kind()
                    .expect(super::FunctionArgKind::Literal)?;
                next_param.expect_val_type(iter::once(Type::Bytes.into()))?;
            }
            3 => {
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
        (3, Some(1))
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
