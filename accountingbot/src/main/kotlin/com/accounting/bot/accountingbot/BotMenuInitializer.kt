package com.accounting.bot.accountingbot

import com.accounting.bot.accountingbot.command.BotCommand
import jakarta.annotation.PostConstruct
import org.springframework.stereotype.Component
import org.telegram.telegrambots.meta.api.methods.commands.SetMyCommands
import org.telegram.telegrambots.meta.api.objects.commands.BotCommand as TgBotCommand
import org.telegram.telegrambots.meta.api.objects.commands.scope.BotCommandScopeDefault
import org.telegram.telegrambots.meta.generics.TelegramClient

@Component
class BotMenuInitializer(
    private val telegramClient: TelegramClient,
    private val botCommands: List<BotCommand> // внедряются все команды
) {
    @PostConstruct
    fun initMenu() {
        val tgCommands = botCommands.map {
            TgBotCommand(it.getCommandName(), it.getDescription())
        }

        val setMyCommands = SetMyCommands(tgCommands, BotCommandScopeDefault(), null)
        telegramClient.execute(setMyCommands)
    }
}