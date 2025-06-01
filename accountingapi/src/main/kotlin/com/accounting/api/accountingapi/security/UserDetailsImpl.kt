package com.accounting.api.accountingapi.security

import com.accounting.api.accountingapi.common.entity.UserProfileEntity
import org.springframework.security.core.GrantedAuthority
import org.springframework.security.core.authority.SimpleGrantedAuthority
import org.springframework.security.core.userdetails.UserDetails


class UserDetailsImpl(
    private val user: UserProfileEntity
) : UserDetails {

    override fun getAuthorities(): Collection<GrantedAuthority> =
        user.roles.map { SimpleGrantedAuthority(it.name) }.toSet()

    override fun getPassword(): String = user.hashPassword

    override fun getUsername(): String = user.username

    override fun isAccountNonExpired(): Boolean = user.accountNonExpired

    override fun isAccountNonLocked(): Boolean = user.accountNonLocked

    override fun isCredentialsNonExpired(): Boolean = user.credentialsNonExpired

    override fun isEnabled(): Boolean = user.enabled

    fun getUser(): UserProfileEntity = user
}