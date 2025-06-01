package com.accounting.api.accountingapi.common.entity

import com.accounting.api.accountingapi.common.dto.transaction.TransactionTypeEnum
import jakarta.persistence.*

@Entity
@Table(
    name = "categories",
    uniqueConstraints = [
        UniqueConstraint(columnNames = ["user_id", "code"])
    ]
)
class CategoryEntity(

    @Column(nullable = false)
    val code: String,

    @Column(nullable = false)
    val name: String,

    @Column(nullable = true)
    val description: String? = null,

    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "user_id", nullable = false)
    val user: UserProfileEntity,

    @Enumerated(EnumType.STRING)  // Добавляем для enum поля
    @Column(nullable = false)
    val type: TransactionTypeEnum = TransactionTypeEnum.EXPENSE,

    @Version
    val version: Long? = null
) : BaseEntity()