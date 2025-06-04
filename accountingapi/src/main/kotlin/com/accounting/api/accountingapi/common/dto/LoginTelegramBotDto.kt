package com.accounting.api.accountingapi.common.dto

data class LoginTelegramBotDto(
    val clientId: String,
    val secret: String,
    val telegramId: Long,
    val username: String,
)
