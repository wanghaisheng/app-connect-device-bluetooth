import org.jlleitschuh.gradle.ktlint.KtlintExtension

// Top-level build file where you can add configuration options common to all sub-projects/modules.
plugins {
    id("com.android.application") version "8.0.2" apply false
    id("com.android.library") version "8.0.2" apply false
    id("org.jetbrains.kotlin.android") version "1.8.22" apply false
    id("com.google.dagger.hilt.android") version "2.46.1" apply false
    id("com.google.devtools.ksp") version "1.8.22-1.0.11" apply false
    id("org.jlleitschuh.gradle.ktlint") version "11.4.2"
}

configure<KtlintExtension> {
    android.set(true)
}
