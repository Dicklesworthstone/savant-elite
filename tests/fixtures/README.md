# Test Fixtures

This directory contains configuration files for testing `savant config check` and other config-related functionality.

## Directory Structure

### `valid/`
Configurations that should pass validation:
- `basic.conf` - Standard copy/paste configuration
- `all_modifiers.conf` - Uses all modifier combinations (ctrl+shift+alt+cmd)
- `function_keys.conf` - Uses function keys (F1, F6, F12)
- `special_keys.conf` - Uses special keys (enter, space, tab)
- `with_whitespace.conf` - Extra whitespace and blank lines (should be trimmed)

### `invalid/`
Configurations that should fail validation:
- `missing_left.conf` - Missing required 'left' field
- `missing_middle.conf` - Missing required 'middle' field
- `missing_right.conf` - Missing required 'right' field
- `invalid_key.conf` - Uses unknown key name ('notakey')
- `invalid_modifier.conf` - Uses unknown modifier ('hyper')
- `malformed_syntax.conf` - Line without '=' separator
- `empty.conf` - Empty file (missing all fields)

### `edge_cases/`
Edge cases that test parser behavior:
- `equals_in_value.conf` - Value contains '=' character (cmd+=)
- `duplicate_keys.conf` - Same key appears twice (last value wins)
- `unknown_fields.conf` - Extra fields that should be ignored
- `comments.conf` - Lines starting with '#' (flagged by strict check, ignored by parser)

## Usage

In shell scripts:
```bash
FIXTURES="$(dirname "$0")/../fixtures"
savant config check "$FIXTURES/valid/basic.conf"
```

In Rust tests:
```rust
let fixture = include_str!("../tests/fixtures/valid/basic.conf");
```

## Notes

- The parser (`PedalConfig::parse`) is lenient and ignores malformed lines
- The validator (`savant config check`) is strict and reports all issues
- Valid configs should pass both parsing and validation
- Invalid configs should fail validation with clear error messages
