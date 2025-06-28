package com.accounting.bot.accountingbot.command.category

import com.accounting.bot.accountingbot.MessageSender
import com.accounting.bot.accountingbot.common.api.AuthClient
import com.accounting.bot.accountingbot.common.api.CategoryClient
import org.springframework.beans.factory.annotation.Value
import org.springframework.stereotype.Component

@Component
class ListAllCategoriesCommand(
    authClient: AuthClient,
    categoryClient: CategoryClient,
    messageSender: MessageSender,
    @Value("\${accounting.accountingbot.username}") botUsername: String,
    @Value("\${accounting.accountingbot.password}") botPassword: String,
) : AbstractListCategoriesCommand(authClient, categoryClient, messageSender, botUsername, botPassword) {

    override fun supports(text: String) = text.startsWith("/list_categories", ignoreCase = true)
    override fun getDescription() = "List all categories"
    override fun getCommandName() = "/list_categories"

    override fun fetchPage(token: String, page: Int, size: Int): CategoryClient.PageCategoryDto =
        categoryClient.getAllCategories(page, size, token)
}