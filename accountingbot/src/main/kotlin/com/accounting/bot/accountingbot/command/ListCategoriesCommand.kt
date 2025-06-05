package com.accounting.bot.accountingbot.command

import com.accounting.bot.accountingbot.common.api.AuthClient
import com.accounting.bot.accountingbot.common.api.CategoryClient
import org.springframework.beans.factory.annotation.Value
import org.springframework.stereotype.Component
import org.telegram.telegrambots.meta.api.objects.Update

@Component
class ListCategoriesCommand(
    private val authClient: AuthClient,
    private val categoryClient: CategoryClient,
    @Value("\${accounting.accountingbot.username}") private val botUsername: String,
    @Value("\${accounting.accountingbot.password}") private val botPassword: String,
) : BotCommand {

    override fun supports(text: String): Boolean = text.startsWith("/list_categories", ignoreCase = true)

    override fun handle(update: Update): String {
        val user = update.message?.from ?: return "Failed to identify the user"
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
            if (categories.content.isEmpty()) return "❗ No categories found."

            val result = categories.content.joinToString("\n") {
                "🔹 ${it.name} (${it.code}) – ${it.type}"
            }

            "📦 Categories (page ${categories.number + 1}/${categories.totalPages}):\n$result"
        } catch (e: Exception) {
            "❌ Error retrieving categories: ${e.message}"
        }
    }

    override fun getDescription(): String = "list of categories"
    override fun getCommandName(): String = "/list_categories"
}