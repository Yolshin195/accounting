package com.accounting.api.accountingapi.service.impl

import com.accounting.api.accountingapi.common.dto.CreateUserDto
import com.accounting.api.accountingapi.common.dto.UserRoleEnum
import com.accounting.api.accountingapi.common.entity.UserProfileEntity
import com.accounting.api.accountingapi.repository.UserProfileRepository
import com.accounting.api.accountingapi.service.UserProfileService
import org.springframework.security.crypto.password.PasswordEncoder
import org.springframework.stereotype.Service


@Service
class UserProfileServiceImpl(
    private val userProfileRepository: UserProfileRepository,
    private val passwordEncoder: PasswordEncoder
) : UserProfileService {

    override fun register(dto: CreateUserDto): UserProfileEntity {
        // Проверка на уникальность логина
        if (userProfileRepository.findByUsername(dto.username) != null) {
            throw IllegalArgumentException("Username already exists")
        }

        val hashedPassword = passwordEncoder.encode(dto.password)

        val user = UserProfileEntity(
            username = dto.username,
            hashPassword = hashedPassword,
            roles = setOf(UserRoleEnum.ROLE_USER)
        )

        return userProfileRepository.save(user)
    }
}