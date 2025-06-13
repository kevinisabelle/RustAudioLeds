plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("org.jetbrains.kotlin.plugin.compose")


    // Hilt + KAPT for code generation
    id("com.google.dagger.hilt.android")
    kotlin("kapt")
}

android {
    namespace = "com.kevinisabelle.visualizerui"
    compileSdk = 35

    defaultConfig {
        applicationId = "com.kevinisabelle.visualizerui"
        minSdk = 29
        targetSdk = 35
        versionCode = 1
        versionName = "1.0"
    }

    buildFeatures { compose = true }
    compileOptions { sourceCompatibility = JavaVersion.VERSION_11; targetCompatibility = JavaVersion.VERSION_11 }
    kotlinOptions { jvmTarget = "11" }
}

dependencies {
    // ── Jetpack Compose core ───────────────────────────────
    implementation(platform("androidx.compose:compose-bom:2024.09.00"))
    implementation("androidx.compose.ui:ui")
    implementation("androidx.compose.ui:ui-tooling-preview")
    implementation("androidx.compose.material3:material3")
    implementation("androidx.activity:activity-compose:1.8.0")
    implementation("com.godaddy.android.colorpicker:compose-color-picker:0.7.0")

    // ── Navigation-Compose ─────────────────────────────────
    implementation("androidx.navigation:navigation-compose:2.9.0")

    // ── Accompanist Permissions (rememberMultiplePermissionsState) ─
    implementation("com.google.accompanist:accompanist-permissions:0.37.3")

    // ── Hilt core + Compose integration ───────────────────
    implementation("com.google.dagger:hilt-android:2.56.2")
    kapt("com.google.dagger:hilt-compiler:2.56.2")
    implementation("androidx.hilt:hilt-navigation-compose:1.2.0")

    // ── Kotlin + AndroidX basics you already had ───────────
    implementation("androidx.core:core-ktx:1.10.1")
    implementation("androidx.lifecycle:lifecycle-runtime-ktx:2.6.1")
    implementation("androidx.appcompat:appcompat:1.6.1")
    implementation("androidx.preference:preference:1.2.0")
    implementation("com.google.android.material:material:1.12.0")
    implementation("androidx.compose.material:material-icons-extended:1.7.8")

    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.5")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.1")
    androidTestImplementation(platform("androidx.compose:compose-bom:2024.09.00"))
    androidTestImplementation("androidx.compose.ui:ui-test-junit4")
    debugImplementation("androidx.compose.ui:ui-tooling")
    debugImplementation("androidx.compose.ui:ui-test-manifest")

    // --- Lottie for Compose animations ─────────────────────
    implementation("com.airbnb.android:lottie-compose:6.4.0")
    testImplementation(kotlin("test"))
}
