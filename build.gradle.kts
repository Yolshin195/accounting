plugins {
    kotlin("jvm") version "1.9.25" apply false
    kotlin("plugin.spring") version "1.9.25" apply false
    kotlin("plugin.jpa") version "1.9.25" apply false
    id("org.springframework.boot") version "3.5.0" apply false
    id("io.spring.dependency-management") version "1.1.7" apply false
}

allprojects {
    group = "com.accounting"
    version = "0.0.6-SNAPSHOT"

    repositories {
        mavenCentral()
    }
}

// 👇 Добавь ниже — задачи Docker Compose
tasks.register<Exec>("dockerUp") {
    group = "docker"
    description = "Build and start all containers using docker-compose"
    commandLine("docker-compose", "up", "--build", "-d")
}

tasks.register<Exec>("dockerDown") {
    group = "docker"
    description = "Stop and remove all containers using docker-compose"
    commandLine("docker-compose", "down")
}