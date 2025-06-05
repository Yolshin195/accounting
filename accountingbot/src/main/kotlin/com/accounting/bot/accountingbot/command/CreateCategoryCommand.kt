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
        val user = update.message?.from ?: return "Не удалось определить пользователя"
        val userId = user.id
        val text = update.message.text.trim()

        // Начало с команды
        if (text == "/create_category") {
            sessions[userId] = CategoryCreationSession()
            states[userId] = "code"
            return "Введите код категории (например, FOOD):"
        }

        val session = sessions[userId] ?: return ""
        val state = states[userId] ?: return ""

        when (state) {
            "code" -> {
                session.code = text
                states[userId] = "name"
                return "Введите название категории (например, 🍔 Еда):"
            }
            "name" -> {
                session.name = text
                states[userId] = "desc"
                return "Введите описание категории:"
            }
            "desc" -> {
                session.description = text
                states[userId] = "type"
                return "Введите тип (EXPENSE или INCOME, по умолчанию EXPENSE):"
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

                    "✅ Категория создана: ${created.name} (${created.code}) [${created.type}]"
                } catch (e: Exception) {
                    "❌ Ошибка при создании категории: ${e.message}"
                }
            }
        }

        return "❓ Неожиданная ошибка. Попробуйте снова /create_category"
    }

    override fun getDescription(): String = "пошаговое создание категории"
    override fun getCommandName(): String = "/create_category"
}