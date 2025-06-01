package com.accounting.api.accountingapi.service

import com.accounting.api.accountingapi.common.dto.CreateUserDto
import com.accounting.api.accountingapi.common.entity.UserProfileEntity

interface UserProfileService {
    fun register(dto: CreateUserDto): UserProfileEntity
}