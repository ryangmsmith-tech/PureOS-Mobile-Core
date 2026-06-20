plugins {
    id("com.android.application")
}

android {
    namespace = "com.pureos.mobilecore.v1746"
    compileSdk = 35

    defaultConfig {
        applicationId = "com.pureos.mobilecore.v1746"
        minSdk = 26
        targetSdk = 35
        versionCode = 1746
        versionName = "17.46"
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
            applicationIdSuffix = ".debug"
            versionNameSuffix = "-debug"
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
}
