package com.accounting.bot.accountingbot.command.category

import com.accounting.bot.accountingbot.MessageSender
import com.accounting.bot.accountingbot.common.api.AuthClient
import com.accounting.bot.accountingbot.common.api.CategoryClient
import org.springframework.beans.factory.annotation.Value
import org.springframework.stereotype.Component

@Component
class ListExpenseCategoriesCommand(
    authClient: AuthClient,
    categoryClient: CategoryClient,
    messageSender: MessageSender,
    @Value("\${accounting.accountingbot.username}") botUsername: String,
    @Value("\${accounting.accountingbot.password}") botPassword: String,
) : AbstractListCategoriesCommand(authClient, categoryClient, messageSender, botUsername, botPassword) {

    override fun supports(text: String) = text.startsWith("/list_expenses", ignoreCase = true)
    override fun getDescription() = "List expense categories"
    override fun getCommandName() = "/list_expenses"

    override fun fetchPage(token: String, page: Int, size: Int): CategoryClient.PageCategoryDto =
        categoryClient.getExpenseCategories(page, size, token)

    override fun getTypeLabel(): String = "EXPENSE"
}