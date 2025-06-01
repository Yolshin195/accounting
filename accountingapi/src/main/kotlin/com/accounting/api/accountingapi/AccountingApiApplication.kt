package com.accounting.api.accountingapi

import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication
import org.springframework.data.jpa.repository.config.EnableJpaAuditing

@EnableJpaAuditing
@SpringBootApplication
class AccountingApiApplication

fun main(args: Array<String>) {
	runApplication<AccountingApiApplication>(*args)
}
