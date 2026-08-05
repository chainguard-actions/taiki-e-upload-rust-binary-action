<!-- markdownlint-disable -->

# Hardening Report: taiki-e--upload-rust-binary-action/v1.29.0

> This file was generated automatically by the hardening agent.

**Policy SHA:** `d636be7e43ef829af6e853da6b3c7566db9f72fe`

**Test Policy SHA:** `843adf9e4b8f85d0c08b27b9d0b09dd094b54702`

**Harden Agent Version:** `2`

Action **taiki-e--upload-rust-binary-action/v1.29.0** was hardened automatically. 1 finding(s) were identified and resolved across 3 iteration(s).

## Findings Fixed

### github-env-injection (severity: high)

In main.sh (called from the composite action's `run:` step), the `archive` variable — derived from `INPUT_ARCHIVE` which is set to `${{ inputs.archive }}` (a user-controlled input) — is written to `$GITHUB_OUTPUT` without the required sanitization step (`printf '%s' "$VAR" | tr -d '\n\r'`). An attacker can inject newlines into the `inputs.archive` value to smuggle arbitrary key=value pairs into GITHUB_OUTPUT, potentially overwriting other step outputs. The same unsanitized `archive` variable is also embedded in the `tar`, `tar-xz`, `zip`, and checksum output writes (e.g. `printf 'tar=%s.tar.gz\n' "${archive}" >> "${GITHUB_OUTPUT}"`). All of these writes must be preceded by a sanitization step such as: `safe_archive=$(printf '%s' "${archive}" | tr -d '\n\r')`.

Locations:

- `main.sh:97`
- `main.sh:248`

## Iteration Notes

### Iteration 1

**Fixes applied:** github-env-injection

**Notes:**

Added `safe_archive=$(printf '%s' "${archive}" | tr -d '\n\r')` immediately before the first GITHUB_OUTPUT write in main.sh (after the archive variable substitutions for $bin/$target/$tag). Replaced all 8 GITHUB_OUTPUT writes that previously used `${archive}` with `${safe_archive}`: the initial `archive=` output, the `tar=`, `tar-xz=`, and `zip=` outputs in both the leading-directory and non-leading-directory branches, and the checksum output in the checksums loop. Filesystem operations (mkdir, tar, zip, cp) continue to use the original `${archive}` variable unchanged.

### Iteration 2

**Fixes applied:** unpinned-uses

**Notes:**

Pinned all 6 unpinned `uses:` references to full 40-character commit SHAs:
- `.github/workflows/ci.yml`: `taiki-e/github-actions/.github/workflows/tidy.yml@main` → `@949aa0f9e198f5cf3b32b9721f4612904b08939b # main`
- `.github/workflows/ci.yml`: `taiki-e/checkout-action@v1` → `@7d1e50e93dc4fb3bba58f85018fadf77898aee8b # v1`
- `.github/workflows/ci.yml`: `taiki-e/github-actions/install-rust@stable` → `@ad0ff15a3caeeca178d3800bf67da910b1ae4e6d # stable`
- `.github/workflows/ci.yml`: `taiki-e/setup-cross-toolchain-action@v1` → `@12b7ad4acfa95a1476779d6c06699b96ec1691f8 # v1`
- `.github/workflows/release.yml`: `taiki-e/checkout-action@v1` → `@7d1e50e93dc4fb3bba58f85018fadf77898aee8b # v1`
- `.github/workflows/release.yml`: `taiki-e/create-gh-release-action@v1` → `@eba8ea96c86cca8a37f1b56e94b4d13301fba651 # v1`

All SHAs were resolved via lookup_action_sha. Original tag/branch names are preserved as inline comments.

### Iteration 1

**Fixes applied:** suspicious-run-content

**Notes:**

Replaced the eval-dynamic pattern `eval "$(cargo metadata ... | jq -r '@sh ...')"` at main.sh line 267 with direct variable assignments. The fix stores the cargo metadata JSON output in a temporary variable `_cargo_metadata_json`, then uses `jq -r '.target_directory'` and `jq -r '.workspace_root'` to extract values directly into `target_dir` and `WORKSPACE_ROOT` without any eval. The temporary variable is cleaned up with `unset`. This eliminates the eval+$() command substitution pattern while preserving identical functionality.

