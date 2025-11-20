import { useMemo } from "react"
import type { FlagType } from "@/lib/types"

type SortField = "id" | "flag" | "team" | "sploit" | "status" | "checksystem_response" | "created_time"
type SortDirection = "asc" | "desc"

export function useFilteredFlags(
  flags: FlagType[],
  searchTerm: string,
  statusFilter: string,
  teamFilter: string,
  sortField: SortField,
  sortDirection: SortDirection,
  currentPage: number,
  itemsPerPage: number,
) {
  return useMemo(() => {
    let filtered = [...flags]

    // Поиск
    if (searchTerm) {
      const search = searchTerm.toLowerCase()
      filtered = filtered.filter(
        (flag) =>
          flag.flag.toLowerCase().includes(search) ||
          flag.team?.toLowerCase().includes(search) ||
          flag.sploit?.toLowerCase().includes(search),
      )
    }

    // Фильтр по статусу
    if (statusFilter !== "all") {
      filtered = filtered.filter((flag) => flag.status === statusFilter)
    }

    // Фильтр по команде
    if (teamFilter !== "all") {
      filtered = filtered.filter((flag) => flag.team === teamFilter)
    }

    // Сортировка
    // biome-ignore lint/complexity/noExcessiveCognitiveComplexity: complex sorting logic for different field types
    filtered.sort((a, b) => {
      let aVal: string | number | undefined = a[sortField as keyof FlagType]
      let bVal: string | number | undefined = b[sortField as keyof FlagType]

      if (sortField === "created_time") {
        aVal = aVal ? new Date(aVal).getTime() : 0
        bVal = bVal ? new Date(bVal).getTime() : 0
      }

      if (aVal === undefined || aVal === null) return 1
      if (bVal === undefined || bVal === null) return -1

      if (sortDirection === "asc") {
        return aVal > bVal ? 1 : -1
      } else {
        return aVal < bVal ? 1 : -1
      }
    })

    // Пагинация
    const total = filtered.length
    const startIndex = (currentPage - 1) * itemsPerPage
    const paginated = filtered.slice(startIndex, startIndex + itemsPerPage)

    // Уникальные команды
    const uniqueTeams = Array.from(new Set(filtered.map((f) => f.team).filter(Boolean))) as string[]

    return {
      flags: paginated,
      total,
      uniqueTeams,
    }
  }, [flags, searchTerm, statusFilter, teamFilter, sortField, sortDirection, currentPage, itemsPerPage])
}
