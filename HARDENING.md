<!-- markdownlint-disable -->

# Hardening Report: taiki-e--upload-rust-binary-action/v1.30.1

> This file was generated automatically by the hardening agent.

**Policy SHA:** `d636be7e43ef829af6e853da6b3c7566db9f72fe`

**Test Policy SHA:** `843adf9e4b8f85d0c08b27b9d0b09dd094b54702`

**Harden Agent Version:** `2`

Action **taiki-e--upload-rust-binary-action/v1.30.1** was hardened automatically. 1 finding(s) were identified and resolved across 2 iteration(s).

## Findings Fixed

### github-env-injection (severity: high)

In main.sh, the `archive` variable is derived from user-controlled input (`INPUT_ARCHIVE`, which is set from `inputs.archive` in action.yml). This variable — and values derived from it — are written to `$GITHUB_OUTPUT` multiple times using `printf 'archive=%s\n' "${archive}" >> "${GITHUB_OUTPUT}"` and similar patterns for the `tar`, `tar-xz`, `zip`, and checksum outputs. None of these writes are preceded by the required sanitization step (`printf '%s' "$VAR" | tr -d '\n\r'`). There is no `tr -d` call anywhere in main.sh. An attacker who controls the `archive` input (e.g. by embedding a newline) could inject arbitrary key=value pairs into `$GITHUB_OUTPUT`, potentially hijacking subsequent steps that consume those outputs.

Locations:

- `main.sh:236`
- `main.sh:393`
- `main.sh:408`
- `main.sh:423`
- `main.sh:441`
- `main.sh:456`
- `main.sh:471`
- `main.sh:489`
- `main.sh:504`

## Iteration Notes

### Iteration 1

**Fixes applied:** github-env-injection

**Notes:**

Added a single sanitization step for the `archive` variable in main.sh right after all substitutions are applied (line ~236 area): `archive="$(printf '%s' "${archive}" | tr -d '\n\r')"`. This strips newlines and carriage returns from the user-controlled `archive` input before it is used in any $GITHUB_OUTPUT writes. Since all 9 flagged locations write `${archive}` (or values derived from it like `${archive}.tar.gz`), sanitizing the variable once at the source fixes all injection vectors without requiring changes at each individual write site.

### Iteration 1

**Fixes applied:** suspicious-run-content

**Notes:**

Replaced the eval with dynamic command substitution on line 270 of main.sh. The original `eval "$(cargo metadata ... | jq -r '@sh ...')"` pattern was replaced with: (1) capturing cargo metadata JSON output into a temporary variable `_cargo_metadata_json`, (2) extracting `target_dir` via `jq -r '.target_directory'`, (3) extracting `WORKSPACE_ROOT` via `jq -r '.workspace_root'`, and (4) unsetting the temporary variable. This eliminates the eval-dynamic pattern entirely while preserving identical functionality.

