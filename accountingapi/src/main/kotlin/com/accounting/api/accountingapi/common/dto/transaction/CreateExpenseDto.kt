package com.accounting.api.accountingapi.common.dto.transaction

import java.math.BigDecimal


data class CreateExpenseDto(
    val amount: BigDecimal,
    val category: String,
    val description: String?,
)
