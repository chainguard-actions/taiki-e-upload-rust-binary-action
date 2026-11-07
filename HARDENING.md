<!-- markdownlint-disable -->

# Hardening Report: taiki-e--upload-rust-binary-action/v1.30.1

> This file was generated automatically by the hardening agent.

**Policy SHA:** `374911c4be584df24e6da57af2d990570ef0f859`

**Test Policy SHA:** `843adf9e4b8f85d0c08b27b9d0b09dd094b54702`

**Harden Agent Version:** `2`

Action **taiki-e--upload-rust-binary-action/v1.30.1** was hardened automatically. 3 finding(s) were identified and resolved across 1 iteration(s).

## Findings Fixed

### github-env-injection (severity: high)

Carried forward from this version's previous harden, which fixed it; fix it again unless the code no longer has the problem. In main.sh, the `archive` variable is derived from user-controlled input (`INPUT_ARCHIVE`, which is set from `inputs.archive` in action.yml). This variable — and values derived from it — are written to `$GITHUB_OUTPUT` multiple times using `printf 'archive=%s\n' "${archive}" >> "${GITHUB_OUTPUT}"` and similar patterns for the `tar`, `tar-xz`, `zip`, and checksum outputs. None of these writes are preceded by the required sanitization step (`printf '%s' "$VAR" | tr -d '\n\r'`). There is no `tr -d` call anywhere in main.sh. An attacker who controls the `archive` input (e.g. by embedding a newline) could inject arbitrary key=value pairs into `$GITHUB_OUTPUT`, potentially hijacking subsequent steps that consume those outputs.

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

### prior-fix (severity: high)

Carried forward from this version's previous harden, which fixed it; fix it again unless the code no longer has the problem. The previous harden changed action.yml without recording a finding for it, and this analysis reported nothing there. Re-apply its security changes unless the code no longer needs them. The previous patch:
```diff
diff --git a/action.yml b/action.yml
index 336d313..59690b9 100644
--- a/action.yml
+++ b/action.yml
@@ -1,11 +1,10 @@
 name: Build and upload Rust binary to GitHub Releases
 description: GitHub Action for building and uploading Rust binary to GitHub Releases
-
 inputs:
   bin:
     description: >
-      Binary names (non-extension portion of filename) to build and upload (whitespace or comma separated list).
-      Note that glob pattern is not supported yet.
+      Binary names (non-extension portion of filename) to build and upload (whitespace or comma separated list). Note that glob pattern is not supported yet.
+
     required: true
   archive:
     description: Archive name (non-extension portion of filename) to be uploaded (variables `$bin`, `$target`, `$tag`, and any string)
@@ -59,13 +58,13 @@ inputs:
     default: 'windows'
   include:
     description: >
-      Additional files to be included to the archive (whitespace or comma separated list).
-      Note that glob pattern is not supported yet.
+      Additional files to be included to the archive (whitespace or comma separated list). Note that glob pattern is not supported yet.
+
     required: false
   asset:
     description: >
-      Additional files to be uploaded separately (whitespace or comma separated list).
-      Note that glob pattern is not supported yet.
+      Additional files to be uploaded separately (whitespace or comma separated list). Note that glob pattern is not supported yet.
+
     required: false
   leading-dir:
     description: Whether to create the leading directory in the archive or not
@@ -87,21 +86,22 @@ inputs:
     description: >
       Algorithms to be used for checksum (sha256, sha512, b2, sha1, or md5) (whitespace or comma separated list)
 
-      Note: b2 is not available by default on macOS, install `b2sum` to use it.
-      sha1 and md5 are insecure and strongly discouraged.
+      Note: b2 is not available by default on macOS, install `b2sum` to use it. sha1 and md5 are insecure and strongly discouraged.
+
     required: false
   token:
     description: >
       GitHub token for uploading assets to GitHub Releases.
 
-      If not set this option, the GITHUB_TOKEN environment variable will be used.
-      If not set both this option and the GITHUB_TOKEN environment variable, github.token will be used.
+      If not set this option, the GITHUB_TOKEN environment variable will be used. If not set both this option and the GITHUB_TOKEN environment variable, github.token will be used.
+
     required: false
   ref:
     description: >
       Fully-formed tag ref for this release.
 
       If not set this option, the GITHUB_REF environment variable (automatically set by GitHub Actions) will be used.
+
     required: false
   profile:
     description: The cargo profile to build. This defaults to the release profile.
@@ -112,10 +112,12 @@ inputs:
       Build and compress binaries, but do not upload them.
 
       Note that some errors are downgraded to warnings in this mode.
+
     required: false
   dry-run-intended:
     description: >
       Suppress informational warnings for `dru-run` keeping the rest
+
     required: false
     default: 'false'
   dry_run:
@@ -137,7 +139,6 @@ inputs:
   codesign_options:
     description: Alias for 'codesign-options'
     required: false
-
 outputs:
   archive:
     description: 'Archive base name'
@@ -166,13 +167,29 @@ outputs:
   md5:
     description: 'MD5 checksum file name'
     value: ${{ steps.upload-rust-binary-action.outputs.md5 }}
-
 # Note:
 # - inputs.* should be manually mapped to INPUT_* due to https://github.com/actions/runner/issues/665
 # - Use GITHUB_*/RUNNER_* instead of github.*/runner.* due to https://github.com/actions/runner/issues/2185
 runs:
   using: composite
   steps:
     - id: upload-rust-binary-action
       run: bash --noprofile --norc "${GITHUB_ACTION_PATH:?}/main.sh"
       shell: bash

```

Locations:

- `action.yml`

### prior-fix (severity: high)

Carried forward from this version's previous harden, which fixed it; fix it again unless the code no longer has the problem. The previous harden changed main.sh without recording a finding for it, and this analysis reported nothing there. Re-apply its security changes unless the code no longer needs them. The previous patch:
```diff
diff --git a/main.sh b/main.sh
index e82fbd6..f960726 100755
--- a/main.sh
+++ b/main.sh
@@ -291,6 +291,8 @@ fi
 archive="${archive/\$bin/${bin_names[0]}}"
 archive="${archive/\$target/${target}}"
 archive="${archive/\$tag/${tag}}"
+# Sanitize archive to prevent newline injection into $GITHUB_OUTPUT / $GITHUB_ENV.
+archive="$(printf '%s' "${archive}" | tr -d '\n\r')"
 
 if [[ -n "${GITHUB_OUTPUT:-}" ]]; then
   printf 'archive=%s\n' "${archive}" >>"${GITHUB_OUTPUT}"
@@ -344,7 +346,10 @@ if [[ -n "${manifest_path}" ]]; then
   build_options+=(--manifest-path "${manifest_path}")
   metadata_options+=(--manifest-path "${manifest_path}")
 fi
-eval "$(cargo metadata "${metadata_options[@]}" | jq -r '@sh "target_dir=\(.target_directory) WORKSPACE_ROOT=\(.workspace_root)"')"
+_cargo_metadata_json="$(cargo metadata "${metadata_options[@]}")"
+target_dir="$(printf '%s' "${_cargo_metadata_json}" | jq -r '.target_directory')"
+WORKSPACE_ROOT="$(printf '%s' "${_cargo_metadata_json}" | jq -r '.workspace_root')"
+unset _cargo_metadata_json
 
 # Skip setting the strip option (requires Cargo 1.59) if it is unavailable or already set.
 # TODO: This check is somewhat rough as it does not look at the type of profile.

```

Locations:

- `main.sh`

## Iteration Notes

### Iteration 1

**Fixes applied:** github-env-injection, prior-fix

**Notes:**

main.sh: added archive="$(printf '%s' "${archive}" | tr -d '\n\r')" right after the $bin/$target/$tag substitutions. Every write of archive or a value built from it to $GITHUB_OUTPUT (archive, tar, tar-xz, zip, checksum outputs) comes after this line. pipefail is set, so a failure in the pipe still stops the script. Also re-applied the prior fix that replaces the eval of jq @sh output with plain jq -r reads of cargo metadata JSON that has already been captured. action.yml: the prior patch only reflowed description text (folded scalars). It made no security change, and the env-mapped inputs are already in place, so I left it unchanged.

