# PureOS Mobile Core — v17.49

PureOS Mobile Core is the phone-first Android launch path for Ryan's Pure OS project.

## Current Android build target

- Clean-install app module: `android/PureOSLaunchCandidate/appv1749`
- Native Rust path: `native/PureOSNativeRuntimeBootstrap`
- Workflow: `.github/workflows/pureos_android_debug_build.yml`
- APK artifact: `PureOS-v17.49-samsung-s25-arm64-clean-install-apk`
- Evidence artifact: `PureOS-v17.49-samsung-s25-arm64-build-evidence`
- Fresh package identity: `com.pureos.mobilecore.s25.v1749`
- Supported APK ABI: `arm64-v8a`
- Native library packaging: extracted compatibility mode
- Intelligence layer: **Pure Intelligence**
- Command layer: **PureLang**

## Why v17.49 exists

The v17.48 APK built successfully but one Samsung Galaxy S25 Ultra reported **App not installed**. The v17.49 target removes likely install conflicts by using:

- a brand-new application ID
- a completely separate Android app module
- explicit ARM64-only packaging
- native library extraction compatibility mode
- an explicit fully-qualified launcher activity

## Phone build and install

1. Open this repository on GitHub.
2. Tap **Actions**.
3. Choose **PureOS Android ARM64 Clean-Install APK Cloud Build**.
4. Run the workflow and wait for success.
5. Download `PureOS-v17.49-samsung-s25-arm64-clean-install-apk`.
6. Extract the artifact ZIP.
7. Before installation, allow **My Files** or the browser to install unknown apps.
8. On Samsung, turn off **Auto Blocker** temporarily if it blocks sideloading.
9. Tap the APK and install it.
10. Open **PureOS Mobile Core v17.49** and run the native contract check.

## Truth status

A successful install and contract check prove that the Android app can load the Rust ARM64 JNI library and validate the v17.39A runtime contract on the phone.

They do not yet prove a native GPU-rendered Gold Ocean City frame, live vehicle physics, headset execution, or production deployment.
