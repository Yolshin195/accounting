package com.accounting.api.accountingapi.common.entity

import com.accounting.api.accountingapi.common.dto.transaction.TransactionTypeEnum
import jakarta.persistence.*
import java.math.BigDecimal

@Entity
@Table(name = "transactions")
class TransactionEntity(

    @Column(nullable = false)
    val amount: BigDecimal,

    @Column(nullable = true)
    var description: String? = null,

    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    val type: TransactionTypeEnum,

    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "category_id", nullable = false)
    var category: CategoryEntity,

    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "user_id", nullable = false)
    val user: UserProfileEntity,

    @Version
    val version: Long? = null
) : BaseEntity()