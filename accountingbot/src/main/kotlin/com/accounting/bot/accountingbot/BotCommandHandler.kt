package com.accounting.bot.accountingbot

import com.accounting.bot.accountingbot.command.BotCommand
import com.accounting.bot.accountingbot.command.StatefulCommand
import org.springframework.stereotype.Component
import org.telegram.telegrambots.meta.api.objects.Update

@Component
class BotCommandHandler(
    private val commands: List<BotCommand>,
) {
    fun handle(update: Update): String {
        // Сначала проверить, есть ли команда, которая ведёт активную сессию для пользователя
        val userId = update.message.from.id

        val sessionCommand = commands.find { it is StatefulCommand && it.hasSessionFor(userId) }
        if (sessionCommand != null) {
            return sessionCommand.handle(update)
        }

        val text = update.message.text
        val command = commands.find { it.supports(text) }
        return command?.handle(update) ?: "❓ Неизвестная команда. Введите /help для списка команд."
    }
}