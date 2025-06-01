package com.accounting.api.accountingapi.service

import com.accounting.api.accountingapi.common.entity.UserProfileEntity

interface CurrentUserService {
    fun getCurrentUser(): UserProfileEntity
}