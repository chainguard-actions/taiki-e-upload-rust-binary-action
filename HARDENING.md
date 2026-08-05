<!-- markdownlint-disable -->

# Hardening Report: taiki-e--upload-rust-binary-action/v1.30.0

> This file was generated automatically by the hardening agent.

**Policy SHA:** `d636be7e43ef829af6e853da6b3c7566db9f72fe`

**Test Policy SHA:** `843adf9e4b8f85d0c08b27b9d0b09dd094b54702`

**Harden Agent Version:** `2`

Action **taiki-e--upload-rust-binary-action/v1.30.0** was hardened automatically. 2 finding(s) were identified and resolved across 1 iteration(s).

## Findings Fixed

### unpinned-uses (severity: high)

Multiple workflow files reference external actions using mutable tags or branch names instead of pinned 40-character commit SHAs. This exposes the workflow to supply-chain attacks if the referenced tag or branch is moved or compromised.

In .github/workflows/ci.yml:
- `uses: taiki-e/github-actions/.github/workflows/tidy.yml@main` (line 33)
- `uses: taiki-e/checkout-action@v1` (line 73)
- `uses: taiki-e/github-actions/install-rust@stable` (line 74)
- `uses: taiki-e/setup-cross-toolchain-action@v1` (line 75)

In .github/workflows/release.yml:
- `uses: taiki-e/github-actions/.github/workflows/action-release.yml@main` (line 28)

All of these should be pinned to a full 40-character hex commit SHA.

Locations:

- `.github/workflows/ci.yml:33`
- `.github/workflows/ci.yml:73`
- `.github/workflows/ci.yml:74`
- `.github/workflows/ci.yml:75`
- `.github/workflows/release.yml:28`

### github-env-injection (severity: high)

In main.sh, the `archive` variable is derived from user-controlled inputs (INPUT_ARCHIVE, INPUT_BIN, INPUT_TARGET, INPUT_REF) and written to $GITHUB_OUTPUT using `printf '%s\n' "${archive}" >> "${GITHUB_OUTPUT}"` without the required sanitization step (`printf '%s' ... | tr -d '\n\r'`). An attacker who controls any of these inputs could inject newlines into $GITHUB_OUTPUT to set arbitrary output variables or environment values. The same unsanitized `archive` value is also used in subsequent tar, tar-xz, zip, and checksum output writes (e.g., `printf 'tar=%s.tar.gz\n' "${archive}" >> "${GITHUB_OUTPUT}"`). None of these writes apply the required `tr -d '\n\r'` sanitization before writing to the special environment file.

Locations:

- `main.sh:228`

## Iteration Notes

### Iteration 1

**Fixes applied:** unpinned-uses, github-env-injection

**Notes:**

Fixed 5 unpinned action references in .github/workflows/ci.yml and .github/workflows/release.yml by pinning each to its full 40-character commit SHA (preserving the original tag/branch as a comment). Fixed github-env-injection in main.sh by sanitizing the user-controlled `archive` variable with `tr -d '\n\r'` immediately after all substitutions are applied, before any writes to $GITHUB_OUTPUT. This single sanitization protects all subsequent GITHUB_OUTPUT writes (archive, tar, tar-xz, zip, and checksum outputs) since they all use the same `archive` variable.

