<!-- markdownlint-disable -->

# Hardening Report: taiki-e--upload-rust-binary-action/v1.29.1

> This file was generated automatically by the hardening agent.

**Policy SHA:** `d636be7e43ef829af6e853da6b3c7566db9f72fe`

**Test Policy SHA:** `843adf9e4b8f85d0c08b27b9d0b09dd094b54702`

**Harden Agent Version:** `2`

Action **taiki-e--upload-rust-binary-action/v1.29.1** was hardened automatically. 1 finding(s) were identified and resolved across 2 iteration(s).

## Findings Fixed

### github-env-injection (severity: high)

In main.sh, the `archive` variable is derived from user-controlled inputs (`INPUT_ARCHIVE` which maps to `inputs.archive`, and `INPUT_REF`/`inputs.ref` via the `tag` variable). This value is written to `$GITHUB_OUTPUT` at multiple locations without the required sanitization step (`printf '%s' "$VAR" | tr -d '\n\r'`). An attacker who controls the `archive` or `ref` input can inject newline characters to poison `GITHUB_OUTPUT` with arbitrary key=value pairs, potentially overwriting outputs consumed by downstream steps. The same unsanitized `${archive}` value is also used in all other GITHUB_OUTPUT writes (tar, tar-xz, zip, checksum outputs).

Locations:

- `main.sh:237`
- `main.sh:390`
- `main.sh:406`
- `main.sh:422`
- `main.sh:449`
- `main.sh:465`
- `main.sh:481`
- `main.sh:519`

## Iteration Notes

### Iteration 1

**Fixes applied:** github-env-injection

**Notes:**

Added a single sanitization step for the `archive` variable in main.sh, immediately after all placeholder substitutions ($bin, $target, $tag) and before the first GITHUB_OUTPUT write. The fix uses `archive="$(printf '%s' "${archive}" | tr -d '\n\r')"` to strip newline and carriage return characters from the user-controlled value. Since all 8 flagged GITHUB_OUTPUT write locations (lines 237, 390, 406, 422, 449, 465, 481, 519) use the same `${archive}` variable, this single sanitization point fixes all of them without requiring changes at each individual write site.

### Iteration 2

**Fixes applied:** unpinned-uses

**Notes:**

Pinned all 6 unpinned `uses:` references to full 40-character commit SHAs:

**ci.yml:**
- `taiki-e/github-actions/.github/workflows/tidy.yml@main` → `@949aa0f9e198f5cf3b32b9721f4612904b08939b # main`
- `taiki-e/checkout-action@v1` → `@7d1e50e93dc4fb3bba58f85018fadf77898aee8b # v1`
- `taiki-e/github-actions/install-rust@stable` → `@ad0ff15a3caeeca178d3800bf67da910b1ae4e6d # stable`
- `taiki-e/setup-cross-toolchain-action@v1` → `@12b7ad4acfa95a1476779d6c06699b96ec1691f8 # v1`

**release.yml:**
- `taiki-e/checkout-action@v1` → `@7d1e50e93dc4fb3bba58f85018fadf77898aee8b # v1`
- `taiki-e/create-gh-release-action@v1` → `@eba8ea96c86cca8a37f1b56e94b4d13301fba651 # v1`

Original tag/branch refs preserved as inline comments for readability.

