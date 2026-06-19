# PureOS Mobile Core — v17.40

PureOS Mobile Core is the phone-first Android launch candidate path for Ryan's Pure OS project.

This repo is set up for a **phone-only GitHub cloud build**. The goal is to let GitHub Actions build a debug APK artifact without needing a computer.

## Current build target

- Android app path: `android/PureOSLaunchCandidate`
- Workflow path: `.github/workflows/pureos_android_debug_build.yml`
- Output artifact: `PureOS-debug-apk-and-build-evidence`
- Intelligence layer name: **Pure Intelligence**
- Command layer name: **PureLang**

## How to build the APK

1. Open this repo on GitHub.
2. Tap **Actions**.
3. Choose **PureOS Android Debug APK Cloud Build**.
4. Tap **Run workflow**.
5. Wait for it to finish.
6. Open the completed run.
7. Download the artifact named **PureOS-debug-apk-and-build-evidence**.
8. Install the APK on your phone.

## Truth status

This repo is now connected and writable from ChatGPT. The Android project and GitHub Actions workflow have been added so the first cloud APK build can be tested.

If the workflow fails, copy the red error section from the Actions log and bring it back to ChatGPT for the v17.41 exact fix patch.
