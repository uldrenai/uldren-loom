// JVM binding for Uldren Loom via the Foreign Function & Memory API (JDK 22+).
plugins {
    `java-library`
}

group = "ai.uldren"
version = "0.0.0"

java {
    // FFM is a stable API from JDK 22 onward. Pin the vendor so Gradle uses (or downloads via the
    // Foojay resolver in settings.gradle.kts) Eclipse Temurin rather than another detected JDK.
    toolchain {
        languageVersion = JavaLanguageVersion.of(22)
        vendor = JvmVendorSpec.ADOPTIUM
    }
}

repositories { mavenCentral() }

tasks.withType<JavaCompile>().configureEach {
    // All lints except `restricted`: the FFM downcalls in Loom.java are intentional native interop,
    // guarded at runtime by --enable-native-access.
    options.compilerArgs.add("-Xlint:all,-restricted")
}

tasks.test {
    useJUnitPlatform()
    jvmArgs("--enable-native-access=ALL-UNNAMED")
}
