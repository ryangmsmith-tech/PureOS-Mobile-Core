# PureOS Mobile Core Project Map

## Important paths

```text
.github/workflows/pureos_android_debug_build.yml
android/PureOSLaunchCandidate/
android/PureOSLaunchCandidate/app/src/main/java/com/pureos/mobile/MainActivity.java
AGENT_MODE_GITHUB_PROMPT.md
TRUTH_STATUS.json
```

## What exists now

This repository contains a minimal Android launch candidate that should build into a debug APK through GitHub Actions.

The app currently shows a PureOS Mobile Core status shell with:

- PureOS Mobile Core title
- Pure Intelligence status placeholder
- PureLang command-layer placeholder
- Build receipt instructions

## What does not exist yet

This is not the full Pure OS runtime yet. It is the first cloud-buildable phone shell.

Missing future pieces include:

- full Pure Intelligence runtime loop
- PureLang parser/interpreter bridge
- Governor approval queue
- module streaming
- VR/world rendering stack
- real device diagnostics and build receipts

## Next target

v17.41 should be one of these:

1. APK artifact pass seal if GitHub Actions succeeds.
2. Exact build error fix patch if GitHub Actions fails.
