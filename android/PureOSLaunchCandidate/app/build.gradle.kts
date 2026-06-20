plugins {
    id("com.android.application")
}

android {
    namespace = "com.pureos.mobilecore.v1743"
    compileSdk = 35

    defaultConfig {
        applicationId = "com.pureos.mobilecore.v1743"
        minSdk = 26
        targetSdk = 35
        versionCode = 1743
        versionName = "17.43"
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

dependencies {
    implementation("androidx.appcompat:appcompat:1.7.0")
    implementation("androidx.constraintlayout:constraintlayout:2.1.4")
    implementation("com.google.android.material:material:1.12.0")
}
