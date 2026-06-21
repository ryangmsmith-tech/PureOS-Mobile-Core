# PureOS Mobile Core — v17.48

PureOS Mobile Core is the phone-first Android launch path for Ryan's Pure OS project.

This repository now supports two cloud-build lines:

1. **Android ARM64 native APK** for installation on a compatible Android phone.
2. **Linux x86_64 native runtime bootstrap** for cloud compilation and evidence generation.

## Current Android build target

- Android app path: `android/PureOSLaunchCandidate`
- Native Rust path: `native/PureOSNativeRuntimeBootstrap`
- Workflow: `.github/workflows/pureos_android_debug_build.yml`
- APK artifact: `PureOS-v17.48-android-arm64-native-debug-apk`
- Evidence artifact: `PureOS-v17.48-android-arm64-native-build-evidence`
- App package: `com.pureos.mobilecore.v1748`
- Supported APK ABI: `arm64-v8a`
- Intelligence layer: **Pure Intelligence**
- Command layer: **PureLang**

## What the v17.48 APK proves

The APK packages a Rust JNI shared library compiled for Android ARM64. The app loads that library and runs the v17.39A runtime contract check on the phone.

The screen reports:

- whether the native ARM64 library loaded
- whether the Rust runtime contract passed
- whether all six Gold Ocean City slice sections are present
- the declared PureRenderIR and Pure Intelligence runtime versions

## How to build from a phone

1. Open this repository on GitHub.
2. Tap **Actions**.
3. Choose **PureOS Android ARM64 Native Debug APK Cloud Build**.
4. Tap **Run workflow**.
5. Wait for a green success result.
6. Open the completed run.
7. Download `PureOS-v17.48-android-arm64-native-debug-apk`.
8. Extract the downloaded artifact ZIP.
9. Install the APK on an ARM64 Android phone.
10. Open the app and tap **Run Native Contract Check**.

## Truth status

A successful workflow and installed APK prove that the Android app and Rust ARM64 JNI bootstrap compile, package, load, and return the expected contract result on the phone.

They do not yet prove:

- a native GPU-rendered Gold Ocean City frame
- live vehicle hover physics
- headset execution
- production deployment

The next runtime milestone after this APK is Android Vulkan device admission followed by one verified offscreen GPU frame.
