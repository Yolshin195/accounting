package com.accounting.bot.accountingbot.command.transaction

import com.accounting.bot.accountingbot.MessageSender
import com.accounting.bot.accountingbot.common.api.AuthClient
import com.accounting.bot.accountingbot.common.api.TransactionClient
import org.springframework.beans.factory.annotation.Value
import org.springframework.stereotype.Component

@Component
class CreateExpenseCommand(
    authClient: AuthClient,
    transactionClient: TransactionClient,
    messageSender: MessageSender,
    @Value("\${accounting.accountingbot.username}") botUsername: String,
    @Value("\${accounting.accountingbot.password}") botPassword: String
) : AbstractTransactionCommand(authClient, transactionClient, messageSender, botUsername, botPassword) {

    override fun getTransactionType() = TransactionClient.TransactionType.EXPENSE
    override fun getStartCommand() = "/create_expense"
    override fun getStartPrompt() = "Введите сумму расхода:"
    override fun getDescription() = "пошаговое создание расхода"
    override fun getCommandName() = "/create_expense"
}