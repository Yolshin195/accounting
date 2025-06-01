# Build stage
FROM gradle:8.7-jdk21 AS builder

WORKDIR /app
COPY . .

# Build fat jar
RUN gradle clean bootJar --no-daemon

# ----------------------------------------

# Final image (JRE only)
FROM eclipse-temurin:21-jdk

WORKDIR /app

# Copy the built jar from the builder stage
COPY --from=builder /app/build/libs/*.jar app.jar

ENTRYPOINT ["java", "-jar", "app.jar"]