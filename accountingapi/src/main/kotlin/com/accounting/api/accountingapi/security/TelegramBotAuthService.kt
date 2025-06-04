package com.accounting.api.accountingapi.security

import com.accounting.api.accountingapi.common.dto.LoginTelegramBotDto
import com.accounting.api.accountingapi.common.dto.UserRoleEnum
import com.accounting.api.accountingapi.common.entity.UserProfileEntity
import com.accounting.api.accountingapi.repository.UserProfileRepository
import org.springframework.security.authentication.AuthenticationManager
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken
import org.springframework.security.core.userdetails.UserDetailsService
import org.springframework.security.crypto.password.PasswordEncoder
import org.springframework.stereotype.Service
import java.util.*

@Service
class TelegramBotAuthService(
    private val userProfileRepository: UserProfileRepository,
    private val passwordEncoder: PasswordEncoder,
    private val authenticationManager: AuthenticationManager,
    private val userDetailsService: UserDetailsService,
    private val jwtUtil: JwtUtil
) {

    fun authenticateTelegramBot(request: LoginTelegramBotDto): String {
        // 1. Авторизуем бота
        val botUser = authenticateBot(request.clientId, request.secret)

        // 2. Проверяем роль бота
        validateBotRole(botUser)

        // 3. Получаем или создаем пользователя
        val targetUser = getOrCreateTelegramUser(request)

        // 4. Генерируем токен для пользователя
        return generateTokenForUser(targetUser)
    }

    private fun authenticateBot(clientId: String, secret: String): UserProfileEntity {
        authenticationManager.authenticate(
            UsernamePasswordAuthenticationToken(clientId, secret)
        )

        return userProfileRepository.findByUsername(clientId)
            ?: throw IllegalArgumentException("Bot user not found")
    }

    private fun validateBotRole(botUser: UserProfileEntity) {
        if (!botUser.roles.contains(UserRoleEnum.ROLE_TELEGRAM_BOT)) {
            throw SecurityException("User does not have TELEGRAM_BOT role")
        }
    }

    private fun getOrCreateTelegramUser(request: LoginTelegramBotDto): UserProfileEntity {
        return userProfileRepository.findByTelegramId(request.telegramId)
            ?: createTelegramUser(request)
    }

    private fun createTelegramUser(request: LoginTelegramBotDto): UserProfileEntity {
        val newUser = UserProfileEntity(
            username = generateUniqueUsername(request.username),
            telegramId = request.telegramId,
            hashPassword = passwordEncoder.encode(generateRandomPassword()),
            roles = setOf(UserRoleEnum.ROLE_USER),
            enabled = true,
            accountNonExpired = true,
            credentialsNonExpired = true,
            accountNonLocked = true
        )

        return userProfileRepository.save(newUser).also {
            println("Created new telegram user: ${it.username} with telegramId: ${request.telegramId}")
        }
    }

    private fun generateUniqueUsername(baseUsername: String): String {
        var username = baseUsername
        var counter = 1

        while (userProfileRepository.findByUsername(username) != null) {
            username = "${baseUsername}_$counter"
            counter++
        }

        return username
    }

    private fun generateRandomPassword(): String {
        return UUID.randomUUID().toString().replace("-", "").substring(0, 16)
    }

    private fun generateTokenForUser(user: UserProfileEntity): String {
        val userDetails = userDetailsService.loadUserByUsername(user.username)
        return jwtUtil.generateToken(userDetails)
    }
}