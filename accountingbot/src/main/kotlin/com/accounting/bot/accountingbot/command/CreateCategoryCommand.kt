package com.accounting.bot.accountingbot.command

import com.accounting.bot.accountingbot.MessageSender
import com.accounting.bot.accountingbot.common.api.AuthClient
import com.accounting.bot.accountingbot.common.api.CategoryClient
import org.springframework.beans.factory.annotation.Value
import org.springframework.stereotype.Component
import org.telegram.telegrambots.meta.api.objects.Update
import org.telegram.telegrambots.meta.api.objects.replykeyboard.InlineKeyboardMarkup
import org.telegram.telegrambots.meta.api.objects.replykeyboard.buttons.InlineKeyboardButton
import org.telegram.telegrambots.meta.api.objects.replykeyboard.buttons.InlineKeyboardRow

@Component
class CreateCategoryCommand(
    private val authClient: AuthClient,
    private val categoryClient: CategoryClient,
    private val messageSender: MessageSender,
    @Value("\${accounting.accountingbot.username}") private val botUsername: String,
    @Value("\${accounting.accountingbot.password}") private val botPassword: String
) : BotCommand, StatefulCommand {

    data class CategoryCreationSession(
        var code: String? = null,
        var name: String? = null,
        var description: String? = null,
        var type: CategoryClient.CategoryType = CategoryClient.CategoryType.EXPENSE
    )

    private val sessions = mutableMapOf<Long, CategoryCreationSession>()
    private val states = mutableMapOf<Long, String>() // код, name, desc, type

    override fun hasSessionFor(userId: Long): Boolean = sessions.containsKey(userId)

    override fun supports(text: String): Boolean = text.startsWith("/create_category", ignoreCase = true)

    override fun handle(update: Update): String {
        val user = update.message?.from ?: return "Could not identify the user"
        val userId = user.id
        val text = update.message.text.trim()

        // Начало с команды
        if (text == "/create_category") {
            sessions[userId] = CategoryCreationSession()
            states[userId] = "code"
            return "Please enter the category code (e.g., FOOD):"
        }

        val session = sessions[userId] ?: return ""
        val state = states[userId] ?: return ""

        when (state) {
            "code" -> {
                session.code = text
                states[userId] = "name"
                return "Please enter the category name (e.g., 🍔 Food):"
            }
            "name" -> {
                session.name = text
                states[userId] = "desc"
                return "Please enter the category description:"
            }
            "desc" -> {
                session.description = text
                states[userId] = "waiting_type"
                messageSender.sendMessageWithKeyboard(update.message.chatId, "Choose category type:", createTypeKeyboard())
                return ""
            }
        }

        return "❓ Unexpected error. Please try again with /create_category"
    }

    fun handleCallback(update: Update): String? {
        val callback = update.callbackQuery ?: return null
        val userId = callback.from.id
        val session = sessions[userId] ?: return null

        val data = callback.data
        if (data == "CATEGORY_TYPE_EXPENSE" || data == "CATEGORY_TYPE_INCOME") {
            session.type = if (data.endsWith("EXPENSE")) CategoryClient.CategoryType.EXPENSE else CategoryClient.CategoryType.INCOME

            val jwt = authClient.loginTelegram(
                AuthClient.LoginTelegramBotDto(
                    clientId = botUsername,
                    secret = botPassword,
                    telegramId = userId,
                    username = callback.from.userName ?: "unknown"
                )
            )

            return try {
                val created = categoryClient.createCategory(
                    CategoryClient.CreateCategoryDto(
                        code = session.code!!,
                        name = session.name!!,
                        description = session.description,
                        type = session.type
                    ),
                    jwt.token
                )

                "✅ Category created: ${created.name} (${created.code}) [${created.type}]"
            } catch (e: Exception) {
                "❌ Error creating category: ${e.message}"
            } finally {
                sessions.remove(userId)
                states.remove(userId)
            }
        }

        return "❌ Unknown selection"
    }

    private fun createTypeKeyboard(): InlineKeyboardMarkup {
        val expenseButton = InlineKeyboardButton.builder()
            .text("💸 Расход")
            .callbackData("CATEGORY_TYPE_EXPENSE")
            .build()

        val incomeButton = InlineKeyboardButton.builder()
            .text("💰 Доход")
            .callbackData("CATEGORY_TYPE_INCOME")
            .build()

        val row = InlineKeyboardRow(expenseButton, incomeButton)

        return InlineKeyboardMarkup.builder()
            .keyboardRow(row)
            .build()
    }

    override fun getDescription(): String = "step-by-step category creation"
    override fun getCommandName(): String = "/create_category"
}