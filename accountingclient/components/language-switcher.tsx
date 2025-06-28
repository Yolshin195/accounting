"use client"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { useLocale } from "@/contexts/locale-context"
import { Languages } from "lucide-react"

const languages = [
  { code: "en", name: "English", flag: "🇺🇸" },
  { code: "th", name: "ไทย", flag: "🇹🇭" },
  { code: "ru", name: "Русский", flag: "🇷🇺" },
]

export function LanguageSwitcher() {
  const { locale, setLocale } = useLocale()

  return (
    <Select value={locale} onValueChange={(value: "en" | "th" | "ru") => setLocale(value)}>
      <SelectTrigger className="w-[140px]">
        <Languages className="h-4 w-4 mr-2" />
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        {languages.map((lang) => (
          <SelectItem key={lang.code} value={lang.code}>
            <div className="flex items-center">
              <span className="mr-2">{lang.flag}</span>
              {lang.name}
            </div>
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  )
}
