package com.accounting.bot.accountingbot.common.api

import org.springframework.beans.factory.annotation.Value
import org.springframework.http.*
import org.springframework.stereotype.Service
import org.springframework.web.client.RestTemplate
import java.util.*


@Service
class CategoryClient(
    private val restTemplate: RestTemplate,
    @Value("\${api.base-url}") private val baseUrl: String
) {
    data class CreateCategoryDto(
        val code: String,
        val name: String,
        val description: String? = null,
        val type: CategoryType
    )

    data class CategoryDto(
        val id: UUID,
        val code: String,
        val name: String,
        val description: String?,
        val type: CategoryType
    )

    data class PageCategoryDto(
        val totalElements: Long,
        val totalPages: Int,
        val first: Boolean,
        val last: Boolean,
        val size: Int,
        val content: List<CategoryDto>,
        val number: Int,
        val sort: SortObject, // ❗️Было List<SortObject>, должно быть просто SortObject
        val numberOfElements: Int,
        val pageable: PageableObject,
        val empty: Boolean
    )

    data class PageableObject(
        val offset: Long,
        val sort: SortObject, // ❗️То же самое
        val paged: Boolean,
        val pageNumber: Int,
        val pageSize: Int,
        val unpaged: Boolean
    )

    data class SortObject(
        val empty: Boolean,
        val unsorted: Boolean,
        val sorted: Boolean
    )

    enum class CategoryType {
        EXPENSE, INCOME
    }

    private fun headers(token: String): HttpHeaders = HttpHeaders().apply {
        contentType = MediaType.APPLICATION_JSON
        setBearerAuth(token)
    }

    fun getAllCategories(page: Int, size: Int, token: String): PageCategoryDto {
        val url = "$baseUrl/categories?page=$page&size=$size"
        val entity = HttpEntity<Void>(headers(token))
        val response = restTemplate.exchange(url, HttpMethod.GET, entity, PageCategoryDto::class.java)
        return response.body ?: throw RuntimeException("Get categories failed")
    }

    fun createCategory(dto: CreateCategoryDto, token: String): CategoryDto {
        val url = "$baseUrl/categories"
        val entity = HttpEntity(dto, headers(token))
        val response = restTemplate.postForEntity(url, entity, CategoryDto::class.java)
        return response.body ?: throw RuntimeException("Create failed")
    }

    fun deleteCategory(code: String, token: String) {
        val url = "$baseUrl/categories/$code"
        val entity = HttpEntity<Void>(headers(token))
        restTemplate.exchange(url, HttpMethod.DELETE, entity, Void::class.java)
    }
}