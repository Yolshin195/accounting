package com.accounting.api.accountingapi.controller.v1

import com.accounting.api.accountingapi.common.dto.CategoryDto
import com.accounting.api.accountingapi.common.dto.CreateCategoryDto
import com.accounting.api.accountingapi.service.CategoryService
import org.springframework.data.domain.Page
import org.springframework.data.domain.Pageable
import org.springframework.data.web.PageableDefault
import org.springframework.http.ResponseEntity
import org.springframework.web.bind.annotation.*


@ApiV1Controller
@RequestMapping("/categories")
class CategoryController(
    private val categoryService: CategoryService
) {

    @GetMapping
    fun getAllCategories(
        @PageableDefault(size = 20) pageable: Pageable
    ): ResponseEntity<Page<CategoryDto>> {
        val categories = categoryService.getAllCategories(pageable)
        return ResponseEntity.ok(categories)
    }

    @PostMapping
    fun createCategory(@RequestBody dto: CreateCategoryDto): ResponseEntity<CategoryDto> {
        val created = categoryService.createCategory(dto)
        return ResponseEntity.ok(created)
    }

    @DeleteMapping("/{code}")
    fun deleteCategory(@PathVariable code: String): ResponseEntity<Void> {
        categoryService.deleteCategory(code)
        return ResponseEntity.noContent().build()
    }
}