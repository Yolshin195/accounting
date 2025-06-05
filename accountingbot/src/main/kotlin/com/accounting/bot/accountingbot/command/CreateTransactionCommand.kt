package com.accounting.bot.accountingbot.command

import com.accounting.bot.accountingbot.common.api.AuthClient
import com.accounting.bot.accountingbot.common.api.TransactionClient
import org.springframework.beans.factory.annotation.Value
import org.springframework.stereotype.Component
import org.telegram.telegrambots.meta.api.objects.Update

@Component
class CreateTransactionCommand(
    private val authClient: AuthClient,
    private val transactionClient: TransactionClient,
    @Value("\${accounting.accountingbot.username}") private val botUsername: String,
    @Value("\${accounting.accountingbot.password}") private val botPassword: String
) : BotCommand, StatefulCommand {

    data class TransactionCreationSession(
        var type: TransactionClient.TransactionType? = null,
        var amount: Double? = null,
        var categoryCode: String? = null,
        var description: String? = null
    )

    private val sessions = mutableMapOf<Long, TransactionCreationSession>()
    private val states = mutableMapOf<Long, String>() // type, amount, category, description

    override fun hasSessionFor(userId: Long): Boolean = sessions.containsKey(userId)

    override fun supports(text: String): Boolean = text.startsWith("/create_transaction", ignoreCase = true)

    override fun handle(update: Update): String {
        val user = update.message?.from ?: return "Не удалось определить пользователя"
        val userId = user.id
        val text = update.message.text.trim()

        // Начало диалога
        if (text.equals("/create_transaction", ignoreCase = true)) {
            sessions[userId] = TransactionCreationSession()
            states[userId] = "type"
            return "Введите тип транзакции (EXPENSE или INCOME):"
        }

        val session = sessions[userId] ?: return ""
        val state = states[userId] ?: return ""

        when (state) {
            "type" -> {
                session.type = try {
                    TransactionClient.TransactionType.valueOf(text.uppercase())
                } catch (_: Exception) {
                    return "Неверный тип. Введите EXPENSE или INCOME:"
                }
                states[userId] = "amount"
                return "Введите сумму:"
            }
            "amount" -> {
                val amount = text.toDoubleOrNull()
                if (amount == null || amount <= 0) {
                    return "Неверная сумма. Введите положительное число:"
                }
                session.amount = amount
                states[userId] = "category"
                return "Введите код категории (например, FOOD):"
            }
            "category" -> {
                session.categoryCode = text
                states[userId] = "description"
                return "Введите описание (можно оставить пустым):"
            }
            "description" -> {
                session.description = if (text.isBlank()) null else text

                val jwt = authClient.loginTelegram(
                    AuthClient.LoginTelegramBotDto(
                        clientId = botUsername,
                        secret = botPassword,
                        telegramId = userId,
                        username = user.userName ?: "unknown"
                    )
                )

                try {
                    val created = when (session.type!!) {
                        TransactionClient.TransactionType.EXPENSE -> transactionClient.createExpense(
                            TransactionClient.CreateExpenseDto(
                                amount = session.amount!!,
                                category = session.categoryCode!!,
                                description = session.description
                            ),
                            jwt.token
                        )
                        TransactionClient.TransactionType.INCOME -> transactionClient.createIncome(
                            TransactionClient.CreateIncomeDto(
                                amount = session.amount!!,
                                category = session.categoryCode!!,
                                description = session.description
                            ),
                            jwt.token
                        )
                    }

                    // Очистка сессии
                    sessions.remove(userId)
                    states.remove(userId)

                    return "✅ Транзакция создана: ${created.type} ${created.amount} в категории ${created.category}"
                } catch (e: Exception) {
                    return "❌ Ошибка при создании транзакции: ${e.message}"
                }
            }
        }

        return "❓ Неожиданная ошибка. Попробуйте снова /create_transaction"
    }

    override fun getDescription(): String = "пошаговое создание транзакции"
    override fun getCommandName(): String = "/create_transaction"
}