"use client"

import { useState, useEffect } from "react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { ChevronLeft, ChevronRight, CalendarIcon } from "lucide-react"
import { TransactionModal } from "@/components/transaction-modal"
import { DayTransactionsModal } from "@/components/day-transactions-modal"
import { useToast } from "@/hooks/use-toast"
import { getTransactions } from "@/lib/api"

interface Transaction {
  id: string
  amount: number
  description: string
  categoryCode: string
  type: "INCOME" | "EXPENSE"
  date: string
  createdAt: string
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

  useEffect(() => {
    loadTransactions()
  }, [currentDate])

  const loadTransactions = async () => {
    try {
      setLoading(true)
      // Загружаем транзакции за текущий месяц
      const response = await getTransactions(0, 100) // Загружаем больше для фильтрации
      const monthTransactions = (response.content || response).filter((t: Transaction) => {
        const transactionDate = new Date(t.date)
        return (
          transactionDate.getMonth() === currentDate.getMonth() &&
          transactionDate.getFullYear() === currentDate.getFullYear()
        )
      })
      setTransactions(monthTransactions)
    } catch (error: any) {
      toast({
        title: "Ошибка",
        description: error.message || "Не удалось загрузить транзакции",
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
    const dateStr = date.toISOString().split("T")[0]
    return transactions.filter((t) => t.date.startsWith(dateStr))
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
  const monthNames = [
    "Январь",
    "Февраль",
    "Март",
    "Апрель",
    "Май",
    "Июнь",
    "Июль",
    "Август",
    "Сентябрь",
    "Октябрь",
    "Ноябрь",
    "Декабрь",
  ]
  const dayNames = ["Вс", "Пн", "Вт", "Ср", "Чт", "Пт", "Сб"]

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold">Календарь</h1>
          <p className="text-muted-foreground">Отслеживайте доходы и расходы по дням</p>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" onClick={() => handleAddTransaction("EXPENSE")}>
            Добавить расход
          </Button>
          <Button onClick={() => handleAddTransaction("INCOME")}>Добавить доход</Button>
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
              const isToday = date.toDateString() === new Date().toDateString()

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
                        {total.toLocaleString("ru-RU")} ₽
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
