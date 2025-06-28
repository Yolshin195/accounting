"use client"

import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from "@/components/ui/dialog"
import { Badge } from "@/components/ui/badge"
import { Input } from "@/components/ui/input"
import { Textarea } from "@/components/ui/textarea"
import { Plus, TrendingUp, TrendingDown, Edit, Trash2, Save, X } from "lucide-react"
import { useToast } from "@/hooks/use-toast"
import { useLocale } from "@/contexts/locale-context"
import { updateTransaction, deleteTransaction } from "@/lib/api"
import { formatDateForLocale } from "@/lib/date-utils"

interface Transaction {
  id: string
  amount: number
  description?: string
  category: string
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
  const { t, locale } = useLocale()

  const localeMap = {
    en: "en-US",
    th: "th-TH",
    ru: "ru-RU",
  }

  if (!date) return null

  const total = transactions.reduce((sum, t) => {
    return sum + (t.type === "INCOME" ? t.amount : -t.amount)
  }, 0)

  const income = transactions.filter((t) => t.type === "INCOME").reduce((sum, t) => sum + t.amount, 0)
  const expense = transactions.filter((t) => t.type === "EXPENSE").reduce((sum, t) => sum + t.amount, 0)

  const startEdit = (transaction: Transaction) => {
    setEditingId(transaction.id)
    setEditAmount(transaction.amount.toString())
    setEditDescription(transaction.description || "")
  }

  const cancelEdit = () => {
    setEditingId(null)
    setEditAmount("")
    setEditDescription("")
  }

  const saveEdit = async (transaction: Transaction) => {
    try {
      const updatedData: any = {
        amount: Number.parseFloat(editAmount),
        category: transaction.category,
        date: transaction.date,
      }

      // Добавляем описание только если оно заполнено
      if (editDescription.trim()) {
        updatedData.description = editDescription.trim()
      }

      const updatedTransaction = await updateTransaction(transaction.id, updatedData)
      onTransactionUpdated({ ...updatedTransaction, type: transaction.type })
      setEditingId(null)

      toast({
        title: t("transactions.updateSuccess"),
        description: t("transactions.updateSuccess"),
      })
    } catch (error: any) {
      toast({
        title: t("errors.unknownError"),
        description: error.message || t("transactions.updateError"),
        variant: "destructive",
      })
    }
  }

  const handleDelete = async (transactionId: string) => {
    try {
      await deleteTransaction(transactionId)
      onTransactionDeleted(transactionId)

      toast({
        title: t("transactions.deleteSuccess"),
        description: t("transactions.deleteSuccess"),
      })
    } catch (error: any) {
      toast({
        title: t("errors.unknownError"),
        description: error.message || t("transactions.deleteError"),
        variant: "destructive",
      })
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-lg max-h-[80vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>
            {formatDateForLocale(date, locale, {
              weekday: "long",
              year: "numeric",
              month: "long",
              day: "numeric",
            })}
          </DialogTitle>
          <DialogDescription>{t("calendar.transactionsForDay")}</DialogDescription>
        </DialogHeader>

        <div className="space-y-4">
          {/* Summary */}
          <div className="grid grid-cols-3 gap-4 p-4 bg-gray-50 rounded-lg">
            <div className="text-center">
              <div className="text-sm text-muted-foreground">{t("calendar.incomes")}</div>
              <div className="text-lg font-semibold text-green-600">
                +{income.toLocaleString(localeMap[locale] || "ru-RU")} ₽
              </div>
            </div>
            <div className="text-center">
              <div className="text-sm text-muted-foreground">{t("calendar.expenses")}</div>
              <div className="text-lg font-semibold text-red-600">
                -{expense.toLocaleString(localeMap[locale] || "ru-RU")} ₽
              </div>
            </div>
            <div className="text-center">
              <div className="text-sm text-muted-foreground">{t("common.total")}</div>
              <div className={`text-lg font-semibold ${total >= 0 ? "text-green-600" : "text-red-600"}`}>
                {total >= 0 ? "+" : ""}
                {total.toLocaleString(localeMap[locale] || "ru-RU")} ₽
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
              {t("calendar.addExpense")}
            </Button>
            <Button
              className="flex-1"
              onClick={() => {
                onAddTransaction("INCOME")
                onOpenChange(false)
              }}
            >
              <Plus className="mr-2 h-4 w-4" />
              {t("calendar.addIncome")}
            </Button>
          </div>

          {/* Transactions list */}
          <div className="space-y-2">
            {transactions.length === 0 ? (
              <div className="text-center py-8 text-muted-foreground">{t("calendar.noTransactions")}</div>
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
                          {transaction.category}
                        </Badge>
                      </div>
                      <div className="space-y-2">
                        <Input
                          type="number"
                          step="0.01"
                          value={editAmount}
                          onChange={(e) => setEditAmount(e.target.value)}
                          placeholder={t("common.amount")}
                        />
                        <Textarea
                          value={editDescription}
                          onChange={(e) => setEditDescription(e.target.value)}
                          placeholder={`${t("common.description")} (${t("common.optional")})`}
                          rows={2}
                        />
                      </div>
                      <div className="flex gap-2">
                        <Button size="sm" onClick={() => saveEdit(transaction)}>
                          <Save className="h-3 w-3 mr-1" />
                          {t("common.save")}
                        </Button>
                        <Button size="sm" variant="outline" onClick={cancelEdit}>
                          <X className="h-3 w-3 mr-1" />
                          {t("common.cancel")}
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
                          <div className="font-medium">
                            {transaction.description || `${transaction.category} ${t("transactions.transaction")}`}
                          </div>
                          <Badge variant="secondary" className="text-xs">
                            {transaction.category}
                          </Badge>
                        </div>
                      </div>
                      <div className="flex items-center gap-2">
                        <div
                          className={`font-semibold ${transaction.type === "INCOME" ? "text-green-600" : "text-red-600"}`}
                        >
                          {transaction.type === "INCOME" ? "+" : "-"}
                          {transaction.amount.toLocaleString(localeMap[locale] || "ru-RU")} ₽
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
