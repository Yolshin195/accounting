package com.accounting.bot.accountingbot.command

import com.accounting.bot.accountingbot.common.api.AuthClient
import org.springframework.beans.factory.annotation.Value
import org.springframework.stereotype.Component
import org.telegram.telegrambots.meta.api.objects.Update

@Component
class StartCommand(
    var authClient: AuthClient,
    @Value("\${accounting.accountingbot.username}") private val botUsername: String,
    @Value("\${accounting.accountingbot.password}") private val botPassword: String,
) : BotCommand {

    override fun supports(text: String) = text.startsWith("/start", ignoreCase = true)

    override fun handle(update: Update): String {
        val user = update.message?.from
        val username = user?.userName ?: "пользователь"  // на случай, если username null
        val telegramId = user?.id ?: throw IllegalArgumentException("telegramId is null")
        val jwt = authClient.loginTelegram(AuthClient.LoginTelegramBotDto(
            clientId = botUsername,
            secret = botPassword,
            telegramId = telegramId,
            username = username
        ))
        println(jwt.token)
        return "Привет, $username! Добро пожаловать в AccountingBot ✨"
    }

    override fun getDescription(): String = "старт"
    override fun getCommandName(): String = "/start"
}