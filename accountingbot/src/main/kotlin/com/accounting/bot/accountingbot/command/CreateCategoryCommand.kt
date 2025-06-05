package com.accounting.bot.accountingbot.command

import com.accounting.bot.accountingbot.common.api.AuthClient
import com.accounting.bot.accountingbot.common.api.CategoryClient
import org.springframework.beans.factory.annotation.Value
import org.springframework.stereotype.Component
import org.telegram.telegrambots.meta.api.objects.Update

@Component
class CreateCategoryCommand(
    private val authClient: AuthClient,
    private val categoryClient: CategoryClient,
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
                states[userId] = "type"
                return "Please enter the type (EXPENSE or INCOME, default is EXPENSE):"
            }
            "type" -> {
                session.type = try {
                    CategoryClient.CategoryType.valueOf(text.uppercase())
                } catch (_: Exception) {
                    CategoryClient.CategoryType.EXPENSE
                }

                // Всё собрано — отправляем
                val jwt = authClient.loginTelegram(
                    AuthClient.LoginTelegramBotDto(
                        clientId = botUsername,
                        secret = botPassword,
                        telegramId = userId,
                        username = user.userName ?: "unknown"
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

                    // Очищаем
                    sessions.remove(userId)
                    states.remove(userId)

                    "✅ Category created: ${created.name} (${created.code}) [${created.type}]"
                } catch (e: Exception) {
                    "❌ Error creating category: ${e.message}"
                }
            }
        }

        return "❓ Unexpected error. Please try again with /create_category"
    }

    override fun getDescription(): String = "step-by-step category creation"
    override fun getCommandName(): String = "/create_category"
}