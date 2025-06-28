package com.accounting.api.accountingapi.repository

import com.accounting.api.accountingapi.common.dto.transaction.TransactionTypeEnum
import com.accounting.api.accountingapi.common.entity.CategoryEntity
import com.accounting.api.accountingapi.common.entity.UserProfileEntity
import org.springframework.data.domain.Page
import org.springframework.data.jpa.repository.JpaRepository
import org.springframework.data.domain.Pageable
import java.util.*

interface CategoryRepository : JpaRepository<CategoryEntity, UUID> {
    fun findAllByUser(user: UserProfileEntity, pageable: Pageable): Page<CategoryEntity>
    fun findAllByUserAndType(user: UserProfileEntity, type: TransactionTypeEnum, pageable: Pageable): Page<CategoryEntity>
    fun findByUserAndCode(user: UserProfileEntity, code: String): CategoryEntity?
}