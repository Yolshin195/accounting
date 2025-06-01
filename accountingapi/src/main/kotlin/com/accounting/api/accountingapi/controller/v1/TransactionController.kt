package com.accounting.api.accountingapi.controller.v1

import com.accounting.api.accountingapi.common.dto.transaction.CategoryExpenseSummary
import com.accounting.api.accountingapi.common.dto.transaction.CreateExpenseDto
import com.accounting.api.accountingapi.common.dto.transaction.CreateIncomeDto

import org.springframework.http.ResponseEntity
import org.springframework.web.bind.annotation.*
import org.springframework.data.domain.Page
import org.springframework.data.domain.Pageable
import org.springframework.data.web.PageableDefault

import com.accounting.api.accountingapi.common.dto.transaction.TransactionDto
import com.accounting.api.accountingapi.service.TransactionService
import java.util.*


@ApiV1Controller
@RequestMapping("/transactions")
class TransactionController(
    private val transactionService: TransactionService
) {
    @GetMapping
    fun getAllTransactions(
        @PageableDefault(size = 20) pageable: Pageable
    ): ResponseEntity<Page<TransactionDto>> {
        val page = transactionService.getAllTransactions(pageable)
        return ResponseEntity.ok(page)
    }

    @GetMapping("/{id}")
    fun getTransactionById(@PathVariable id: UUID): ResponseEntity<TransactionDto> {
        val transaction = transactionService.getTransactionById(id)
        return ResponseEntity.ok(transaction)
    }

    @PostMapping("/expense")
    fun createExpense(@RequestBody dto: CreateExpenseDto): ResponseEntity<TransactionDto> {
        val created = transactionService.createExpense(dto)
        return ResponseEntity.ok(created)
    }

    @PostMapping("/income")
    fun createIncome(@RequestBody dto: CreateIncomeDto): ResponseEntity<TransactionDto> {
        val created = transactionService.createIncome(dto)
        return ResponseEntity.ok(created)
    }

    @PutMapping("/{id}")
    fun updateTransaction(@PathVariable id: UUID, @RequestBody dto: TransactionDto): ResponseEntity<TransactionDto> {
        val updated = transactionService.updateTransaction(id, dto)
        return ResponseEntity.ok(updated)
    }

    @DeleteMapping("/{id}")
    fun deleteTransaction(@PathVariable id: UUID): ResponseEntity<Void> {
        transactionService.deleteTransaction(id)
        return ResponseEntity.noContent().build()
    }

    @GetMapping("/expenses/today")
    fun getTodayExpensesByCategory(): ResponseEntity<List<CategoryExpenseSummary>> {
        val result = transactionService.getTodayExpensesByCategory()
        return ResponseEntity.ok(result)
    }
}