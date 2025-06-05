package com.accounting.bot.accountingbot.command

interface StatefulCommand : BotCommand {
    fun hasSessionFor(userId: Long): Boolean
}