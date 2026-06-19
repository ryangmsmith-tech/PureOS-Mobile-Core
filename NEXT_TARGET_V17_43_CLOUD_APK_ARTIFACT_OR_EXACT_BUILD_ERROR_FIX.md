# Next Target — v17.43 Cloud APK Artifact or Exact Build Error Fix

Use this if v17.42 fails.

## If the workflow fails

Copy the red error block from GitHub Actions and fix only the failing cause.

## If the APK builds but will not install

Check:

- artifact name is `PureOS-debug-apk`
- APK size is larger than 100 KB
- phone has older PureOS test apps uninstalled
- app package is `com.pureos.mobilecore.v1742.debug`

## v17.43 likely tasks

- Add a simple launch smoke test.
- Add a build report parser.
- Add PureLang parser seed files into assets.
- Add Pure Intelligence local text-loop shell after the APK installs.
