package com.accounting.bot.accountingbot.command.transaction

import com.accounting.bot.accountingbot.common.api.AuthClient
import com.accounting.bot.accountingbot.common.api.TransactionClient
import org.springframework.beans.factory.annotation.Value
import org.springframework.stereotype.Component

@Component
class CreateIncomeCommand(
    authClient: AuthClient,
    transactionClient: TransactionClient,
    @Value("\${accounting.accountingbot.username}") botUsername: String,
    @Value("\${accounting.accountingbot.password}") botPassword: String
) : AbstractTransactionCommand(authClient, transactionClient, botUsername, botPassword) {

    override fun getTransactionType() = TransactionClient.TransactionType.INCOME
    override fun getStartCommand() = "/create_income"
    override fun getStartPrompt() = "Введите сумму дохода:"
    override fun getDescription() = "пошаговое создание дохода"
    override fun getCommandName() = "/create_income"
}