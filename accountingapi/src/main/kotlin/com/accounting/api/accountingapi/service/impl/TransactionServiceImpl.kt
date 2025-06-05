package com.accounting.api.accountingapi.service.impl

import com.accounting.api.accountingapi.common.dto.transaction.*
import com.accounting.api.accountingapi.common.entity.TransactionEntity
import com.accounting.api.accountingapi.repository.CategoryRepository
import com.accounting.api.accountingapi.repository.TransactionRepository
import com.accounting.api.accountingapi.service.CurrentUserService
import com.accounting.api.accountingapi.service.TransactionService
import org.springframework.data.domain.Page
import org.springframework.data.domain.Pageable
import org.springframework.data.domain.PageImpl
import org.springframework.stereotype.Service
import java.time.LocalDate
import java.time.ZoneOffset
import java.util.*

@Service
class TransactionServiceImpl(
    private val transactionRepository: TransactionRepository,
    private val categoryRepository: CategoryRepository,
    private val currentUserService: CurrentUserService
) : TransactionService {

    override fun getAllTransactions(pageable: Pageable): Page<TransactionDto> {
        val user = currentUserService.getCurrentUser()
        val page = transactionRepository.findAllByUser(user, pageable)

        val dtoList = page.content.map { tx ->
            TransactionDto(
                id = tx.id!!,
                amount = tx.amount,
                category = tx.category.code,
                description = tx.description,
                date = tx.createdAt.toString(),
                type = tx.type
            )
        }
        return PageImpl(dtoList, pageable, page.totalElements)
    }

    override fun getTransactionById(id: UUID): TransactionDto {
        val tx = transactionRepository.findById(id).orElseThrow { NoSuchElementException("Transaction not found") }
        return TransactionDto(
            id = tx.id!!,
            amount = tx.amount,
            category = tx.category.code,
            description = tx.description,
            date = tx.createdAt.toString(),
            type = tx.type
        )
    }

    override fun createExpense(dto: CreateExpenseDto): TransactionDto {
        return createTransaction(dto.amount, dto.category, dto.description, TransactionTypeEnum.EXPENSE)
    }

    override fun createIncome(dto: CreateIncomeDto): TransactionDto {
        return createTransaction(dto.amount, dto.category, dto.description, TransactionTypeEnum.INCOME)
    }

    private fun createTransaction(
        amount: java.math.BigDecimal,
        categoryCode: String,
        description: String?,
        type: TransactionTypeEnum
    ): TransactionDto {
        val user = currentUserService.getCurrentUser()
        val category = categoryRepository.findByUserAndCode(user, categoryCode.uppercase())
            ?: throw IllegalArgumentException("Category not found: $categoryCode")

        val entity = TransactionEntity(
            amount = amount,
            description = description,
            type = type,
            category = category,
            user = user
        )
        val saved = transactionRepository.save(entity)
        return TransactionDto(
            id = saved.id!!,
            amount = saved.amount,
            category = saved.category.code,
            description = saved.description,
            date = saved.createdAt.toString(),
            type = saved.type
        )
    }

    override fun updateTransaction(id: UUID, dto: TransactionDto): TransactionDto {
        val user = currentUserService.getCurrentUser()

        val existing = transactionRepository.findByIdAndUser(id, user)
            ?: throw NoSuchElementException("Transaction not found or does not belong to the current user")

        val category = categoryRepository.findByUserAndCode(user, dto.category.lowercase())
            ?: throw IllegalArgumentException("Category not found: ${dto.category}")

        existing.description = dto.description
        existing.category = category

        val saved = transactionRepository.save(existing)

        return TransactionDto(
            id = saved.id!!,
            amount = saved.amount,
            category = saved.category.code,
            description = saved.description,
            date = saved.createdAt.toString(),
            type = saved.type
        )
    }

    override fun deleteTransaction(id: UUID) {
        val user = currentUserService.getCurrentUser()
        val existing = transactionRepository.findByIdAndUser(id, user)
            ?: throw NoSuchElementException("Transaction not found or does not belong to the current user")
        transactionRepository.delete(existing)
    }

    override fun getTodayExpensesByCategory(): List<CategoryExpenseSummary> {
        val user = currentUserService.getCurrentUser()
        val zoneId = ZoneOffset.UTC
        val today = LocalDate.now(zoneId)
        val startOfDay = today.atStartOfDay(zoneId).toInstant()
        val endOfDay = today.plusDays(1).atStartOfDay(zoneId).toInstant()

        return transactionRepository.getExpensesByCategoryForDay(user, startOfDay, endOfDay)
    }
}