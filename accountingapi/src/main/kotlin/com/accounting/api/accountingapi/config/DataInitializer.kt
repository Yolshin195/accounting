package com.accounting.api.accountingapi.config

import com.accounting.api.accountingapi.common.entity.UserProfileEntity
import com.accounting.api.accountingapi.common.dto.UserRoleEnum
import com.accounting.api.accountingapi.common.entity.CategoryEntity
import com.accounting.api.accountingapi.repository.CategoryRepository
import com.accounting.api.accountingapi.repository.UserProfileRepository
import org.springframework.beans.factory.annotation.Value
import org.springframework.boot.CommandLineRunner
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.security.crypto.password.PasswordEncoder


@Configuration
class DataInitializer(
    private val userProfileRepository: UserProfileRepository,
    private val passwordEncoder: PasswordEncoder,
    private val categoryRepository: CategoryRepository,
    @Value("\${accounting.accountingbot.username}") private val botUsername: String,
    @Value("\${accounting.accountingbot.password}") private val botPassword: String,
) {

    @Bean
    fun init(): CommandLineRunner = CommandLineRunner {
        val user = userProfileRepository.findByUsername("admin") ?: run {
            val newUser = UserProfileEntity(
                username = "admin",
                hashPassword = passwordEncoder.encode("admin"),
                roles = setOf(UserRoleEnum.ROLE_USER),
                enabled = true,
                accountNonExpired = true,
                credentialsNonExpired = true,
                accountNonLocked = true
            )
            userProfileRepository.save(newUser).also {
                println("Admin user created")
            }
        }

        val botUser = userProfileRepository.findByUsername(botUsername) ?: run {
            val newBotUser = UserProfileEntity(
                username = botUsername,
                hashPassword = passwordEncoder.encode(botPassword),
                roles = setOf(UserRoleEnum.ROLE_TELEGRAM_BOT),
                enabled = true,
                accountNonExpired = true,
                credentialsNonExpired = true,
                accountNonLocked = true
            )
            userProfileRepository.save(newBotUser).also {
                println("Bot user created: $botUsername")
            }
        }

        println("Using user: ${user.username}")

        // Создание категорий, если их нет
        val existingCategories = categoryRepository.findAll()
        if (existingCategories.isEmpty()) {
            val categories = listOf(
                CategoryEntity(code="food", name = "Еда", user = user, description = "Expenses on food and groceries"),
                CategoryEntity(code="Coffee", name = "Кофе", user = user, description = "Expenses on food and groceries"),
                // Добавь свои категории сюда
            )
            categoryRepository.saveAll(categories)
            println("Default categories created")
        } else {
            println("Categories already exist")
        }
    }
}