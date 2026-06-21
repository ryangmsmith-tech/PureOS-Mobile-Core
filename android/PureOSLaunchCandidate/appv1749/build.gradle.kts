plugins {
    id("com.android.application")
}

android {
    namespace = "com.pureos.mobilecore.s25.v1749"
    compileSdk = 35

    defaultConfig {
        applicationId = "com.pureos.mobilecore.s25.v1749"
        minSdk = 26
        targetSdk = 35
        versionCode = 1749
        versionName = "17.49"

        ndk {
            abiFilters += listOf("arm64-v8a")
        }
    }

    buildTypes {
        debug {
            isDebuggable = true
            signingConfig = signingConfigs.getByName("debug")
            versionNameSuffix = "-debug"
        }
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    packaging {
        jniLibs {
            useLegacyPackaging = true
        }
    }
}
