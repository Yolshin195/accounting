"use client"

import type React from "react"

import { useState } from "react"
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
import { createCategory } from "@/lib/api"
import { Loader2 } from "lucide-react"

interface Category {
  id: string
  code: string
  name: string
  type: "INCOME" | "EXPENSE"
  description?: string
}

interface CategoryModalProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onCategoryCreated: (category: Category) => void
}

export function CategoryModal({ open, onOpenChange, onCategoryCreated }: CategoryModalProps) {
  const [code, setCode] = useState("")
  const [name, setName] = useState("")
  const [type, setType] = useState<"INCOME" | "EXPENSE">("EXPENSE")
  const [description, setDescription] = useState("")
  const [loading, setLoading] = useState(false)
  const { toast } = useToast()
  const { t } = useLocale()

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setLoading(true)

    try {
      const categoryData = {
        code,
        name,
        type,
        description: description || undefined,
      }

      const newCategory = await createCategory(categoryData)
      onCategoryCreated(newCategory)

      // Reset form
      setCode("")
      setName("")
      setType("EXPENSE")
      setDescription("")

      toast({
        title: t("categories.createSuccess"),
        description: t("categories.createSuccess"),
      })
    } catch (error: any) {
      toast({
        title: t("errors.unknownError"),
        description: error.message || t("categories.createError"),
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
          <DialogTitle>{t("categories.addCategoryTitle")}</DialogTitle>
          <DialogDescription>{t("categories.addCategoryDescription")}</DialogDescription>
        </DialogHeader>
        <form onSubmit={handleSubmit}>
          <div className="space-y-4 py-4">
            <div className="space-y-2">
              <Label htmlFor="code">{t("categories.categoryCode")}</Label>
              <Input
                id="code"
                value={code}
                onChange={(e) => setCode(e.target.value)}
                placeholder={t("categories.categoryCodePlaceholder")}
                required
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="name">{t("categories.categoryName")}</Label>
              <Input
                id="name"
                value={name}
                onChange={(e) => setName(e.target.value)}
                placeholder={t("categories.categoryNamePlaceholder")}
                required
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="type">{t("categories.categoryType")}</Label>
              <Select value={type} onValueChange={(value: "INCOME" | "EXPENSE") => setType(value)}>
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="EXPENSE">{t("common.expense")}</SelectItem>
                  <SelectItem value="INCOME">{t("common.income")}</SelectItem>
                </SelectContent>
              </Select>
            </div>
            <div className="space-y-2">
              <Label htmlFor="description">{t("categories.categoryDescription")}</Label>
              <Textarea
                id="description"
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                placeholder={t("categories.categoryDescriptionPlaceholder")}
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
              {t("common.create")}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
