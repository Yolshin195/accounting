package com.accounting.api.accountingapi.common.dto.transaction

import java.math.BigDecimal
import java.util.*


data class TransactionDto(
    val id: UUID,
    val amount: BigDecimal,
    val category: String,
    val description: String?,
    val date: String,
    val type: TransactionTypeEnum
)