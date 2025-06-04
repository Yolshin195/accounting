package com.accounting.bot.accountingbot.common.api

import org.springframework.beans.factory.annotation.Value
import org.springframework.stereotype.Service
import org.springframework.web.client.RestTemplate

// DTO классы должны быть объявлены отдельно (JwtResponse, CreateUserDto, и т.д.)

@Service
class AuthClient(
    private val restTemplate: RestTemplate,
    @Value("\${api.base-url}") private val baseUrl: String
) {

    data class JwtResponse(val token: String)

    data class LoginTelegramBotDto(
        val clientId: String,
        val secret: String,
        val telegramId: Long,
        val username: String
    )

    fun loginTelegram(dto: LoginTelegramBotDto): JwtResponse {
        val url = "$baseUrl/users/login/telegram"
        val response = restTemplate.postForEntity(url, dto, JwtResponse::class.java)
        return response.body ?: throw RuntimeException("Telegram login failed")
    }
}