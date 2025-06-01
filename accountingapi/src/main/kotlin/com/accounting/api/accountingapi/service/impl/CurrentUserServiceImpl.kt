package com.accounting.api.accountingapi.service.impl

import com.accounting.api.accountingapi.common.entity.UserProfileEntity
import com.accounting.api.accountingapi.security.UserDetailsImpl
import com.accounting.api.accountingapi.service.CurrentUserService
import org.springframework.security.core.context.SecurityContextHolder
import org.springframework.stereotype.Service


@Service
class CurrentUserServiceImpl : CurrentUserService {
    override fun getCurrentUser(): UserProfileEntity {
        val auth = SecurityContextHolder.getContext().authentication
        val principal = auth?.principal
        if (principal is UserDetailsImpl) {
            return principal.getUser()
        } else {
            throw IllegalStateException("Unknown principal type or not authenticated")
        }
    }
}