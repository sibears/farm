"use client"

import dynamic from "next/dynamic"
import Image from "next/image"
import { useEffect, useState } from "react"
import { Alert, AlertDescription } from "@/components/ui/alert"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { useFilteredFlags } from "@/hooks/useFilteredFlags"
import { useFlags } from "@/hooks/useFlags"
import type { FlagType } from "@/lib/types"
import { FlagFilters } from "./dashboard/FlagFilters"
import { FlagPagination } from "./dashboard/FlagPagination"
import { FlagStats } from "./dashboard/FlagStats"
import { FlagSubmitForm } from "./dashboard/FlagSubmitForm"
import { FlagTable } from "./dashboard/FlagTable"

const FlagDetailsModal = dynamic(
  () => import("@/components/flag-details-modal").then((mod) => ({ default: mod.FlagDetailsModal })),
  { loading: () => <div className="text-center font-mono">Loading...</div> },
)

type SortField = "id" | "flag" | "team" | "sploit" | "status" | "checksystem_response" | "created_time"
type SortDirection = "asc" | "desc"

export function FlagDashboard() {
  const [activeTab, setActiveTab] = useState<"flags" | "statistics">("flags")
  const [selectedFlag, setSelectedFlag] = useState<FlagType | null>(null)

  // Фильтры и поиск
  const [searchTerm, setSearchTerm] = useState("")
  const [statusFilter, setStatusFilter] = useState<string>("all")
  const [teamFilter, setTeamFilter] = useState<string>("all")
  const [currentPage, setCurrentPage] = useState(1)
  const [itemsPerPage, setItemsPerPage] = useState(10)
  const [sortField, setSortField] = useState<SortField>("created_time")
  const [sortDirection, setSortDirection] = useState<SortDirection>("desc")

  // Загрузка данных через React Query
  const { data: flags = [], isLoading, error, refetch } = useFlags()

  // Фильтрация и пагинация
  const {
    flags: filteredFlags,
    total,
    uniqueTeams,
  } = useFilteredFlags(flags, searchTerm, statusFilter, teamFilter, sortField, sortDirection, currentPage, itemsPerPage)

  // Сброс страницы при изменении фильтров
  useEffect(() => {
    setCurrentPage(1)
  }, [])

  const handleSort = (field: SortField) => {
    if (sortField === field) {
      setSortDirection(sortDirection === "asc" ? "desc" : "asc")
    } else {
      setSortField(field)
      setSortDirection("asc")
    }
  }

  const handleLogout = async () => {
    try {
      await fetch("/api/auth/logout", { method: "POST" })
    } catch (error) {
      console.error("Logout error:", error)
    } finally {
      window.location.reload()
    }
  }

  const totalPages = Math.max(1, Math.ceil(total / itemsPerPage))
  const startIndex = (currentPage - 1) * itemsPerPage
  const showingFrom = total === 0 ? 0 : startIndex + 1
  const showingTo = Math.min(startIndex + itemsPerPage, total)

  if (isLoading) {
    return (
      <div className="min-h-screen bg-background flex items-center justify-center">
        <div className="text-center space-y-4">
          <div className="w-8 h-8 border-2 border-primary border-t-transparent rounded-full animate-spin mx-auto" />
          <p className="font-mono text-muted-foreground">Loading flag database...</p>
        </div>
      </div>
    )
  }

  return (
    <div className="min-h-screen bg-background p-6">
      <div className="max-w-7xl mx-auto space-y-6">
        {/* Заголовок */}
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-4">
            <Image
              src="/images/sibears-paw.jpg"
              alt="SiBears"
              width={48}
              height={48}
              className="object-contain red-glow"
            />
            <div>
              <h1 className="text-3xl font-bold font-mono text-foreground">SiBears Farm Dashboard</h1>
              <p className="text-muted-foreground font-mono">Flag Submission System</p>
            </div>
          </div>
          <div className="flex items-center space-x-3">
            <Button
              variant="outline"
              onClick={() => refetch()}
              className="font-mono border-border hover:bg-muted bg-transparent"
            >
              <span className="mr-2">↻</span>
              Refresh
            </Button>
            <Button
              variant="outline"
              onClick={handleLogout}
              className="font-mono border-destructive text-destructive hover:bg-destructive hover:text-destructive-foreground bg-transparent"
            >
              <span className="mr-2">→</span>
              Logout
            </Button>
          </div>
        </div>

        {/* Ошибка загрузки */}
        {error && (
          <Alert className="border-destructive bg-destructive/10">
            <AlertDescription className="font-mono text-destructive text-sm">
              Failed to load flags: {error.message}
            </AlertDescription>
          </Alert>
        )}

        {/* Форма отправки флага */}
        <FlagSubmitForm />

        {/* Табы */}
        <div className="flex items-center space-x-2 border-b border-border">
          <Button
            variant="ghost"
            onClick={() => setActiveTab("flags")}
            className={`font-mono rounded-none border-b-2 ${
              activeTab === "flags"
                ? "border-primary text-primary"
                : "border-transparent text-muted-foreground hover:text-primary hover:border-primary/50"
            }`}
          >
            Flags
          </Button>
          <Button
            variant="ghost"
            onClick={() => setActiveTab("statistics")}
            className={`font-mono rounded-none border-b-2 ${
              activeTab === "statistics"
                ? "border-primary text-primary"
                : "border-transparent text-muted-foreground hover:text-primary hover:border-primary/50"
            }`}
          >
            Statistics
          </Button>
        </div>

        {/* Вкладка статистики */}
        {activeTab === "statistics" && <FlagStats />}

        {/* Вкладка флагов */}
        {activeTab === "flags" && (
          <Card className="bg-card border-border red-glow">
            <CardHeader>
              <div className="flex items-center justify-between">
                <div>
                  <CardTitle className="font-mono text-foreground">Flag Submissions</CardTitle>
                  <CardDescription className="font-mono text-muted-foreground">
                    Search, filter, and manage flag submissions
                  </CardDescription>
                </div>
                <Badge variant="outline" className="font-mono">
                  {total} total
                </Badge>
              </div>
            </CardHeader>
            <CardContent className="space-y-4">
              <FlagFilters
                searchTerm={searchTerm}
                onSearchChange={setSearchTerm}
                statusFilter={statusFilter}
                onStatusChange={setStatusFilter}
                teamFilter={teamFilter}
                onTeamChange={setTeamFilter}
                uniqueTeams={uniqueTeams}
                itemsPerPage={itemsPerPage}
                onItemsPerPageChange={setItemsPerPage}
              />

              <FlagTable
                flags={filteredFlags}
                onFlagClick={setSelectedFlag}
                sortField={sortField}
                sortDirection={sortDirection}
                onSort={handleSort}
              />

              <FlagPagination
                currentPage={currentPage}
                totalPages={totalPages}
                onPageChange={setCurrentPage}
                showingFrom={showingFrom}
                showingTo={showingTo}
                totalCount={total}
                isAtEnd={currentPage === totalPages}
              />
            </CardContent>
          </Card>
        )}
      </div>

      <FlagDetailsModal flag={selectedFlag} isOpen={!!selectedFlag} onClose={() => setSelectedFlag(null)} />
    </div>
  )
}
