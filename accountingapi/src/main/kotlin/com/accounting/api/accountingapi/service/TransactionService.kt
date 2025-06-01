package com.accounting.api.accountingapi.service

import com.accounting.api.accountingapi.common.dto.transaction.*
import org.springframework.data.domain.Page
import org.springframework.data.domain.Pageable
import java.util.*

interface TransactionService {
    fun getAllTransactions(pageable: Pageable): Page<TransactionDto>
    fun getTransactionById(id: UUID): TransactionDto
    fun createExpense(dto: CreateExpenseDto): TransactionDto
    fun createIncome(dto: CreateIncomeDto): TransactionDto
    fun updateTransaction(id: UUID, dto: TransactionDto): TransactionDto
    fun deleteTransaction(id: UUID)
    fun getTodayExpensesByCategory(): List<CategoryExpenseSummary>
}