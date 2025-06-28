"use client"

import { useState, useEffect } from "react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Plus, Trash2, Tag, Loader2 } from "lucide-react"
import { CategoryModal } from "@/components/category-modal"
import { useToast } from "@/hooks/use-toast"
import { useLocale } from "@/contexts/locale-context"
import { getCategories, deleteCategory } from "@/lib/api"

interface Category {
  id: string
  code: string
  name: string
  type: "INCOME" | "EXPENSE"
  description?: string
}

export default function CategoriesPage() {
  const [categories, setCategories] = useState<Category[]>([])
  const [loading, setLoading] = useState(true)
  const [modalOpen, setModalOpen] = useState(false)
  const [page, setPage] = useState(0)
  const [hasMore, setHasMore] = useState(true)
  const { toast } = useToast()
  const { t } = useLocale()

  useEffect(() => {
    loadCategories()
  }, [])

  const loadCategories = async (pageNum = 0) => {
    try {
      setLoading(true)
      const response = await getCategories(pageNum, 10)

      if (pageNum === 0) {
        setCategories(response.content || response)
      } else {
        setCategories((prev) => [...prev, ...(response.content || response)])
      }

      setHasMore(response.hasNext || false)
      setPage(pageNum)
    } catch (error: any) {
      toast({
        title: t("errors.unknownError"),
        description: error.message || t("categories.loadError"),
        variant: "destructive",
      })
    } finally {
      setLoading(false)
    }
  }

  const handleDeleteCategory = async (code: string) => {
    try {
      await deleteCategory(code)
      setCategories(categories.filter((cat) => cat.code !== code))
      toast({
        title: t("categories.deleteSuccess"),
        description: t("categories.deleteSuccess"),
      })
    } catch (error: any) {
      toast({
        title: t("errors.unknownError"),
        description: error.message || t("categories.deleteError"),
        variant: "destructive",
      })
    }
  }

  const handleCategoryCreated = (newCategory: Category) => {
    setCategories([newCategory, ...categories])
    setModalOpen(false)
  }

  const loadMore = () => {
    if (!loading && hasMore) {
      loadCategories(page + 1)
    }
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold">{t("categories.title")}</h1>
          <p className="text-muted-foreground">{t("categories.description")}</p>
        </div>
        <Button onClick={() => setModalOpen(true)}>
          <Plus className="mr-2 h-4 w-4" />
          {t("categories.addCategory")}
        </Button>
      </div>

      {loading && categories.length === 0 ? (
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
          {[...Array(6)].map((_, i) => (
            <Card key={i} className="animate-pulse">
              <CardHeader className="pb-3">
                <div className="h-4 bg-gray-200 rounded w-3/4"></div>
              </CardHeader>
              <CardContent>
                <div className="h-3 bg-gray-200 rounded w-1/2"></div>
              </CardContent>
            </Card>
          ))}
        </div>
      ) : categories.length === 0 ? (
        <Card>
          <CardContent className="flex flex-col items-center justify-center py-12">
            <Tag className="h-12 w-12 text-muted-foreground mb-4" />
            <h3 className="text-lg font-semibold mb-2">{t("categories.noCategories")}</h3>
            <p className="text-muted-foreground text-center mb-4">{t("categories.noCategoriesDescription")}</p>
            <Button onClick={() => setModalOpen(true)}>
              <Plus className="mr-2 h-4 w-4" />
              {t("categories.addCategory")}
            </Button>
          </CardContent>
        </Card>
      ) : (
        <>
          <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
            {categories.map((category) => (
              <Card key={category.code} className="hover:shadow-md transition-shadow">
                <CardHeader className="pb-3">
                  <div className="flex items-center justify-between">
                    <CardTitle className="text-lg flex items-center">
                      <Tag className="mr-2 h-4 w-4" />
                      {category.name}
                    </CardTitle>
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => handleDeleteCategory(category.code)}
                      className="text-destructive hover:text-destructive"
                    >
                      <Trash2 className="h-4 w-4" />
                    </Button>
                  </div>
                </CardHeader>
                <CardContent className="space-y-2">
                  <Badge variant={category.type === "INCOME" ? "default" : "secondary"}>
                    {category.type === "INCOME" ? t("common.income") : t("common.expense")}
                  </Badge>
                  <p className="text-sm text-muted-foreground">
                    {t("common.code")}: {category.code}
                  </p>
                  {category.description && <p className="text-sm text-muted-foreground">{category.description}</p>}
                </CardContent>
              </Card>
            ))}
          </div>

          {hasMore && (
            <div className="flex justify-center">
              <Button variant="outline" onClick={loadMore} disabled={loading}>
                {loading && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
                {t("categories.loadMore")}
              </Button>
            </div>
          )}
        </>
      )}

      <CategoryModal open={modalOpen} onOpenChange={setModalOpen} onCategoryCreated={handleCategoryCreated} />
    </div>
  )
}
