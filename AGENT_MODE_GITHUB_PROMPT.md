# Agent Mode Prompt — PureOS Mobile Core v17.42

You are working inside Ryan's Pure OS / PureLang / Pure Intelligence project.

## Naming lock

- The intelligence layer is always called **Pure Intelligence**.
- Do not rename it.
- Use terms such as geometry-native, geometry-coded, shape-validated, PureLang, Pure Intelligence, Governor, and approval gate.

## Current mission

Build and test the Android phone launch candidate.

Primary goal:

```text
Produce a downloadable debug APK artifact from GitHub Actions.
```

Secondary goal:

```text
If the build fails, identify the exact failing line and produce the smallest v17.43 fix patch.
```

## Safety / truth rules

- Do not claim the APK works until GitHub Actions builds it and a phone install test passes.
- Keep changes small and reviewable.
- Do not remove Ryan approval gates.
- Do not add unsafe remote command execution.
- Do not claim this is a finished operating system; this repo is an Android launch candidate and cloud-build path.

## Build path

```text
android/PureOSLaunchCandidate
.github/workflows/pureos_android_debug_build.yml
```

## Expected artifacts

```text
PureOS-debug-apk
PureOS-debug-apk-and-build-evidence
```

## v17.42 check

The APK must be larger than 100 KB or the workflow should fail. Do not install tiny stub APKs.
