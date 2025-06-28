package com.accounting.api.accountingapi.service.impl

import com.accounting.api.accountingapi.common.dto.CategoryDto
import com.accounting.api.accountingapi.common.dto.CreateCategoryDto
import com.accounting.api.accountingapi.common.dto.transaction.TransactionTypeEnum
import com.accounting.api.accountingapi.common.entity.CategoryEntity
import com.accounting.api.accountingapi.repository.CategoryRepository
import com.accounting.api.accountingapi.service.CategoryService
import com.accounting.api.accountingapi.service.CurrentUserService
import org.springframework.data.domain.Page
import org.springframework.data.domain.Pageable
import org.springframework.data.domain.PageImpl
import org.springframework.stereotype.Service
import java.util.*

@Service
class CategoryServiceImpl(
    private val categoryRepository: CategoryRepository,
    private val currentUserService: CurrentUserService
) : CategoryService {

    override fun getAllCategories(pageable: Pageable): Page<CategoryDto> {
        val user = currentUserService.getCurrentUser()
        val page = categoryRepository.findAllByUser(user, pageable)
        val dtoList = page.content.map {
            CategoryDto(
                id = it.id!!,
                code = it.code,
                name = it.name,
                description = it.description,
                type = it.type
            )
        }
        return PageImpl(dtoList, pageable, page.totalElements)
    }

    override fun getExpenseCategories(pageable: Pageable): Page<CategoryDto> {
        val user = currentUserService.getCurrentUser()
        val page = categoryRepository.findAllByUserAndType(user, TransactionTypeEnum.EXPENSE, pageable)
        val dtoList = page.content.map {
            CategoryDto(
                id = it.id!!,
                code = it.code,
                name = it.name,
                description = it.description,
                type = it.type
            )
        }
        return PageImpl(dtoList, pageable, page.totalElements)
    }

    override fun getIncomeCategories(pageable: Pageable): Page<CategoryDto> {
        val user = currentUserService.getCurrentUser()
        val page = categoryRepository.findAllByUserAndType(user, TransactionTypeEnum.INCOME, pageable)
        val dtoList = page.content.map {
            CategoryDto(
                id = it.id!!,
                code = it.code,
                name = it.name,
                description = it.description,
                type = it.type
            )
        }
        return PageImpl(dtoList, pageable, page.totalElements)
    }

    override fun createCategory(dto: CreateCategoryDto): CategoryDto {
        val user = currentUserService.getCurrentUser()
        val category = CategoryEntity(
            code = dto.code.uppercase(),
            name = dto.name,
            description = dto.description,
            type = dto.type,
            user = user
        )
        val saved = categoryRepository.save(category)
        return CategoryDto(
            id = saved.id!!,
            code = saved.code,
            name = saved.name,
            description = saved.description,
            type = saved.type
        )
    }

    override fun deleteCategory(code: String) {
        val user = currentUserService.getCurrentUser()
        val category = categoryRepository.findByUserAndCode(user, code)
            ?: throw NoSuchElementException("Category with code '$code' not found")
        categoryRepository.delete(category)
    }
}