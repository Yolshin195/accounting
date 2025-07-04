package com.accounting.api.accountingapi.repository

import com.accounting.api.accountingapi.common.dto.transaction.CategoryExpenseSummary
import com.accounting.api.accountingapi.common.entity.TransactionEntity
import com.accounting.api.accountingapi.common.entity.UserProfileEntity
import org.springframework.data.domain.Page
import org.springframework.data.domain.Pageable
import org.springframework.data.jpa.repository.JpaRepository
import org.springframework.data.jpa.repository.Query
import org.springframework.data.repository.query.Param
import java.time.Instant
import java.util.*

interface TransactionRepository : JpaRepository<TransactionEntity, UUID> {
    fun findAllByUser(user: UserProfileEntity, pageable: Pageable): Page<TransactionEntity>
    fun findByIdAndUser(id: UUID, user: UserProfileEntity): TransactionEntity?

    @Query("""
        SELECT t.category.code as categoryCode, SUM(t.amount) as totalAmount
        FROM TransactionEntity t
        WHERE t.user = :user 
          AND t.type = com.accounting.api.accountingapi.common.dto.transaction.TransactionTypeEnum.EXPENSE
          AND t.createdAt BETWEEN :startOfDay AND :endOfDay
        GROUP BY t.category.code
    """)
    fun getExpensesByCategoryForDay(
        @Param("user") user: UserProfileEntity,
        @Param("startOfDay") startOfDay: Instant,
        @Param("endOfDay") endOfDay: Instant
    ): List<CategoryExpenseSummary>

    @Query("""
        SELECT t FROM TransactionEntity t
        WHERE t.user = :user 
        AND YEAR(t.createdAt) = :year 
        AND MONTH(t.createdAt) = :month
    """)
    fun findAllByYearAndMonth(
        @Param("user") user: UserProfileEntity,
        @Param("year") year: Int,
        @Param("month") month: Int,
        pageable: Pageable
    ): Page<TransactionEntity>

}