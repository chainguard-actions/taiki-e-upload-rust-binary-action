<!-- markdownlint-disable -->

# Hardening Report: taiki-e--upload-rust-binary-action/v1.30.2

> This file was generated automatically by the hardening agent.

**Policy SHA:** `d636be7e43ef829af6e853da6b3c7566db9f72fe`

**Test Policy SHA:** `843adf9e4b8f85d0c08b27b9d0b09dd094b54702`

**Harden Agent Version:** `2`

Action **taiki-e--upload-rust-binary-action/v1.30.2** was hardened automatically. 1 finding(s) were identified and resolved across 1 iteration(s).

## Findings Fixed

### github-env-injection (severity: high)

In main.sh, the `archive` variable is derived from user-controlled inputs (INPUT_ARCHIVE → inputs.archive, INPUT_BIN → inputs.bin, INPUT_TARGET → inputs.target, INPUT_REF → inputs.ref) and is written to $GITHUB_OUTPUT multiple times via `printf` without the required sanitization step (`printf '%s' "$VAR" | tr -d '\n\r'`). An attacker who controls any of these inputs could inject newlines to smuggle arbitrary key=value pairs into GITHUB_OUTPUT, potentially overwriting subsequent step outputs. The same unsanitized `${archive}` value is used in all tar, tar-xz, zip, and checksum output writes. Example unsanitized write: `printf 'archive=%s\n' "${archive}" >> "${GITHUB_OUTPUT}"`

Locations:

- `main.sh:310`
- `main.sh:349`
- `main.sh:366`
- `main.sh:383`
- `main.sh:406`
- `main.sh:423`
- `main.sh:440`
- `main.sh:558`

## Iteration Notes

### Iteration 1

**Fixes applied:** github-env-injection

**Notes:**

Added a single sanitization step for the `archive` variable in main.sh, right after all substitutions ($bin, $target, $tag) are applied and before any GITHUB_OUTPUT writes. The fix uses `archive="$(printf '%s' "${archive}" | tr -d '\n\r')"` to strip newlines and carriage returns, preventing injection of arbitrary key=value pairs into GITHUB_OUTPUT. This single fix covers all 8 reported vulnerable locations since they all reference the same `${archive}` variable.

