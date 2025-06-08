package com.accounting.bot.accountingbot.command

import com.accounting.bot.accountingbot.MessageSender
import com.accounting.bot.accountingbot.common.api.AuthClient
import com.accounting.bot.accountingbot.common.api.TransactionClient
import org.springframework.beans.factory.annotation.Value
import org.springframework.stereotype.Component
import org.telegram.telegrambots.meta.api.objects.Update

@Component
class TodayExpensesByCategoryCommand(
    private val authClient: AuthClient,
    private val transactionClient: TransactionClient,
    private val messageSender: MessageSender,
    @Value("\${accounting.accountingbot.username}") private val botUsername: String,
    @Value("\${accounting.accountingbot.password}") private val botPassword: String
) : BotCommand {

    override fun supports(text: String): Boolean = text.equals("/today_expenses", ignoreCase = true)

    override fun handle(update: Update) {
        val chatId = update.message?.chatId ?: return
        val user = update.message?.from ?: return
        val userId = user.id

        val jwt = authClient.loginTelegram(
            AuthClient.LoginTelegramBotDto(
                clientId = botUsername,
                secret = botPassword,
                telegramId = userId,
                username = user.userName ?: "unknown"
            )
        )

        return try {
            val expenses = transactionClient.getTodayExpensesByCategory(jwt.token)
            if (expenses.isEmpty()) {
                messageSender.sendMessage(chatId, "Сегодня расходов нет.")
            } else {
                val lines = expenses.map { "• ${it.categoryCode}: ${"%.2f".format(it.totalAmount)}" }
                messageSender.sendMessage(chatId, "Сегодняшние расходы по категориям:\n" + lines.joinToString("\n"))
            }
        } catch (e: Exception) {
            messageSender.sendMessage(chatId, "❌ Ошибка при получении данных: ${e.message}")
        }
    }

    override fun getDescription(): String = "показать сегодняшние расходы по категориям"
    override fun getCommandName(): String = "/today_expenses"
}