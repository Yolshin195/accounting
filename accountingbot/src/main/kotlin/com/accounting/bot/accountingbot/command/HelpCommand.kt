package com.accounting.bot.accountingbot.command

import org.springframework.stereotype.Component
import org.telegram.telegrambots.meta.api.objects.Update


@Component
class HelpCommand(
    private val commands: List<BotCommand> // Внедряются все команды
) : BotCommand {
    override fun supports(text: String): Boolean = text.startsWith("/help", ignoreCase = true)

    override fun handle(update: Update): String {
        return buildString {
            appendLine("📋 Доступные команды:")
            commands.forEach {
                appendLine("${it.getCommandName()} - ${it.getDescription()}")
            }
        }
    }

    override fun getDescription(): String = "показать список всех команд"
    override fun getCommandName(): String = "/help"
}