"use client"

import { useState, useEffect } from "react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { ChevronLeft, ChevronRight, CalendarIcon } from "lucide-react"
import { TransactionModal } from "@/components/transaction-modal"
import { DayTransactionsModal } from "@/components/day-transactions-modal"
import { useToast } from "@/hooks/use-toast"
import { useLocale } from "@/contexts/locale-context"
import { getTransactions } from "@/lib/api"
import { isSameDay, isSameMonth } from "@/lib/date-utils"

interface Transaction {
  id: string
  amount: number
  description?: string
  category: string
  type: "INCOME" | "EXPENSE"
  date: string
}

export default function CalendarPage() {
  const [currentDate, setCurrentDate] = useState(new Date())
  const [transactions, setTransactions] = useState<Transaction[]>([])
  const [selectedDate, setSelectedDate] = useState<Date | null>(null)
  const [transactionModalOpen, setTransactionModalOpen] = useState(false)
  const [transactionType, setTransactionType] = useState<"INCOME" | "EXPENSE">("EXPENSE")
  const [dayModalOpen, setDayModalOpen] = useState(false)
  const [loading, setLoading] = useState(false)
  const { toast } = useToast()
  const { t, locale } = useLocale()

  const formatCurrency = (amount: number) => {
    const localeMap = {
      en: "en-US",
      th: "th-TH",
      ru: "ru-RU",
    }
    return amount.toLocaleString(localeMap[locale] || "ru-RU")
  }

  useEffect(() => {
    loadTransactions()
  }, [currentDate])

  const loadTransactions = async () => {
    try {
      setLoading(true)
      // Загружаем транзакции за текущий месяц
      const response = await getTransactions(0, 100) // Загружаем больше для фильтрации
      const monthTransactions = (response.content || response).filter((t: Transaction) => {
        // Конвертируем UTC дату в локальную и сравниваем месяц
        return isSameMonth(t.date, currentDate)
      })
      setTransactions(monthTransactions)
    } catch (error: any) {
      toast({
        title: t("errors.unknownError"),
        description: error.message || t("calendar.loadError"),
        variant: "destructive",
      })
    } finally {
      setLoading(false)
    }
  }

  const getDaysInMonth = (date: Date) => {
    const year = date.getFullYear()
    const month = date.getMonth()
    const firstDay = new Date(year, month, 1)
    const lastDay = new Date(year, month + 1, 0)
    const daysInMonth = lastDay.getDate()
    const startingDayOfWeek = firstDay.getDay()

    const days = []

    // Add empty cells for days before the first day of the month
    for (let i = 0; i < startingDayOfWeek; i++) {
      days.push(null)
    }

    // Add days of the month
    for (let day = 1; day <= daysInMonth; day++) {
      days.push(new Date(year, month, day))
    }

    return days
  }

  const getTransactionsForDate = (date: Date) => {
    return transactions.filter((t) => isSameDay(t.date, date))
  }

  const getTotalForDate = (date: Date) => {
    const dayTransactions = getTransactionsForDate(date)
    return dayTransactions.reduce((total, t) => {
      return total + (t.type === "INCOME" ? t.amount : -t.amount)
    }, 0)
  }

  const handleDateClick = (date: Date) => {
    setSelectedDate(date)
    setDayModalOpen(true)
  }

  const handleAddTransaction = (type: "INCOME" | "EXPENSE") => {
    setTransactionType(type)
    setTransactionModalOpen(true)
  }

  const handleTransactionCreated = (newTransaction: Transaction) => {
    setTransactions([...transactions, newTransaction])
    setTransactionModalOpen(false)
  }

  const handleTransactionUpdated = (updatedTransaction: Transaction) => {
    setTransactions(transactions.map((t) => (t.id === updatedTransaction.id ? updatedTransaction : t)))
  }

  const handleTransactionDeleted = (transactionId: string) => {
    setTransactions(transactions.filter((t) => t.id !== transactionId))
  }

  const navigateMonth = (direction: "prev" | "next") => {
    setCurrentDate((prev) => {
      const newDate = new Date(prev)
      if (direction === "prev") {
        newDate.setMonth(prev.getMonth() - 1)
      } else {
        newDate.setMonth(prev.getMonth() + 1)
      }
      return newDate
    })
  }

  const days = getDaysInMonth(currentDate)

  // Локализованные названия месяцев и дней
  const monthNames = [
    t("months.january"),
    t("months.february"),
    t("months.march"),
    t("months.april"),
    t("months.may"),
    t("months.june"),
    t("months.july"),
    t("months.august"),
    t("months.september"),
    t("months.october"),
    t("months.november"),
    t("months.december"),
  ]

  const dayNames = [
    t("days.sunday"),
    t("days.monday"),
    t("days.tuesday"),
    t("days.wednesday"),
    t("days.thursday"),
    t("days.friday"),
    t("days.saturday"),
  ]

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold">{t("calendar.title")}</h1>
          <p className="text-muted-foreground">{t("calendar.description")}</p>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" onClick={() => handleAddTransaction("EXPENSE")}>
            {t("calendar.addExpense")}
          </Button>
          <Button onClick={() => handleAddTransaction("INCOME")}>{t("calendar.addIncome")}</Button>
        </div>
      </div>

      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle className="flex items-center">
              <CalendarIcon className="mr-2 h-5 w-5" />
              {monthNames[currentDate.getMonth()]} {currentDate.getFullYear()}
            </CardTitle>
            <div className="flex gap-2">
              <Button variant="outline" size="sm" onClick={() => navigateMonth("prev")}>
                <ChevronLeft className="h-4 w-4" />
              </Button>
              <Button variant="outline" size="sm" onClick={() => navigateMonth("next")}>
                <ChevronRight className="h-4 w-4" />
              </Button>
            </div>
          </div>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-7 gap-1 mb-4">
            {dayNames.map((day) => (
              <div key={day} className="p-2 text-center text-sm font-medium text-muted-foreground">
                {day}
              </div>
            ))}
          </div>
          <div className="grid grid-cols-7 gap-1">
            {days.map((date, index) => {
              if (!date) {
                return <div key={index} className="p-2 h-20" />
              }

              const dayTransactions = getTransactionsForDate(date)
              const total = getTotalForDate(date)
              const isToday = isSameDay(new Date(), date)

              return (
                <div
                  key={date.toISOString()}
                  className={`
                    p-2 h-20 border rounded-lg cursor-pointer hover:bg-gray-50 transition-colors
                    ${isToday ? "bg-blue-50 border-blue-200" : "border-gray-200"}
                  `}
                  onClick={() => handleDateClick(date)}
                >
                  <div className="text-sm font-medium mb-1">{date.getDate()}</div>
                  {dayTransactions.length > 0 && (
                    <div className="space-y-1">
                      <div className="text-xs text-muted-foreground">{dayTransactions.length} тр.</div>
                      <div className={`text-xs font-medium ${total >= 0 ? "text-green-600" : "text-red-600"}`}>
                        {total >= 0 ? "+" : ""}
                        {formatCurrency(total)} ₽
                      </div>
                    </div>
                  )}
                </div>
              )
            })}
          </div>
        </CardContent>
      </Card>

      <TransactionModal
        open={transactionModalOpen}
        onOpenChange={setTransactionModalOpen}
        type={transactionType}
        selectedDate={selectedDate}
        onTransactionCreated={handleTransactionCreated}
      />

      <DayTransactionsModal
        open={dayModalOpen}
        onOpenChange={setDayModalOpen}
        date={selectedDate}
        transactions={selectedDate ? getTransactionsForDate(selectedDate) : []}
        onAddTransaction={handleAddTransaction}
        onTransactionUpdated={handleTransactionUpdated}
        onTransactionDeleted={handleTransactionDeleted}
      />
    </div>
  )
}
