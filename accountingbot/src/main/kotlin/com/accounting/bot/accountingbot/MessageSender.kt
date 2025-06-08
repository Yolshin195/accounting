package com.accounting.bot.accountingbot

import org.telegram.telegrambots.meta.api.objects.replykeyboard.InlineKeyboardMarkup

interface MessageSender {
    fun sendMessageWithKeyboard(chatId: Long, text: String, keyboard: InlineKeyboardMarkup)
    fun sendMessage(chatId: Long, text: String)
}