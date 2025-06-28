// Утилиты для работы с датами и временем

export const convertUTCToLocal = (utcDateString: string): Date => {
  // Создаем объект Date из UTC строки
  const utcDate = new Date(utcDateString)

  // Возвращаем локальную дату
  return new Date(utcDate.getTime())
}

export const formatDateForLocale = (
  date: Date | string,
  locale: string,
  options?: Intl.DateTimeFormatOptions,
): string => {
  const dateObj = typeof date === "string" ? convertUTCToLocal(date) : date

  const localeMap = {
    en: "en-US",
    th: "th-TH",
    ru: "ru-RU",
  }

  const defaultOptions: Intl.DateTimeFormatOptions = {
    year: "numeric",
    month: "long",
    day: "numeric",
    ...options,
  }

  return dateObj.toLocaleDateString(localeMap[locale as keyof typeof localeMap] || "ru-RU", defaultOptions)
}

export const formatDateTimeForLocale = (
  date: Date | string,
  locale: string,
  options?: Intl.DateTimeFormatOptions,
): string => {
  const dateObj = typeof date === "string" ? convertUTCToLocal(date) : date

  const localeMap = {
    en: "en-US",
    th: "th-TH",
    ru: "ru-RU",
  }

  const defaultOptions: Intl.DateTimeFormatOptions = {
    year: "numeric",
    month: "long",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
    ...options,
  }

  return dateObj.toLocaleDateString(localeMap[locale as keyof typeof localeMap] || "ru-RU", defaultOptions)
}

export const getLocalDateString = (date: Date | string): string => {
  const dateObj = typeof date === "string" ? convertUTCToLocal(date) : date
  return dateObj.toISOString().split("T")[0]
}

export const isSameDay = (date1: Date | string, date2: Date | string): boolean => {
  const d1 = typeof date1 === "string" ? convertUTCToLocal(date1) : date1
  const d2 = typeof date2 === "string" ? convertUTCToLocal(date2) : date2

  return d1.getFullYear() === d2.getFullYear() && d1.getMonth() === d2.getMonth() && d1.getDate() === d2.getDate()
}

export const isSameMonth = (date1: Date | string, date2: Date | string): boolean => {
  const d1 = typeof date1 === "string" ? convertUTCToLocal(date1) : date1
  const d2 = typeof date2 === "string" ? convertUTCToLocal(date2) : date2

  return d1.getFullYear() === d2.getFullYear() && d1.getMonth() === d2.getMonth()
}
