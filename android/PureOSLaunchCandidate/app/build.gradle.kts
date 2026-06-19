plugins {
    id("com.android.application")
}

android {
    namespace = "com.pureos.mobilecore.v1741"
    compileSdk = 35

    defaultConfig {
        applicationId = "com.pureos.mobilecore.v1741"
        minSdk = 26
        targetSdk = 35
        versionCode = 1741
        versionName = "17.41"
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
            applicationIdSuffix = ".debug"
            versionNameSuffix = "-debug"
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
}
