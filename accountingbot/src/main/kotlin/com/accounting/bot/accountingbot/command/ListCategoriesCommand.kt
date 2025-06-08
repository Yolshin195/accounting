package com.accounting.bot.accountingbot.command

import com.accounting.bot.accountingbot.MessageSender
import com.accounting.bot.accountingbot.common.api.AuthClient
import com.accounting.bot.accountingbot.common.api.CategoryClient
import org.springframework.beans.factory.annotation.Value
import org.springframework.stereotype.Component
import org.telegram.telegrambots.meta.api.objects.Update

@Component
class ListCategoriesCommand(
    private val authClient: AuthClient,
    private val categoryClient: CategoryClient,
    private val messageSender: MessageSender,
    @Value("\${accounting.accountingbot.username}") private val botUsername: String,
    @Value("\${accounting.accountingbot.password}") private val botPassword: String,
) : BotCommand {

    override fun supports(text: String): Boolean = text.startsWith("/list_categories", ignoreCase = true)

    override fun handle(update: Update) {
        val user = update.message?.from ?: return
        val chatId = update.message?.chatId ?: return
        val jwt = authClient.loginTelegram(AuthClient.LoginTelegramBotDto(
            clientId = botUsername,
            secret = botPassword,
            telegramId = user.id,
            username = user.userName ?: "unknown"
        ))

        val page = 0
        val size = 5
        return try {
            val categories = categoryClient.getAllCategories(page, size, jwt.token)
            if (categories.content.isEmpty()) return messageSender.sendMessage(chatId,"❗ No categories found.")

            val result = categories.content.joinToString("\n") {
                "🔹 ${it.name} (${it.code}) – ${it.type}"
            }

            messageSender.sendMessage(chatId,"📦 Categories (page ${categories.page.number + 1}/${categories.page.totalPages}):\n$result")
        } catch (e: Exception) {
            e.printStackTrace()
            messageSender.sendMessage(chatId,"❌ Error retrieving categories: ${e.message}")
        }
    }

    override fun getDescription(): String = "list of categories"
    override fun getCommandName(): String = "/list_categories"
}