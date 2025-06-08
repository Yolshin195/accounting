package com.accounting.bot.accountingbot.command.transaction

import com.accounting.bot.accountingbot.MessageSender
import com.accounting.bot.accountingbot.command.BotCommand
import com.accounting.bot.accountingbot.command.StatefulCommand
import com.accounting.bot.accountingbot.common.api.AuthClient
import com.accounting.bot.accountingbot.common.api.TransactionClient
import org.telegram.telegrambots.meta.api.objects.Update

abstract class AbstractTransactionCommand(
    private val authClient: AuthClient,
    private val transactionClient: TransactionClient,
    private val messageSender: MessageSender,
    private val botUsername: String,
    private val botPassword: String,
) : BotCommand, StatefulCommand {

    data class TransactionCreationSession(
        var amount: Double? = null,
        var categoryCode: String? = null,
        var description: String? = null
    )

    private val sessions = mutableMapOf<Long, TransactionCreationSession>()
    private val states = mutableMapOf<Long, String>() // amount, category, description

    abstract fun getTransactionType(): TransactionClient.TransactionType
    abstract fun getStartCommand(): String
    abstract fun getStartPrompt(): String

    override fun hasSessionFor(userId: Long): Boolean = sessions.containsKey(userId)

    override fun supports(text: String): Boolean = text.startsWith(getStartCommand(), ignoreCase = true)

    override fun handle(update: Update) {
        val chatId = update.message?.chatId ?: return
        val user = update.message?.from ?: return
        val userId = user.id
        val text = update.message.text.trim()

        if (text.equals(getStartCommand(), ignoreCase = true)) {
            sessions[userId] = TransactionCreationSession()
            states[userId] = "amount"
            return messageSender.sendMessage(chatId, getStartPrompt())
        }

        val session = sessions[userId] ?: return
        val state = states[userId] ?: return

        when (state) {
            "amount" -> {
                val amount = text.toDoubleOrNull()
                if (amount == null || amount <= 0) {
                    return messageSender.sendMessage(chatId, "Неверная сумма. Введите положительное число:")
                }
                session.amount = amount
                states[userId] = "category"
                return messageSender.sendMessage(chatId, "Введите код категории (например, FOOD):")
            }
            "category" -> {
                session.categoryCode = text
                states[userId] = "description"
                return messageSender.sendMessage(chatId, "Введите описание (можно оставить пустым):")
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

                return try {
                    val created = when (getTransactionType()) {
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

                    messageSender.sendMessage(chatId, "✅ Транзакция создана: ${created.type} ${created.amount} в категории ${created.category}")
                } catch (e: Exception) {
                    messageSender.sendMessage(chatId, "❌ Ошибка при создании транзакции: ${e.message}")
                } finally {
                    // Очистка в любом случае
                    sessions.remove(userId)
                    states.remove(userId)
                }
            }
        }

        return messageSender.sendMessage(chatId, "❓ Неожиданная ошибка. Попробуйте снова ${getStartCommand()}")
    }
}