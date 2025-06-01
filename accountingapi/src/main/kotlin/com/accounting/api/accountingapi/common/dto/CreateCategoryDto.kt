package com.accounting.api.accountingapi.common.dto

import com.accounting.api.accountingapi.common.dto.transaction.TransactionTypeEnum


data class CreateCategoryDto (
    val code: String,
    val name: String,
    val description: String?,
    val type: TransactionTypeEnum = TransactionTypeEnum.EXPENSE,
)