package com.accounting.bot.accountingbot

import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication

@SpringBootApplication
class AccountingBotApplication

fun main(args: Array<String>) {
	runApplication<AccountingBotApplication>(*args)
}
