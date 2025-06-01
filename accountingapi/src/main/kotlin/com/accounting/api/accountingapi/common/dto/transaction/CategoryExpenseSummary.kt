package com.accounting.api.accountingapi.common.dto.transaction

import java.math.BigDecimal

data class CategoryExpenseSummary(
    val categoryCode: String,
    val totalAmount: BigDecimal,
)
