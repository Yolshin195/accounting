"use client"

import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from "@/components/ui/dialog"
import { Badge } from "@/components/ui/badge"
import { Input } from "@/components/ui/input"
import { Textarea } from "@/components/ui/textarea"
import { Plus, TrendingUp, TrendingDown, Edit, Trash2, Save, X } from "lucide-react"
import { useToast } from "@/hooks/use-toast"
import { updateTransaction, deleteTransaction } from "@/lib/api"

interface Transaction {
  id: string
  amount: number
  description: string
  categoryCode: string
  type: "INCOME" | "EXPENSE"
  date: string
}

interface DayTransactionsModalProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  date: Date | null
  transactions: Transaction[]
  onAddTransaction: (type: "INCOME" | "EXPENSE") => void
  onTransactionUpdated: (transaction: Transaction) => void
  onTransactionDeleted: (transactionId: string) => void
}

export function DayTransactionsModal({
  open,
  onOpenChange,
  date,
  transactions,
  onAddTransaction,
  onTransactionUpdated,
  onTransactionDeleted,
}: DayTransactionsModalProps) {
  const [editingId, setEditingId] = useState<string | null>(null)
  const [editAmount, setEditAmount] = useState("")
  const [editDescription, setEditDescription] = useState("")
  const { toast } = useToast()

  if (!date) return null

  const total = transactions.reduce((sum, t) => {
    return sum + (t.type === "INCOME" ? t.amount : -t.amount)
  }, 0)

  const income = transactions.filter((t) => t.type === "INCOME").reduce((sum, t) => sum + t.amount, 0)
  const expense = transactions.filter((t) => t.type === "EXPENSE").reduce((sum, t) => sum + t.amount, 0)

  const startEdit = (transaction: Transaction) => {
    setEditingId(transaction.id)
    setEditAmount(transaction.amount.toString())
    setEditDescription(transaction.description)
  }

  const cancelEdit = () => {
    setEditingId(null)
    setEditAmount("")
    setEditDescription("")
  }

  const saveEdit = async (transaction: Transaction) => {
    try {
      const updatedData = {
        amount: Number.parseFloat(editAmount),
        description: editDescription,
        categoryCode: transaction.categoryCode,
        date: transaction.date,
      }

      const updatedTransaction = await updateTransaction(transaction.id, updatedData)
      onTransactionUpdated({ ...updatedTransaction, type: transaction.type })
      setEditingId(null)

      toast({
        title: "Успешно",
        description: "Транзакция обновлена",
      })
    } catch (error: any) {
      toast({
        title: "Ошибка",
        description: error.message || "Не удалось обновить транзакцию",
        variant: "destructive",
      })
    }
  }

  const handleDelete = async (transactionId: string) => {
    try {
      await deleteTransaction(transactionId)
      onTransactionDeleted(transactionId)

      toast({
        title: "Успешно",
        description: "Транзакция удалена",
      })
    } catch (error: any) {
      toast({
        title: "Ошибка",
        description: error.message || "Не удалось удалить транзакцию",
        variant: "destructive",
      })
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-lg max-h-[80vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>
            {date.toLocaleDateString("ru-RU", {
              weekday: "long",
              year: "numeric",
              month: "long",
              day: "numeric",
            })}
          </DialogTitle>
          <DialogDescription>Транзакции за день</DialogDescription>
        </DialogHeader>

        <div className="space-y-4">
          {/* Summary */}
          <div className="grid grid-cols-3 gap-4 p-4 bg-gray-50 rounded-lg">
            <div className="text-center">
              <div className="text-sm text-muted-foreground">Доходы</div>
              <div className="text-lg font-semibold text-green-600">+{income.toLocaleString("ru-RU")} ₽</div>
            </div>
            <div className="text-center">
              <div className="text-sm text-muted-foreground">Расходы</div>
              <div className="text-lg font-semibold text-red-600">-{expense.toLocaleString("ru-RU")} ₽</div>
            </div>
            <div className="text-center">
              <div className="text-sm text-muted-foreground">Итого</div>
              <div className={`text-lg font-semibold ${total >= 0 ? "text-green-600" : "text-red-600"}`}>
                {total >= 0 ? "+" : ""}
                {total.toLocaleString("ru-RU")} ₽
              </div>
            </div>
          </div>

          {/* Add buttons */}
          <div className="flex gap-2">
            <Button
              variant="outline"
              className="flex-1 bg-transparent"
              onClick={() => {
                onAddTransaction("EXPENSE")
                onOpenChange(false)
              }}
            >
              <Plus className="mr-2 h-4 w-4" />
              Добавить расход
            </Button>
            <Button
              className="flex-1"
              onClick={() => {
                onAddTransaction("INCOME")
                onOpenChange(false)
              }}
            >
              <Plus className="mr-2 h-4 w-4" />
              Добавить доход
            </Button>
          </div>

          {/* Transactions list */}
          <div className="space-y-2">
            {transactions.length === 0 ? (
              <div className="text-center py-8 text-muted-foreground">Нет транзакций за этот день</div>
            ) : (
              transactions.map((transaction) => (
                <div key={transaction.id} className="border rounded-lg p-3">
                  {editingId === transaction.id ? (
                    <div className="space-y-3">
                      <div className="flex items-center space-x-3">
                        <div
                          className={`p-1 rounded-full ${transaction.type === "INCOME" ? "bg-green-100" : "bg-red-100"}`}
                        >
                          {transaction.type === "INCOME" ? (
                            <TrendingUp className="h-4 w-4 text-green-600" />
                          ) : (
                            <TrendingDown className="h-4 w-4 text-red-600" />
                          )}
                        </div>
                        <Badge variant="secondary" className="text-xs">
                          {transaction.categoryCode}
                        </Badge>
                      </div>
                      <div className="space-y-2">
                        <Input
                          type="number"
                          step="0.01"
                          value={editAmount}
                          onChange={(e) => setEditAmount(e.target.value)}
                          placeholder="Сумма"
                        />
                        <Textarea
                          value={editDescription}
                          onChange={(e) => setEditDescription(e.target.value)}
                          placeholder="Описание"
                          rows={2}
                        />
                      </div>
                      <div className="flex gap-2">
                        <Button size="sm" onClick={() => saveEdit(transaction)}>
                          <Save className="h-3 w-3 mr-1" />
                          Сохранить
                        </Button>
                        <Button size="sm" variant="outline" onClick={cancelEdit}>
                          <X className="h-3 w-3 mr-1" />
                          Отмена
                        </Button>
                      </div>
                    </div>
                  ) : (
                    <div className="flex items-center justify-between">
                      <div className="flex items-center space-x-3">
                        <div
                          className={`p-1 rounded-full ${transaction.type === "INCOME" ? "bg-green-100" : "bg-red-100"}`}
                        >
                          {transaction.type === "INCOME" ? (
                            <TrendingUp className="h-4 w-4 text-green-600" />
                          ) : (
                            <TrendingDown className="h-4 w-4 text-red-600" />
                          )}
                        </div>
                        <div>
                          <div className="font-medium">{transaction.description}</div>
                          <Badge variant="secondary" className="text-xs">
                            {transaction.categoryCode}
                          </Badge>
                        </div>
                      </div>
                      <div className="flex items-center gap-2">
                        <div
                          className={`font-semibold ${transaction.type === "INCOME" ? "text-green-600" : "text-red-600"}`}
                        >
                          {transaction.type === "INCOME" ? "+" : "-"}
                          {transaction.amount.toLocaleString("ru-RU")} ₽
                        </div>
                        <div className="flex gap-1">
                          <Button size="sm" variant="ghost" onClick={() => startEdit(transaction)}>
                            <Edit className="h-3 w-3" />
                          </Button>
                          <Button size="sm" variant="ghost" onClick={() => handleDelete(transaction.id)}>
                            <Trash2 className="h-3 w-3" />
                          </Button>
                        </div>
                      </div>
                    </div>
                  )}
                </div>
              ))
            )}
          </div>
        </div>
      </DialogContent>
    </Dialog>
  )
}
