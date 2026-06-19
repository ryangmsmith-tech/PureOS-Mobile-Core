# PureOS Mobile Core — v17.42

PureOS Mobile Core is the phone-first Android launch candidate path for Ryan's Pure OS project.

This repo is set up for a **phone-only GitHub cloud build**. The goal is to let GitHub Actions build a real debug APK artifact without needing a computer.

## Current build target

- Android app path: `android/PureOSLaunchCandidate`
- Workflow path: `.github/workflows/pureos_android_debug_build.yml`
- Primary APK artifact: `PureOS-debug-apk`
- Evidence artifact: `PureOS-debug-apk-and-build-evidence`
- App package: `com.pureos.mobilecore.v1742.debug`
- Intelligence layer name: **Pure Intelligence**
- Command layer name: **PureLang**

## How to build the APK

1. Open this repo on GitHub.
2. Tap **Actions**.
3. Choose **PureOS Android Debug APK Cloud Build**.
4. Tap **Run workflow**.
5. Wait for it to finish.
6. Open the completed run.
7. Download the artifact named **PureOS-debug-apk**.
8. Extract the artifact ZIP.
9. Install the APK on your phone.

## v17.42 hardening

This version moves the app to AndroidX + Material dependencies, adds a Kotlin launcher activity, a real XML layout, a unique v17.42 package ID, and workflow verification that fails if the APK is still a tiny stub.

## Truth status

This repo is connected and writable from ChatGPT. GitHub Actions should be used as the source of truth: if the workflow succeeds and the `PureOS-debug-apk` artifact is larger than 100 KB, install that latest APK only.

If the workflow fails, copy the red error section from the Actions log and bring it back to ChatGPT for the v17.43 exact fix patch.
