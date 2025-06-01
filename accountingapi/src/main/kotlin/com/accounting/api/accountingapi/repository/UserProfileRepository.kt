package com.accounting.api.accountingapi.repository

import com.accounting.api.accountingapi.common.entity.UserProfileEntity
import org.springframework.data.repository.CrudRepository
import java.util.UUID


interface UserProfileRepository : CrudRepository<UserProfileEntity, UUID> {
    fun findByUsername(username: String): UserProfileEntity?
}