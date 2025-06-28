"use client"

import type React from "react"

import { useState, useEffect } from "react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Textarea } from "@/components/ui/textarea"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import { useToast } from "@/hooks/use-toast"
import { useLocale } from "@/contexts/locale-context"
import { createIncomeTransaction, createExpenseTransaction, getCategories } from "@/lib/api"
import { formatDateForLocale } from "@/lib/date-utils"
import { Loader2 } from "lucide-react"

interface Transaction {
  id: string
  amount: number
  description: string
  category: string
  type: "INCOME" | "EXPENSE"
  date: string
}

interface Category {
  id: string
  code: string
  name: string
  type: "INCOME" | "EXPENSE"
}

interface TransactionModalProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  type: "INCOME" | "EXPENSE"
  selectedDate: Date | null
  onTransactionCreated: (transaction: Transaction) => void
}

export function TransactionModal({
  open,
  onOpenChange,
  type,
  selectedDate,
  onTransactionCreated,
}: TransactionModalProps) {
  const [amount, setAmount] = useState("")
  const [description, setDescription] = useState("")
  const [categoryCode, setCategoryCode] = useState("")
  const [categories, setCategories] = useState<Category[]>([])
  const [loading, setLoading] = useState(false)
  const { toast } = useToast()
  const { t, locale } = useLocale()

  useEffect(() => {
    if (open) {
      loadCategories()
    }
  }, [open, type])

  const loadCategories = async () => {
    try {
      const response = await getCategories(0, 100)
      const allCategories = response.content || response
      const filteredCategories = allCategories.filter((cat: Category) => cat.type === type)
      setCategories(filteredCategories)
    } catch (error: any) {
      toast({
        title: t("errors.unknownError"),
        description: error.message || t("transactions.loadCategoriesError"),
        variant: "destructive",
      })
    }
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setLoading(true)

    try {
      const transactionData: any = {
        amount: Number.parseFloat(amount),
        category: categoryCode,
      }

      // Добавляем описание только если оно заполнено
      if (description.trim()) {
        transactionData.description = description.trim()
      }

      // Добавляем дату только если она не сегодняшняя или если дата была выбрана
      const today = new Date().toISOString().split("T")[0]
      const transactionDate = selectedDate ? selectedDate.toISOString().split("T")[0] : today

      if (transactionDate !== today) {
        transactionData.date = transactionDate
      }

      let newTransaction
      if (type === "INCOME") {
        newTransaction = await createIncomeTransaction(transactionData)
      } else {
        newTransaction = await createExpenseTransaction(transactionData)
      }

      onTransactionCreated({ ...newTransaction, type })

      // Reset form
      setAmount("")
      setDescription("")
      setCategoryCode("")

      toast({
        title: t("transactions.createSuccess"),
        description: t("transactions.createSuccess"),
      })
    } catch (error: any) {
      toast({
        title: t("errors.unknownError"),
        description: error.message || t("transactions.createError"),
        variant: "destructive",
      })
    } finally {
      setLoading(false)
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>
            {type === "INCOME" ? t("transactions.addIncomeTitle") : t("transactions.addExpenseTitle")}
          </DialogTitle>
          <DialogDescription>
            {selectedDate && `${t("common.date")}: ${formatDateForLocale(selectedDate, locale)}`}
          </DialogDescription>
        </DialogHeader>
        <form onSubmit={handleSubmit}>
          <div className="space-y-4 py-4">
            <div className="space-y-2">
              <Label htmlFor="amount">{t("common.amount")}</Label>
              <Input
                id="amount"
                type="number"
                step="0.01"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                placeholder="0.00"
                required
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="category">{t("common.category")}</Label>
              <Select value={categoryCode} onValueChange={setCategoryCode} required>
                <SelectTrigger>
                  <SelectValue placeholder={t("transactions.selectCategory")} />
                </SelectTrigger>
                <SelectContent>
                  {categories.map((category) => (
                    <SelectItem key={category.code} value={category.code}>
                      {category.name} ({category.code})
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <div className="space-y-2">
              <Label htmlFor="description">
                {t("common.description")} ({t("common.optional")})
              </Label>
              <Textarea
                id="description"
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                placeholder={t("transactions.transactionDescription")}
                rows={3}
              />
            </div>
          </div>
          <DialogFooter>
            <Button type="button" variant="outline" onClick={() => onOpenChange(false)}>
              {t("common.cancel")}
            </Button>
            <Button type="submit" disabled={loading}>
              {loading && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
              {t("common.add")}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
