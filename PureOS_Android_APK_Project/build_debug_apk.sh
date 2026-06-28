#!/usr/bin/env sh
set -eu
if [ -x ./gradlew ] && [ -f gradle/wrapper/gradle-wrapper.jar ]; then
  exec ./gradlew assembleDebug
elif command -v gradle >/dev/null 2>&1; then
  exec gradle assembleDebug
else
  echo 'Gradle is not installed in this shell. Open this folder in Android Studio, let it sync, then Build > Build APK(s).'
  exit 2
fi
