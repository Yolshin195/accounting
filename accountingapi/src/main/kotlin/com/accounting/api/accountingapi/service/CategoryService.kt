package com.accounting.api.accountingapi.service

import com.accounting.api.accountingapi.common.dto.CategoryDto
import com.accounting.api.accountingapi.common.dto.CreateCategoryDto
import com.accounting.api.accountingapi.common.dto.transaction.TransactionTypeEnum
import org.springframework.data.domain.Page
import org.springframework.data.domain.Pageable

interface CategoryService {
    fun getAllCategories(pageable: Pageable): Page<CategoryDto>
    fun getExpenseCategories(pageable: Pageable): Page<CategoryDto>
    fun getIncomeCategories(pageable: Pageable): Page<CategoryDto>
    fun createCategory(dto: CreateCategoryDto): CategoryDto
    fun deleteCategory(code: String)
}