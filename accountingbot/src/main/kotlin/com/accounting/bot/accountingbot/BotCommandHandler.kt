package com.accounting.bot.accountingbot

import com.accounting.bot.accountingbot.command.BotCommand
import com.accounting.bot.accountingbot.command.CreateCategoryCommand
import com.accounting.bot.accountingbot.command.StatefulCommand
import org.springframework.stereotype.Component
import org.telegram.telegrambots.meta.api.objects.Update

@Component
class BotCommandHandler(
    private val commands: List<BotCommand>,
    private val messageSender: MessageSender,
) {
    fun handle(update: Update) {
        // Сначала проверить, есть ли команда, которая ведёт активную сессию для пользователя
        val userId = update.message?.from?.id ?: update.callbackQuery?.from?.id ?: return

        if (update.hasCallbackQuery()) {
            val sessionCommand = commands.find { it is StatefulCommand && it.hasSessionFor(userId) }
            if (sessionCommand is CreateCategoryCommand) {
                return sessionCommand.handleCallback(update)
            }
        }

        val sessionCommand = commands.find { it is StatefulCommand && it.hasSessionFor(userId) }
        if (sessionCommand != null) {
            return sessionCommand.handle(update)
        }

        val chatId = update.message?.chatId ?: return
        val text = update.message.text
        val command = commands.find { it.supports(text) }
        return command?.handle(update) ?: messageSender.sendMessage(chatId, "❓ Неизвестная команда. Введите /help для списка команд.")
    }
}