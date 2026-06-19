# Agent Mode Prompt — PureOS Mobile Core

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
If the build fails, identify the exact failing line and produce the smallest v17.41 fix patch.
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

## Expected artifact

```text
PureOS-debug-apk-and-build-evidence
```
