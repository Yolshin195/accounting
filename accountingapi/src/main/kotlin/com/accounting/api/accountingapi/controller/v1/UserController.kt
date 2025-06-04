package com.accounting.api.accountingapi.controller.v1

import com.accounting.api.accountingapi.common.dto.CreateUserDto
import com.accounting.api.accountingapi.common.dto.LoginTelegramBotDto
import com.accounting.api.accountingapi.security.JwtUtil
import com.accounting.api.accountingapi.security.TelegramBotAuthService
import com.accounting.api.accountingapi.service.UserProfileService
import org.springframework.http.HttpStatus
import org.springframework.http.ResponseEntity
import org.springframework.security.authentication.AuthenticationManager
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken
import org.springframework.security.core.userdetails.UserDetailsService
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RequestBody
import org.springframework.web.bind.annotation.RequestMapping


@ApiV1Controller
@RequestMapping("/users")
class UserController(
    val userProfileService: UserProfileService,
    private val authenticationManager: AuthenticationManager,
    private val jwtUtil: JwtUtil,
    private val userDetailsService: UserDetailsService,
    private val telegramBotAuthService: TelegramBotAuthService,
) {

    data class LoginRequest(
        val username: String,
        val password: String
    )

    data class JwtResponse(val token: String)

    @PostMapping("/register")
    fun register(@RequestBody request: CreateUserDto): ResponseEntity<String> {
        userProfileService.register(request)
        return ResponseEntity.ok("User registered: ${request.username}")
    }

    @PostMapping("/login")
    fun login(@RequestBody request: LoginRequest): ResponseEntity<JwtResponse> {
        authenticationManager.authenticate(
            UsernamePasswordAuthenticationToken(request.username, request.password)
        )

        val userDetails = userDetailsService.loadUserByUsername(request.username)
        val token = jwtUtil.generateToken(userDetails)

        return ResponseEntity.ok(JwtResponse(token))
    }

    @PostMapping("/login/telegram")
    fun loginTelegramBot(@RequestBody request: LoginTelegramBotDto): ResponseEntity<JwtResponse> {
        try {
            val token = telegramBotAuthService.authenticateTelegramBot(request)
            return ResponseEntity.ok(JwtResponse(token))
        } catch (e: SecurityException) {
            return ResponseEntity.status(HttpStatus.FORBIDDEN)
                .body(JwtResponse("Access denied: ${e.message}"))
        } catch (e: IllegalArgumentException) {
            return ResponseEntity.status(HttpStatus.BAD_REQUEST)
                .body(JwtResponse("Invalid request: ${e.message}"))
        } catch (e: Exception) {
            return ResponseEntity.status(HttpStatus.INTERNAL_SERVER_ERROR)
                .body(JwtResponse("Authentication failed: ${e.message}"))
        }
    }
}