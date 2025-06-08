package com.accounting.bot.accountingbot.common.api

import org.springframework.beans.factory.annotation.Value
import org.springframework.core.ParameterizedTypeReference
import org.springframework.http.*
import org.springframework.stereotype.Service
import org.springframework.web.client.RestTemplate
import java.util.*



@Service
class TransactionClient(
    private val restTemplate: RestTemplate,
    @Value("\${api.base-url}") private val baseUrl: String
) {
    data class CreateIncomeDto(
        val amount: Double,
        val category: String,
        val description: String? = null
    )

    data class CreateExpenseDto(
        val amount: Double,
        val category: String,
        val description: String? = null
    )

    data class TransactionDto(
        val id: UUID,
        val amount: Double,
        val category: String,
        val description: String?,
        val date: String,
        val type: TransactionType
    )

    data class CategoryExpenseSummary(
        val categoryCode: String,
        val totalAmount: Double
    )

    data class PageTransactionDto(
        val content: List<TransactionDto>,
        val page: PageInfo
    )

    data class PageInfo(
        val size: Int,
        val number: Int,
        val totalElements: Long,
        val totalPages: Int
    )

    data class PageableObject(
        val offset: Long,
        val sort: List<SortObject>,
        val paged: Boolean,
        val pageNumber: Int,
        val pageSize: Int,
        val unpaged: Boolean
    )

    data class SortObject(
        val direction: String?,
        val nullHandling: String?,
        val ascending: Boolean,
        val property: String?,
        val ignoreCase: Boolean?
    )

    enum class TransactionType {
        EXPENSE, INCOME
    }

    private fun headers(token: String): HttpHeaders = HttpHeaders().apply {
        contentType = MediaType.APPLICATION_JSON
        setBearerAuth(token)
    }

    fun getTransaction(id: UUID, token: String): TransactionDto {
        val url = "$baseUrl/transactions/$id"
        val entity = HttpEntity<Void>(headers(token))
        val response = restTemplate.exchange(url, HttpMethod.GET, entity, TransactionDto::class.java)
        return response.body ?: throw RuntimeException("Not found")
    }

    fun updateTransaction(id: UUID, dto: TransactionDto, token: String): TransactionDto {
        val url = "$baseUrl/transactions/$id"
        val entity = HttpEntity(dto, headers(token))
        val response = restTemplate.exchange(url, HttpMethod.PUT, entity, TransactionDto::class.java)
        return response.body ?: throw RuntimeException("Update failed")
    }

    fun deleteTransaction(id: UUID, token: String) {
        val url = "$baseUrl/transactions/$id"
        val entity = HttpEntity<Void>(headers(token))
        restTemplate.exchange(url, HttpMethod.DELETE, entity, Void::class.java)
    }

    fun createIncome(dto: CreateIncomeDto, token: String): TransactionDto {
        val url = "$baseUrl/transactions/income"
        val entity = HttpEntity(dto, headers(token))
        val response = restTemplate.postForEntity(url, entity, TransactionDto::class.java)
        return response.body ?: throw RuntimeException("Failed to create income")
    }

    fun createExpense(dto: CreateExpenseDto, token: String): TransactionDto {
        val url = "$baseUrl/transactions/expense"
        val entity = HttpEntity(dto, headers(token))
        val response = restTemplate.postForEntity(url, entity, TransactionDto::class.java)
        return response.body ?: throw RuntimeException("Failed to create expense")
    }

    fun getAllTransactions(page: Int, size: Int, token: String): PageTransactionDto {
        val url = "$baseUrl/transactions?page=$page&size=$size"
        val entity = HttpEntity<Void>(headers(token))
        val response = restTemplate.exchange(url, HttpMethod.GET, entity, PageTransactionDto::class.java)
        return response.body ?: throw RuntimeException("Fetch failed")
    }

    fun getTodayExpensesByCategory(token: String): List<CategoryExpenseSummary> {
        val url = "$baseUrl/transactions/expenses/today"
        val entity = HttpEntity<Void>(headers(token))
        val response = restTemplate.exchange(url, HttpMethod.GET, entity,
            object : ParameterizedTypeReference<List<CategoryExpenseSummary>>() {})
        return response.body ?: emptyList()
    }
}