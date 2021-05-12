plugins {
    id("org.jetbrains.kotlin.jvm") version "1.4.31"
    application
}

repositories {
    mavenCentral()
}

dependencies {
    implementation(platform("org.jetbrains.kotlin:kotlin-bom"))
    implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk8")
    implementation("com.github.ajalt", "clikt", "2.4.0")
    implementation("com.google.guava:guava:30.0-jre")
    implementation("org.slf4j", "slf4j-api", "1.7.30")
    implementation("ch.qos.logback", "logback-classic", "1.2.3")

    testImplementation("org.jetbrains.kotlin:kotlin-test")
    testImplementation("org.jetbrains.kotlin:kotlin-test-junit5")
    testImplementation("org.junit.jupiter", "junit-jupiter", "5.6.0")

}

application {
    mainClass.set("tech.skagedal.tracker.AppKt")
}

tasks {
    test {
        useJUnitPlatform()
        testLogging {
            events = setOf(
                org.gradle.api.tasks.testing.logging.TestLogEvent.STARTED,
                org.gradle.api.tasks.testing.logging.TestLogEvent.PASSED,
                org.gradle.api.tasks.testing.logging.TestLogEvent.FAILED
            )
            // show standard out and standard error of the test
            // JVM(s) on the console
            showStandardStreams = true
        }
    }
}

java {
    sourceCompatibility = org.gradle.api.JavaVersion.VERSION_11
    targetCompatibility = org.gradle.api.JavaVersion.VERSION_11
}

tasks.withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile> {
    kotlinOptions {
        jvmTarget = "11"
    }
}