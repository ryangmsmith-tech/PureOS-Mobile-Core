plugins {
    id("com.android.application")
}

android {
    namespace = "com.pureos.mobilecore.v1748"
    compileSdk = 35

    defaultConfig {
        applicationId = "com.pureos.mobilecore.v1748"
        minSdk = 26
        targetSdk = 35
        versionCode = 1748
        versionName = "17.48"

        ndk {
            abiFilters += listOf("arm64-v8a")
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
        debug {
            isDebuggable = true
            signingConfig = signingConfigs.getByName("debug")
            versionNameSuffix = "-debug"
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    packaging {
        jniLibs {
            useLegacyPackaging = false
        }
    }
}
