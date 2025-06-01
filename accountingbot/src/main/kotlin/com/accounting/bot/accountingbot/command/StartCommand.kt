package com.accounting.bot.accountingbot.command

import org.springframework.stereotype.Component
import org.telegram.telegrambots.meta.api.objects.Update

@Component
class StartCommand : BotCommand {

    override fun supports(text: String) = text.startsWith("/start", ignoreCase = true)

    override fun handle(update: Update): String {
        val user = update.message?.from
        val username = user?.userName ?: "пользователь"  // на случай, если username null
        return "Привет, $username! Добро пожаловать в AccountingBot ✨"
    }

    override fun getDescription(): String = "/start – старт"
}