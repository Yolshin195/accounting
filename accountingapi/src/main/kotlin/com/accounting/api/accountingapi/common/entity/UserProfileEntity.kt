package com.accounting.api.accountingapi.common.entity

import com.accounting.api.accountingapi.common.dto.UserRoleEnum
import jakarta.persistence.*

@Entity
@Table(name = "user_profiles")
class UserProfileEntity(

    @Column(nullable = false, unique = true)
    val username: String,

    @Column(nullable = false)
    val hashPassword: String,

    @ElementCollection(fetch = FetchType.EAGER)
    @Enumerated(EnumType.STRING)
    @CollectionTable(name = "user_roles", joinColumns = [JoinColumn(name = "user_id")])
    @Column(name = "role")
    val roles: Set<UserRoleEnum> = setOf(UserRoleEnum.ROLE_USER),

    @Column(nullable = false)
    var enabled: Boolean = true,

    @Column(nullable = false)
    var accountNonExpired: Boolean = true,

    @Column(nullable = false)
    var credentialsNonExpired: Boolean = true,

    @Column(nullable = false)
    var accountNonLocked: Boolean = true,

    @Version
    var version: Long? = null // <--- Добавьте это поле
) : BaseEntity()