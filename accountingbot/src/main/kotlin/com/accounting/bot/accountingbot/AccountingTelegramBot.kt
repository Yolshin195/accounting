package com.accounting.bot.accountingbot


import org.springframework.stereotype.Component
import org.springframework.beans.factory.annotation.Value
import org.telegram.telegrambots.meta.api.objects.Update
import org.telegram.telegrambots.longpolling.interfaces.LongPollingUpdateConsumer
import org.telegram.telegrambots.longpolling.starter.SpringLongPollingBot
import org.telegram.telegrambots.longpolling.util.LongPollingSingleThreadUpdateConsumer


@Component
class AccountingTelegramBot (
    private val botCommandHandler: BotCommandHandler,
    @Value("\${telegram.bot.token}") private val botToken: String
) : SpringLongPollingBot, LongPollingSingleThreadUpdateConsumer {

    override fun getBotToken(): String = botToken
    override fun getUpdatesConsumer(): LongPollingUpdateConsumer = this

    override fun consume(update: Update?) {
        if (update == null) return

        if (update.hasCallbackQuery()) {
            return botCommandHandler.handle(update)
        }

        if (!update.hasMessage() || !update.message.hasText()) return

        return botCommandHandler.handle(update)
    }
}
