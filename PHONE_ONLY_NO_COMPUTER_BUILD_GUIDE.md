# Phone-Only PureOS Android Build Guide

This repo is built so Ryan can create an APK from a phone using GitHub Actions.

## Build steps

1. Open the repo on GitHub.
2. Tap **Actions**.
3. Open **PureOS Android Debug APK Cloud Build**.
4. Tap **Run workflow**.
5. Wait for the run to finish.
6. Open the completed run.
7. Download **PureOS-debug-apk**.
8. Extract the artifact ZIP.
9. Install the APK inside it.

## Important

Do not install APKs that are only a few KB. v17.42 fails the workflow if the APK is under 100 KB, because tiny stubs caused install errors.

The APK should use this debug package:

```text
com.pureos.mobilecore.v1742.debug
```

If installation still fails, uninstall older PureOS Mobile Core test apps, then install the newest APK from the latest successful Actions run.
