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
import { createIncomeTransaction, createExpenseTransaction, getCategories } from "@/lib/api"
import { Loader2 } from "lucide-react"

interface Transaction {
  id: string
  amount: number
  description: string
  categoryCode: string
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
        title: "Ошибка",
        description: error.message || "Не удалось загрузить категории",
        variant: "destructive",
      })
    }
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setLoading(true)

    try {
      const transactionData = {
        amount: Number.parseFloat(amount),
        description,
        categoryCode,
        date: selectedDate ? selectedDate.toISOString().split("T")[0] : new Date().toISOString().split("T")[0],
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
        title: "Успешно",
        description: `${type === "INCOME" ? "Доход" : "Расход"} добавлен`,
      })
    } catch (error: any) {
      toast({
        title: "Ошибка",
        description: error.message || "Не удалось создать транзакцию",
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
          <DialogTitle>Добавить {type === "INCOME" ? "доход" : "расход"}</DialogTitle>
          <DialogDescription>{selectedDate && `Дата: ${selectedDate.toLocaleDateString("ru-RU")}`}</DialogDescription>
        </DialogHeader>
        <form onSubmit={handleSubmit}>
          <div className="space-y-4 py-4">
            <div className="space-y-2">
              <Label htmlFor="amount">Сумма</Label>
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
              <Label htmlFor="category">Категория</Label>
              <Select value={categoryCode} onValueChange={setCategoryCode} required>
                <SelectTrigger>
                  <SelectValue placeholder="Выберите категорию" />
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
              <Label htmlFor="description">Описание</Label>
              <Textarea
                id="description"
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                placeholder="Описание транзакции..."
                rows={3}
                required
              />
            </div>
          </div>
          <DialogFooter>
            <Button type="button" variant="outline" onClick={() => onOpenChange(false)}>
              Отмена
            </Button>
            <Button type="submit" disabled={loading}>
              {loading && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
              Добавить
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
