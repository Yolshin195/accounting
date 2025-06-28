package com.accounting.api.accountingapi.common.dto.transaction

import com.fasterxml.jackson.annotation.JsonFormat
import java.math.BigDecimal
import java.time.LocalDate


data class CreateExpenseDto(
    val amount: BigDecimal,
    val category: String,
    val description: String?,
    @JsonFormat(pattern = "yyyy-MM-dd")
    val date: LocalDate? = null
)
