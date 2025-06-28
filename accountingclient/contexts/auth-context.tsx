"use client"

import { createContext, useContext, useState, useEffect, type ReactNode } from "react"
import { loginUser, registerUser } from "@/lib/api"

interface User {
  username: string
}

interface AuthContextType {
  user: User | null
  loading: boolean
  login: (username: string, password: string) => Promise<void>
  register: (email: string, password: string, name: string) => Promise<void>
  logout: () => void
}

const AuthContext = createContext<AuthContextType | undefined>(undefined)

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null)
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    // Check for existing token on mount
    const token = localStorage.getItem("token")
    const username = localStorage.getItem("username")

    if (token && username) {
      setUser({ username })
    }
    setLoading(false)
  }, [])

  const login = async (username: string, password: string) => {
    const response = await loginUser(username, password)
    localStorage.setItem("token", response.token)
    localStorage.setItem("username", username)
    setUser({ username })
  }

  const register = async (email: string, password: string, name: string) => {
    const response = await registerUser(email, password, name)
    localStorage.setItem("token", response.token)
    localStorage.setItem("username", name) // Используем name как username
    setUser({ username: name })
  }

  const logout = () => {
    localStorage.removeItem("token")
    localStorage.removeItem("username")
    setUser(null)
  }

  return <AuthContext.Provider value={{ user, loading, login, register, logout }}>{children}</AuthContext.Provider>
}

export function useAuth() {
  const context = useContext(AuthContext)
  if (context === undefined) {
    throw new Error("useAuth must be used within an AuthProvider")
  }
  return context
}
