package com.accounting.bot.accountingbot


import org.springframework.stereotype.Component
import org.springframework.beans.factory.annotation.Value
import org.telegram.telegrambots.meta.api.objects.Update
import org.telegram.telegrambots.longpolling.interfaces.LongPollingUpdateConsumer
import org.telegram.telegrambots.longpolling.starter.SpringLongPollingBot
import org.telegram.telegrambots.longpolling.util.LongPollingSingleThreadUpdateConsumer
import org.telegram.telegrambots.meta.api.methods.send.SendMessage
import org.telegram.telegrambots.meta.generics.TelegramClient


@Component
class AccountingTelegramBot (
    private val telegramClient: TelegramClient,
    private val botCommandHandler: BotCommandHandler,
    @Value("\${telegram.bot.token}") private val botToken: String
) : SpringLongPollingBot, LongPollingSingleThreadUpdateConsumer {

    override fun getBotToken(): String = botToken
    override fun getUpdatesConsumer(): LongPollingUpdateConsumer = this

    override fun consume(update: Update?) {
        if (update == null) return

        if (update.hasCallbackQuery()) {
            val responseText = botCommandHandler.handle(update)
            println("responseText= $responseText")
            if (responseText.isNotEmpty()) {
                sendMessage(update.callbackQuery.message.chatId, responseText)
            }
            return
        }

        if (!update.hasMessage() || !update.message.hasText()) return

        val responseText = botCommandHandler.handle(update)
        println("responseText= $responseText")
        if (responseText.isNotEmpty()) {
            sendMessage(update.message.chatId, responseText)
        }
    }

    private fun sendMessage(chatId: Long, text: String) {
        try {
            val message = SendMessage(chatId.toString(), text)
            telegramClient.execute(message)
        } catch (e: Exception) {
            println("Ошибка при отправке сообщения: ${e.message}")
            e.printStackTrace()
        }
    }
}
