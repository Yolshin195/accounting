package com.accounting.api.accountingapi.common.dto

import com.accounting.api.accountingapi.common.dto.transaction.TransactionTypeEnum
import java.util.*

data class CategoryDto(
    val id: UUID,
    val code: String,
    val name: String,
    val description: String?,

    val type: TransactionTypeEnum,
)
