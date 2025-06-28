package com.accounting.bot.accountingbot

import org.springframework.stereotype.Component
import org.telegram.telegrambots.meta.api.methods.send.SendMessage
import org.telegram.telegrambots.meta.api.objects.replykeyboard.InlineKeyboardMarkup
import org.telegram.telegrambots.meta.generics.TelegramClient


@Component
class MessageSenderImpl(
    private val telegramClient: TelegramClient,
): MessageSender {

    override fun sendMessageWithKeyboard(chatId: Long, text: String, keyboard: InlineKeyboardMarkup) {
        val message = SendMessage(chatId.toString(), text)
        message.replyMarkup = keyboard
        send(message)
    }

    override fun sendMessage(chatId: Long, text: String) {
        send(SendMessage(chatId.toString(), text))
    }

    override fun send(sendMessage: SendMessage) {
        try {
            telegramClient.execute(sendMessage)
        } catch (e: Exception) {
            println("Ошибка при отправке сообщения: ${e.message}")
            e.printStackTrace()
        }
    }

}