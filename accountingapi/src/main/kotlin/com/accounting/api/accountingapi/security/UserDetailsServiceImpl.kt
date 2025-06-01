package com.accounting.api.accountingapi.security

import com.accounting.api.accountingapi.repository.UserProfileRepository
import org.springframework.security.core.userdetails.UserDetails
import org.springframework.security.core.userdetails.UserDetailsService
import org.springframework.security.core.userdetails.UsernameNotFoundException
import org.springframework.stereotype.Service


@Service
class UserDetailsServiceImpl(
    private val userProfileRepository: UserProfileRepository
) : UserDetailsService {

    override fun loadUserByUsername(username: String): UserDetails {
        val user =  userProfileRepository.findByUsername(username)
            ?: throw UsernameNotFoundException("User not found: $username")

        return UserDetailsImpl(user)
    }
}