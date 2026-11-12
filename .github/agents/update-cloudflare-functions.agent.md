---
name: Update Cloudflare Functions
description: Reconcile Wirefilter function definitions with Cloudflare's Rules language documentation
tools: [execute, read, edit, search, web, 'cloudflare-mcp-docs/*']
mcp-servers:
  cloudflare-mcp-docs:
    type: http
    url: https://docs.mcp.cloudflare.com/mcp
    deferTools: auto
    tools: ["*"]
---

Update the built-in function definitions in `engine/src/functions` so their names, arguments, argument types, optionality, and return types match Cloudflare's Rules language function reference:
https://developers.cloudflare.com/ruleset-engine/rules-language/functions/

Use the connected `cloudflare-mcp-docs` MCP server at `https://docs.mcp.cloudflare.com/mcp` to retrieve the reference. Do not rely on memory or other sources. If the documentation search does not provide enough detail to establish the complete function catalog or a signature, search the same MCP documentation for the missing information. Do not guess; report any unresolved documentation gaps.

## Repository conventions

- Read `engine/src/functions/README.md`, `engine/src/functions/mod.rs`, and relevant neighboring implementations before editing.
- Keep one function implementation per file. Follow existing file, type, constructor, and module naming patterns.
- Keep `engine/src/functions/mod.rs` module declarations and public re-exports consistent with the implementation files.
- In `check_param`, use the number of already-checked parameters to identify the next positional argument. Check argument kind separately from value type: `FunctionArgKind::Field` means a non-literal/dynamic field, and `FunctionArgKind::Literal` means a literal. Use the existing `arg_kind().expect(...)` and `expect_val_type(...)` helpers; use `expect_const_value(...)` when the documented signature constrains a literal's value. For type-dependent arguments, derive the expected type from the already-checked parameter. Use `ExpectedType::Array` or `ExpectedType::Map` only when the signature accepts any corresponding container type. Keep any existing `arg_kind().expect(...)` checks even if constraints are not mentioned in the documentation.
- In `return_type`, follow the documented return type, deriving it from checked parameters where the function's return type depends on them.
- `arg_count` returns `(mandatory_count, optional_count)`: use `Some(0)` when there are no optional arguments, `Some(n)` for a fixed number of optional arguments, and `None` for an unbounded number of additional arguments.
- `compile` is required by the trait but is not used by this project. Implement it with `unimplemented!()`; do not add runtime function behavior.
- Always ignore `any`, `all`, and `bit_slice`, even though it appears in the online documentation. Never add a local implementation, module declaration, or re-export for it. If a local implementation exists, remove it and its registrations.

## Workflow

1. Read the current function files and module registrations, then use the Cloudflare docs MCP server to establish the documented function names and signatures.
2. Reconcile the local definitions with the documentation. Add or update definitions as needed, and remove definitions that should not be present. Do not change unrelated code.
3. Keep each argument's required/optional status and field/literal restriction aligned with the documentation; do not infer one from the other.
4. Run `cargo fmt --all -- --check` and `cargo test -p wirefilter-engine`. Fix issues caused by your changes.
5. Summarize the functions added, changed, or removed, and report the documentation source and validation results.
