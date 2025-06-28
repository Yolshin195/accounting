package com.accounting.bot.accountingbot.command.category

import com.accounting.bot.accountingbot.MessageSender
import com.accounting.bot.accountingbot.command.BotCommand
import com.accounting.bot.accountingbot.common.api.AuthClient
import com.accounting.bot.accountingbot.common.api.CategoryClient
import com.accounting.bot.accountingbot.common.api.CategoryClient.PageCategoryDto
import org.springframework.beans.factory.annotation.Value
import org.telegram.telegrambots.meta.api.methods.send.SendMessage
import org.telegram.telegrambots.meta.api.objects.Update
import org.telegram.telegrambots.meta.api.objects.replykeyboard.InlineKeyboardMarkup
import org.telegram.telegrambots.meta.api.objects.replykeyboard.buttons.InlineKeyboardButton
import org.telegram.telegrambots.meta.api.objects.replykeyboard.buttons.InlineKeyboardRow

abstract class AbstractListCategoriesCommand(
    protected val authClient: AuthClient,
    protected val categoryClient: CategoryClient,
    protected val messageSender: MessageSender,
    @Value("\${accounting.accountingbot.username}") private val botUsername: String,
    @Value("\${accounting.accountingbot.password}") private val botPassword: String,
) : BotCommand {

    protected abstract fun fetchPage(token: String, page: Int, size: Int): PageCategoryDto

    override fun handle(update: Update) {
        val user = update.message?.from ?: return
        val chatId = update.message?.chatId ?: return

        val jwt = authClient.loginTelegram(
            AuthClient.LoginTelegramBotDto(botUsername, botPassword, user.id, user.userName ?: "unknown")
        )

        val page = 0
        val size = 5
        val categories = fetchPage(jwt.token, page, size)

        if (categories.content.isEmpty()) {
            messageSender.sendMessage(chatId, "❗ No categories found.")
            return
        }

        val keyboard = buildKeyboard(categories, page, size)
        val message = SendMessage.builder()
            .chatId(chatId.toString())
            .text("📦 Категории (страница ${categories.page.number + 1}/${categories.page.totalPages}):")
            .replyMarkup(keyboard)
            .build()

        messageSender.send(message)
    }

    private fun buildKeyboard(categories: PageCategoryDto, page: Int, size: Int): InlineKeyboardMarkup {
        val keyboard = mutableListOf<InlineKeyboardRow>()

        categories.content.chunked(2).forEach { chunk ->
            keyboard.add(InlineKeyboardRow(chunk.map {
                InlineKeyboardButton.builder()
                    .text("${it.name} (${it.type})")
                    .callbackData("category:${it.code}")
                    .build()
            }))
        }

        val navRow = InlineKeyboardRow()
        if (page > 0) {
            navRow.add(
                InlineKeyboardButton.builder()
                    .text("◀️ Пред.")
                    .callbackData("categories_page:${getTypeLabel()}:${page - 1}:$size")
                    .build()
            )
        }
        if (page + 1 < categories.page.totalPages) {
            navRow.add(
                InlineKeyboardButton.builder()
                    .text("▶️ След.")
                    .callbackData("categories_page:${getTypeLabel()}:${page + 1}:$size")
                    .build()
            )
        }
        if (navRow.isNotEmpty()) keyboard.add(navRow)

        return InlineKeyboardMarkup.builder().keyboard(keyboard).build()
    }

    protected open fun getTypeLabel(): String = "ALL"
}