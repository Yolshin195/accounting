// API functions for interacting with the backend at http://localhost:8888
// Based on OpenAPI 3.0 specification

const API_BASE_URL = "http://localhost:8888"

// Helper function to get auth headers
const getAuthHeaders = () => {
  const token = localStorage.getItem("token")
  return {
    "Content-Type": "application/json",
    ...(token && { Authorization: `Bearer ${token}` }),
  }
}

// Helper function to handle API responses
const handleResponse = async (response: Response) => {
  if (!response.ok) {
    // Если получили 401 (Unauthorized), очищаем токен и перенаправляем на логин
    if (response.status === 401) {
      localStorage.removeItem("token")
      localStorage.removeItem("username")
      window.location.href = "/login"
      return
    }

    const errorData = await response.json().catch(() => ({ message: "Network error" }))
    throw new Error(errorData.message || `HTTP error! status: ${response.status}`)
  }
  return response.json()
}

// Auth API (БЕЗ токена)
export const loginUser = async (username: string, password: string) => {
  const response = await fetch(`${API_BASE_URL}/users/login`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ username, password }),
  })
  return handleResponse(response)
}

export const registerUser = async (email: string, password: string, name: string) => {
  const response = await fetch(`${API_BASE_URL}/users/register`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ email, password, name }),
  })
  return handleResponse(response)
}

// Categories API (С токеном)
export const getCategories = async (page = 0, size = 10) => {
  const response = await fetch(`${API_BASE_URL}/categories?page=${page}&size=${size}`, {
    headers: getAuthHeaders(),
  })
  return handleResponse(response)
}

export const createCategory = async (categoryData: {
  code: string
  name: string
  type: "INCOME" | "EXPENSE"
  description?: string
}) => {
  const response = await fetch(`${API_BASE_URL}/categories`, {
    method: "POST",
    headers: getAuthHeaders(),
    body: JSON.stringify(categoryData),
  })
  return handleResponse(response)
}

export const deleteCategory = async (code: string) => {
  const response = await fetch(`${API_BASE_URL}/categories/${code}`, {
    method: "DELETE",
    headers: getAuthHeaders(),
  })
  if (!response.ok) {
    if (response.status === 401) {
      localStorage.removeItem("token")
      localStorage.removeItem("username")
      window.location.href = "/login"
      return
    }
    const errorData = await response.json().catch(() => ({ message: "Network error" }))
    throw new Error(errorData.message || `HTTP error! status: ${response.status}`)
  }
}

// Transactions API (С токеном)
export const getTransactions = async (page = 0, size = 10) => {
  const response = await fetch(`${API_BASE_URL}/transactions?page=${page}&size=${size}`, {
    headers: getAuthHeaders(),
  })
  return handleResponse(response)
}

export const createExpenseTransaction = async (transactionData: {
  amount: number
  description?: string // Сделать необязательным
  category: string
  date?: string
}) => {
  const response = await fetch(`${API_BASE_URL}/transactions/expense`, {
    method: "POST",
    headers: getAuthHeaders(),
    body: JSON.stringify(transactionData),
  })
  return handleResponse(response)
}

export const createIncomeTransaction = async (transactionData: {
  amount: number
  description?: string // Сделать необязательным
  category: string
  date?: string
}) => {
  const response = await fetch(`${API_BASE_URL}/transactions/income`, {
    method: "POST",
    headers: getAuthHeaders(),
    body: JSON.stringify(transactionData),
  })
  return handleResponse(response)
}

export const updateTransaction = async (
  id: string,
  transactionData: {
    amount: number
    description?: string // Сделать необязательным
    category: string
    date: string
  },
) => {
  const response = await fetch(`${API_BASE_URL}/transactions/${id}`, {
    method: "PUT",
    headers: getAuthHeaders(),
    body: JSON.stringify(transactionData),
  })
  return handleResponse(response)
}

export const deleteTransaction = async (id: string) => {
  const response = await fetch(`${API_BASE_URL}/transactions/${id}`, {
    method: "DELETE",
    headers: getAuthHeaders(),
  })
  if (!response.ok) {
    if (response.status === 401) {
      localStorage.removeItem("token")
      localStorage.removeItem("username")
      window.location.href = "/login"
      return
    }
    const errorData = await response.json().catch(() => ({ message: "Network error" }))
    throw new Error(errorData.message || `HTTP error! status: ${response.status}`)
  }
}
