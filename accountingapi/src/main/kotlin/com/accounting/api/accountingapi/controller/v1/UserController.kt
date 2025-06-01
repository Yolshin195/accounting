package com.accounting.api.accountingapi.controller.v1

import com.accounting.api.accountingapi.common.dto.CreateUserDto
import com.accounting.api.accountingapi.service.UserProfileService
import org.springframework.http.ResponseEntity
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RequestBody
import org.springframework.web.bind.annotation.RequestMapping


@ApiV1Controller
@RequestMapping("/users")
class UserController(
    val userProfileService: UserProfileService
) {

    data class LoginRequest(
        val username: String,
        val password: String
    )

    @PostMapping("/register")
    fun register(@RequestBody request: CreateUserDto): ResponseEntity<String> {
        userProfileService.register(request)
        return ResponseEntity.ok("User registered: ${request.username}")
    }

    @PostMapping("/login")
    fun login(@RequestBody request: LoginRequest): ResponseEntity<String> {
        return ResponseEntity.ok("Logged in: ${request.username}")
    }
}