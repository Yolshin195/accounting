"use client"

import { createContext, useContext, useState, useEffect, type ReactNode } from "react"

type Locale = "en" | "th" | "ru"

interface LocaleContextType {
  locale: Locale
  setLocale: (locale: Locale) => void
  t: (key: string) => string
  loading: boolean
}

const LocaleContext = createContext<LocaleContextType | undefined>(undefined)

interface Translations {
  [key: string]: any
}

export function LocaleProvider({ children }: { children: ReactNode }) {
  const [locale, setLocaleState] = useState<Locale>("ru")
  const [translations, setTranslations] = useState<Translations>({})
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    // Load saved locale from localStorage
    const savedLocale = localStorage.getItem("locale") as Locale
    if (savedLocale && ["en", "th", "ru"].includes(savedLocale)) {
      setLocaleState(savedLocale)
    }
  }, [])

  useEffect(() => {
    loadTranslations(locale)
  }, [locale])

  const loadTranslations = async (currentLocale: Locale) => {
    try {
      setLoading(true)
      const response = await fetch(`/locales/${currentLocale}.json`)
      const data = await response.json()
      setTranslations(data)
    } catch (error) {
      console.error("Failed to load translations:", error)
      // Fallback to English if loading fails
      if (currentLocale !== "en") {
        try {
          const response = await fetch("/locales/en.json")
          const data = await response.json()
          setTranslations(data)
        } catch (fallbackError) {
          console.error("Failed to load fallback translations:", fallbackError)
        }
      }
    } finally {
      setLoading(false)
    }
  }

  const setLocale = (newLocale: Locale) => {
    setLocaleState(newLocale)
    localStorage.setItem("locale", newLocale)
  }

  const t = (key: string): string => {
    const keys = key.split(".")
    let value = translations

    for (const k of keys) {
      if (value && typeof value === "object" && k in value) {
        value = value[k]
      } else {
        return key // Return key if translation not found
      }
    }

    return typeof value === "string" ? value : key
  }

  return <LocaleContext.Provider value={{ locale, setLocale, t, loading }}>{children}</LocaleContext.Provider>
}

export function useLocale() {
  const context = useContext(LocaleContext)
  if (context === undefined) {
    throw new Error("useLocale must be used within a LocaleProvider")
  }
  return context
}
