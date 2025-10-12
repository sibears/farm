"use client"

import { useState, useEffect, useMemo, useCallback } from "react"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { Input } from "@/components/ui/input"
import dynamic from "next/dynamic"
import Image from "next/image"
import { FlagStatus, type FlagType } from "@/lib/types"

const FlagDetailsModal = dynamic(
  () => import("@/components/flag-details-modal").then((mod) => ({ default: mod.FlagDetailsModal })),
  {
    loading: () => <div className="text-center font-mono">Loading...</div>,
  },
)

type SortField = "id" | "flag" | "team" | "sploit" | "status" | "checksystem_response" | "created_time"
type SortDirection = "asc" | "desc"

const MOCK_FLAGS: FlagType[] = [
  {
    id: 1,
    flag: "FLAG{test_flag_1_accepted}",
    sploit: "web_exploit_v1",
    team: "Team Alpha",
    created_time: new Date(Date.now() - 3600000).toISOString(),
    start_waiting_time: new Date(Date.now() - 3500000).toISOString(),
    status: FlagStatus.ACCEPTED,
    checksystem_response: "Flag accepted successfully",
  },
  {
    id: 2,
    flag: "FLAG{test_flag_2_queued}",
    sploit: "pwn_exploit_v2",
    team: "Team Beta",
    created_time: new Date(Date.now() - 7200000).toISOString(),
    status: FlagStatus.QUEUED,
  },
  {
    id: 3,
    flag: "FLAG{test_flag_3_rejected}",
    team: "Team Gamma",
    created_time: new Date(Date.now() - 10800000).toISOString(),
    start_waiting_time: new Date(Date.now() - 10700000).toISOString(),
    status: FlagStatus.REJECTED,
    checksystem_response: "Invalid flag format",
  },
  {
    id: 4,
    flag: "FLAG{test_flag_4_waiting}",
    sploit: "crypto_exploit_v1",
    team: "Team Alpha",
    created_time: new Date(Date.now() - 14400000).toISOString(),
    start_waiting_time: new Date(Date.now() - 14300000).toISOString(),
    status: FlagStatus.WAITING,
    checksystem_response: "Checking with system...",
  },
  {
    id: 5,
    flag: "FLAG{test_flag_5_accepted}",
    sploit: "web_exploit_v2",
    team: "Team Delta",
    created_time: new Date(Date.now() - 18000000).toISOString(),
    start_waiting_time: new Date(Date.now() - 17900000).toISOString(),
    status: FlagStatus.ACCEPTED,
    checksystem_response: "Flag accepted successfully",
  },
  {
    id: 6,
    flag: "FLAG{test_flag_6_skipped}",
    team: "Team Beta",
    created_time: new Date(Date.now() - 21600000).toISOString(),
    status: FlagStatus.SKIPPED,
    checksystem_response: "Flag skipped due to timeout",
  },
  {
    id: 7,
    flag: "FLAG{test_flag_7_accepted}",
    sploit: "reverse_exploit_v1",
    team: "Team Gamma",
    created_time: new Date(Date.now() - 25200000).toISOString(),
    start_waiting_time: new Date(Date.now() - 25100000).toISOString(),
    status: FlagStatus.ACCEPTED,
    checksystem_response: "Flag accepted successfully",
  },
  {
    id: 8,
    flag: "FLAG{test_flag_8_rejected}",
    sploit: "web_exploit_v1",
    team: "Team Alpha",
    created_time: new Date(Date.now() - 28800000).toISOString(),
    start_waiting_time: new Date(Date.now() - 28700000).toISOString(),
    status: FlagStatus.REJECTED,
    checksystem_response: "Flag already submitted",
  },
  {
    id: 9,
    flag: "FLAG{test_flag_9_waiting}",
    team: "Team Delta",
    created_time: new Date(Date.now() - 32400000).toISOString(),
    start_waiting_time: new Date(Date.now() - 32300000).toISOString(),
    status: FlagStatus.WAITING,
  },
  {
    id: 10,
    flag: "FLAG{test_flag_10_accepted}",
    sploit: "pwn_exploit_v3",
    team: "Team Beta",
    created_time: new Date(Date.now() - 36000000).toISOString(),
    start_waiting_time: new Date(Date.now() - 35900000).toISOString(),
    status: FlagStatus.ACCEPTED,
    checksystem_response: "Flag accepted successfully",
  },
  {
    id: 11,
    flag: "FLAG{test_flag_11_queued}",
    sploit: "crypto_exploit_v2",
    team: "Team Gamma",
    created_time: new Date(Date.now() - 39600000).toISOString(),
    status: FlagStatus.QUEUED,
  },
  {
    id: 12,
    flag: "FLAG{test_flag_12_accepted}",
    team: "Team Alpha",
    created_time: new Date(Date.now() - 43200000).toISOString(),
    start_waiting_time: new Date(Date.now() - 43100000).toISOString(),
    status: FlagStatus.ACCEPTED,
    checksystem_response: "Flag accepted successfully",
  },
  {
    id: 13,
    flag: "FLAG{test_flag_13_rejected}",
    sploit: "web_exploit_v3",
    team: "Team Delta",
    created_time: new Date(Date.now() - 46800000).toISOString(),
    start_waiting_time: new Date(Date.now() - 46700000).toISOString(),
    status: FlagStatus.REJECTED,
    checksystem_response: "Service is down",
  },
  {
    id: 14,
    flag: "FLAG{test_flag_14_skipped}",
    sploit: "reverse_exploit_v2",
    team: "Team Beta",
    created_time: new Date(Date.now() - 50400000).toISOString(),
    start_waiting_time: new Date(Date.now() - 50300000).toISOString(),
    status: FlagStatus.SKIPPED,
    checksystem_response: "Skipped by user",
  },
  {
    id: 15,
    flag: "FLAG{test_flag_15_accepted}",
    team: "Team Gamma",
    created_time: new Date(Date.now() - 54000000).toISOString(),
    start_waiting_time: new Date(Date.now() - 53900000).toISOString(),
    status: FlagStatus.ACCEPTED,
    checksystem_response: "Flag accepted successfully",
  },
]

export function FlagDashboard() {
  const [allFlags, setAllFlags] = useState<FlagType[]>(MOCK_FLAGS)
  const [flags, setFlags] = useState<FlagType[]>([])
  const [isLoading, setIsLoading] = useState(true)
  const [selectedFlag, setSelectedFlag] = useState<FlagType | null>(null)
  const [isRefreshing, setIsRefreshing] = useState(false)
  const [lastUpdated, setLastUpdated] = useState<Date>(new Date())

  const [activeTab, setActiveTab] = useState<"flags" | "statistics">("flags")

  const [flagInput, setFlagInput] = useState("")
  const [isSubmitting, setIsSubmitting] = useState(false)

  const [currentPage, setCurrentPage] = useState(1)
  const [itemsPerPage, setItemsPerPage] = useState(10)
  const [searchTerm, setSearchTerm] = useState("")
  const [statusFilter, setStatusFilter] = useState<string>("all")
  const [teamFilter, setTeamFilter] = useState<string>("all")
  const [sortField, setSortField] = useState<SortField>("created_time")
  const [sortDirection, setSortDirection] = useState<SortDirection>("desc")

  const [stats, setStats] = useState({
    total: 0,
    accepted: 0,
    queued: 0,
    rejected: 0,
    waiting: 0,
    skipped: 0,
    teams: 0,
  })

  const loadFlags = useCallback(() => {
    setIsRefreshing(true)

    let filtered = [...allFlags]

    if (searchTerm) {
      filtered = filtered.filter(
        (flag) =>
          flag.flag.toLowerCase().includes(searchTerm.toLowerCase()) ||
          flag.team?.toLowerCase().includes(searchTerm.toLowerCase()) ||
          flag.sploit?.toLowerCase().includes(searchTerm.toLowerCase()),
      )
    }

    if (statusFilter !== "all") {
      filtered = filtered.filter((flag) => flag.status === statusFilter)
    }

    if (teamFilter !== "all") {
      filtered = filtered.filter((flag) => flag.team === teamFilter)
    }

    filtered.sort((a, b) => {
      let aVal: string | number | undefined = a[sortField as keyof typeof a]
      let bVal: string | number | undefined = b[sortField as keyof typeof b]

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

    const newStats = {
      total: filtered.length,
      accepted: filtered.filter((f) => f.status === FlagStatus.ACCEPTED).length,
      queued: filtered.filter((f) => f.status === FlagStatus.QUEUED).length,
      rejected: filtered.filter((f) => f.status === FlagStatus.REJECTED).length,
      waiting: filtered.filter((f) => f.status === FlagStatus.WAITING).length,
      skipped: filtered.filter((f) => f.status === FlagStatus.SKIPPED).length,
      teams: new Set(filtered.map((f) => f.team).filter(Boolean)).size,
    }

    setStats(newStats)

    const startIndex = (currentPage - 1) * itemsPerPage
    const paginatedFlags = filtered.slice(startIndex, startIndex + itemsPerPage)

    setFlags(paginatedFlags)
    setLastUpdated(new Date())
    setIsLoading(false)
    setIsRefreshing(false)
  }, [allFlags, searchTerm, statusFilter, teamFilter, sortField, sortDirection, currentPage, itemsPerPage])

  useEffect(() => {
    loadFlags()
  }, [loadFlags])

  const handleQuickSubmit = async () => {
    if (!flagInput.trim()) return

    setIsSubmitting(true)

    const newFlag: FlagType = {
      id: Math.max(...allFlags.map((f) => f.id)) + 1,
      flag: flagInput.trim(),
      created_time: new Date().toISOString(),
      status: FlagStatus.QUEUED,
    }

    setAllFlags([newFlag, ...allFlags])
    setFlagInput("")
    setIsSubmitting(false)
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

  const totalPages = Math.ceil(stats.total / itemsPerPage)
  const startIndex = (currentPage - 1) * itemsPerPage

  useEffect(() => {
    setCurrentPage(1)
  }, [searchTerm, statusFilter, teamFilter, itemsPerPage])

  const uniqueTeams = useMemo(() => {
    return Array.from(new Set(allFlags.map((f) => f.team).filter(Boolean)))
  }, [allFlags])

  const handleSort = (field: SortField) => {
    if (sortField === field) {
      setSortDirection(sortDirection === "asc" ? "desc" : "asc")
    } else {
      setSortField(field)
      setSortDirection("asc")
    }
  }

  const getStatusIcon = (status: FlagStatus) => {
    switch (status) {
      case FlagStatus.ACCEPTED:
        return <span className="text-green-500">✓</span>
      case FlagStatus.REJECTED:
        return <span className="text-destructive">✗</span>
      case FlagStatus.WAITING:
        return <span className="text-yellow-500">⚠</span>
      case FlagStatus.SKIPPED:
        return <span className="text-muted-foreground">⊘</span>
      case FlagStatus.QUEUED:
      default:
        return <span className="text-blue-500">⏱</span>
    }
  }

  const getStatusColor = (status: FlagStatus) => {
    switch (status) {
      case FlagStatus.ACCEPTED:
        return "text-green-500"
      case FlagStatus.REJECTED:
        return "text-destructive"
      case FlagStatus.WAITING:
        return "text-yellow-500"
      case FlagStatus.SKIPPED:
        return "text-muted-foreground"
      case FlagStatus.QUEUED:
      default:
        return "text-blue-500"
    }
  }

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
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-4">
            <Image src="/images/sibears-paw.jpg" alt="SiBears" width={48} height={48} className="object-contain red-glow" />
            <div>
              <h1 className="text-3xl font-bold font-mono text-foreground">SiBears Farm Dashboard</h1>
              <p className="text-muted-foreground font-mono">Flag Submission System</p>
            </div>
          </div>
          <div className="flex items-center space-x-3">
            <Button
              variant="outline"
              onClick={loadFlags}
              disabled={isRefreshing}
              className="font-mono border-border hover:bg-muted bg-transparent"
            >
              <span className={`mr-2 ${isRefreshing ? "animate-spin" : ""}`}>↻</span>
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

        <Card className="bg-card border-border red-glow">
          <CardContent className="p-4">
            <div className="flex gap-3">
              <Input
                placeholder="Enter flag to submit (e.g., FLAG{...})"
                value={flagInput}
                onChange={(e) => setFlagInput(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === "Enter") handleQuickSubmit()
                }}
                className="flex-1 font-mono bg-input border-border text-foreground focus:ring-primary"
                disabled={isSubmitting}
              />
              <Button
                onClick={handleQuickSubmit}
                disabled={!flagInput.trim() || isSubmitting}
                className="font-mono bg-primary hover:bg-primary/90 px-6"
              >
                {isSubmitting ? "Submitting..." : "Submit Flag"}
              </Button>
            </div>
          </CardContent>
        </Card>

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

        {activeTab === "statistics" && (
          <div className="space-y-6">
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              <Card className="bg-card border-border red-glow">
                <CardHeader>
                  <CardTitle className="font-mono text-foreground flex items-center gap-2">
                    <span className="text-2xl text-primary">▸</span>
                    Total Flags
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <p className="text-4xl font-bold font-mono text-foreground">{stats.total}</p>
                  <p className="text-sm font-mono text-muted-foreground mt-2">All submitted flags</p>
                </CardContent>
              </Card>

              <Card className="bg-card border-border red-glow">
                <CardHeader>
                  <CardTitle className="font-mono text-green-500 flex items-center gap-2">
                    <span className="text-2xl">✓</span>
                    Accepted
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <p className="text-4xl font-bold font-mono text-green-500">{stats.accepted}</p>
                  <p className="text-sm font-mono text-muted-foreground mt-2">
                    {stats.total > 0 ? ((stats.accepted / stats.total) * 100).toFixed(1) : 0}% success rate
                  </p>
                </CardContent>
              </Card>

              <Card className="bg-card border-border red-glow">
                <CardHeader>
                  <CardTitle className="font-mono text-blue-500 flex items-center gap-2">
                    <span className="text-2xl">⏱</span>
                    Queued
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <p className="text-4xl font-bold font-mono text-blue-500">{stats.queued}</p>
                  <p className="text-sm font-mono text-muted-foreground mt-2">Awaiting submission</p>
                </CardContent>
              </Card>

              <Card className="bg-card border-border red-glow">
                <CardHeader>
                  <CardTitle className="font-mono text-yellow-500 flex items-center gap-2">
                    <span className="text-2xl">⚠</span>
                    Waiting
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <p className="text-4xl font-bold font-mono text-yellow-500">{stats.waiting}</p>
                  <p className="text-sm font-mono text-muted-foreground mt-2">Currently being checked</p>
                </CardContent>
              </Card>

              <Card className="bg-card border-border red-glow">
                <CardHeader>
                  <CardTitle className="font-mono text-destructive flex items-center gap-2">
                    <span className="text-2xl">✗</span>
                    Rejected
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <p className="text-4xl font-bold font-mono text-destructive">{stats.rejected}</p>
                  <p className="text-sm font-mono text-muted-foreground mt-2">Invalid or duplicate flags</p>
                </CardContent>
              </Card>

              <Card className="bg-card border-border red-glow">
                <CardHeader>
                  <CardTitle className="font-mono text-muted-foreground flex items-center gap-2">
                    <span className="text-2xl">⊘</span>
                    Skipped
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <p className="text-4xl font-bold font-mono text-muted-foreground">{stats.skipped}</p>
                  <p className="text-sm font-mono text-muted-foreground mt-2">Skipped flags</p>
                </CardContent>
              </Card>

              <Card className="bg-card border-border red-glow">
                <CardHeader>
                  <CardTitle className="font-mono text-primary flex items-center gap-2">
                    <span className="text-2xl">◆</span>
                    Teams
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <p className="text-4xl font-bold font-mono text-primary">{stats.teams}</p>
                  <p className="text-sm font-mono text-muted-foreground mt-2">Active teams</p>
                </CardContent>
              </Card>
            </div>

            <Card className="bg-card border-border red-glow">
              <CardHeader>
                <CardTitle className="font-mono text-foreground">Last Updated</CardTitle>
              </CardHeader>
              <CardContent>
                <p className="font-mono text-muted-foreground">{lastUpdated.toLocaleString()}</p>
              </CardContent>
            </Card>
          </div>
        )}

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
                  {stats.total} total
                </Badge>
              </div>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex flex-col md:flex-row gap-4">
                <div className="flex-1">
                  <Input
                    placeholder="Search flags, teams, or exploits..."
                    value={searchTerm}
                    onChange={(e) => setSearchTerm(e.target.value)}
                    className="font-mono bg-input border-border text-foreground"
                  />
                </div>

                <div className="flex gap-2">
                  <select
                    value={statusFilter}
                    onChange={(e) => setStatusFilter(e.target.value)}
                    className="w-40 px-3 py-2 font-mono bg-input border border-border rounded-md text-foreground cursor-pointer"
                  >
                    <option value="all">All Status</option>
                    <option value={FlagStatus.QUEUED}>⏱ Queued</option>
                    <option value={FlagStatus.WAITING}>⚠ Waiting</option>
                    <option value={FlagStatus.ACCEPTED}>✓ Accepted</option>
                    <option value={FlagStatus.REJECTED}>✗ Rejected</option>
                    <option value={FlagStatus.SKIPPED}>⊘ Skipped</option>
                  </select>

                  <select
                    value={teamFilter}
                    onChange={(e) => setTeamFilter(e.target.value)}
                    className="w-40 px-3 py-2 font-mono bg-input border border-border rounded-md text-foreground cursor-pointer"
                  >
                    <option value="all">All Teams</option>
                    {uniqueTeams.map((team) => (
                      <option key={team} value={team}>
                        {team}
                      </option>
                    ))}
                  </select>

                  <select
                    value={itemsPerPage.toString()}
                    onChange={(e) => setItemsPerPage(Number(e.target.value))}
                    className="w-20 px-3 py-2 font-mono bg-input border border-border rounded-md text-foreground cursor-pointer"
                  >
                    <option value="5">5</option>
                    <option value="10">10</option>
                    <option value="25">25</option>
                    <option value="50">50</option>
                  </select>
                </div>
              </div>

              <div className="overflow-x-auto">
                <table className="w-full">
                  <thead>
                    <tr className="border-b border-border">
                      <th className="text-left p-3 font-mono text-muted-foreground">
                        <Button
                          variant="ghost"
                          onClick={() => handleSort("id")}
                          className="font-mono text-muted-foreground hover:text-foreground p-0 h-auto"
                        >
                          ID {sortField === "id" && (sortDirection === "asc" ? "↑" : "↓")}
                        </Button>
                      </th>
                      <th className="text-left p-3 font-mono text-muted-foreground">
                        <Button
                          variant="ghost"
                          onClick={() => handleSort("flag")}
                          className="font-mono text-muted-foreground hover:text-foreground p-0 h-auto"
                        >
                          Flag {sortField === "flag" && (sortDirection === "asc" ? "↑" : "↓")}
                        </Button>
                      </th>
                      <th className="text-left p-3 font-mono text-muted-foreground">
                        <Button
                          variant="ghost"
                          onClick={() => handleSort("team")}
                          className="font-mono text-muted-foreground hover:text-foreground p-0 h-auto"
                        >
                          Team {sortField === "team" && (sortDirection === "asc" ? "↑" : "↓")}
                        </Button>
                      </th>
                      <th className="text-left p-3 font-mono text-muted-foreground">
                        <Button
                          variant="ghost"
                          onClick={() => handleSort("sploit")}
                          className="font-mono text-muted-foreground hover:text-foreground p-0 h-auto"
                        >
                          Sploit {sortField === "sploit" && (sortDirection === "asc" ? "↑" : "↓")}
                        </Button>
                      </th>
                      <th className="text-left p-3 font-mono text-muted-foreground">
                        <Button
                          variant="ghost"
                          onClick={() => handleSort("status")}
                          className="font-mono text-muted-foreground hover:text-foreground p-0 h-auto"
                        >
                          Status {sortField === "status" && (sortDirection === "asc" ? "↑" : "↓")}
                        </Button>
                      </th>
                      <th className="text-left p-3 font-mono text-muted-foreground">
                        <Button
                          variant="ghost"
                          onClick={() => handleSort("checksystem_response")}
                          className="font-mono text-muted-foreground hover:text-foreground p-0 h-auto"
                        >
                          Response {sortField === "checksystem_response" && (sortDirection === "asc" ? "↑" : "↓")}
                        </Button>
                      </th>
                      <th className="text-left p-3 font-mono text-muted-foreground">
                        <Button
                          variant="ghost"
                          onClick={() => handleSort("created_time")}
                          className="font-mono text-muted-foreground hover:text-foreground p-0 h-auto"
                        >
                          Submitted {sortField === "created_time" && (sortDirection === "asc" ? "↑" : "↓")}
                        </Button>
                      </th>
                      <th className="text-left p-3 font-mono text-muted-foreground">Actions</th>
                    </tr>
                  </thead>
                  <tbody>
                    {flags.map((flag) => (
                      <tr key={flag.id} className="border-b border-border hover:bg-muted/10">
                        <td className="p-3 font-mono text-foreground">#{flag.id}</td>
                        <td className="p-3 font-mono text-foreground max-w-xs truncate">{flag.flag}</td>
                        <td className="p-3 font-mono text-foreground">
                          {flag.team ? (
                            flag.team
                          ) : (
                            <Badge variant="outline" className="font-mono text-xs text-muted-foreground bg-muted/30">
                              —
                            </Badge>
                          )}
                        </td>
                        <td className="p-3 font-mono text-muted-foreground">
                          {flag.sploit ? (
                            <Badge variant="secondary" className="font-mono text-xs">
                              {flag.sploit}
                            </Badge>
                          ) : (
                            <Badge variant="outline" className="font-mono text-xs text-muted-foreground bg-muted/30">
                              —
                            </Badge>
                          )}
                        </td>
                        <td className="p-3">
                          <div className={`flex items-center space-x-2 ${getStatusColor(flag.status)}`}>
                            {getStatusIcon(flag.status)}
                            <span className="font-mono">{flag.status}</span>
                          </div>
                        </td>
                        <td className="p-3 font-mono text-muted-foreground text-sm max-w-xs truncate">
                          {flag.checksystem_response ? (
                            flag.checksystem_response
                          ) : (
                            <Badge variant="outline" className="font-mono text-xs text-muted-foreground bg-muted/30">
                              —
                            </Badge>
                          )}
                        </td>
                        <td className="p-3 font-mono text-muted-foreground text-sm">
                          {new Date(flag.created_time).toLocaleString()}
                        </td>
                        <td className="p-3">
                          <Button
                            variant="ghost"
                            size="sm"
                            onClick={() => setSelectedFlag(flag)}
                            className="font-mono text-primary hover:text-primary-foreground hover:bg-primary text-xs"
                          >
                            View
                          </Button>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>

              {totalPages > 1 && (
                <div className="flex items-center justify-between pt-4">
                  <div className="flex items-center space-x-2">
                    <p className="text-sm font-mono text-muted-foreground">
                      Showing {startIndex + 1} to {Math.min(startIndex + itemsPerPage, stats.total)} of {stats.total}{" "}
                      results
                    </p>
                  </div>

                  <div className="flex items-center space-x-2">
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => setCurrentPage(1)}
                      disabled={currentPage === 1}
                      className="font-mono bg-transparent border-border"
                    >
                      <span>⇤</span>
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => setCurrentPage((prev) => Math.max(prev - 1, 1))}
                      disabled={currentPage === 1}
                      className="font-mono bg-transparent border-border"
                    >
                      <span>←</span>
                    </Button>

                    <div className="flex items-center space-x-1">
                      {Array.from({ length: Math.min(5, totalPages) }, (_, i) => {
                        let pageNum
                        if (totalPages <= 5) {
                          pageNum = i + 1
                        } else if (currentPage <= 3) {
                          pageNum = i + 1
                        } else if (currentPage >= totalPages - 2) {
                          pageNum = totalPages - 4 + i
                        } else {
                          pageNum = currentPage - 2 + i
                        }

                        return (
                          <Button
                            key={pageNum}
                            variant={currentPage === pageNum ? "default" : "outline"}
                            size="sm"
                            onClick={() => setCurrentPage(pageNum)}
                            className={`font-mono w-8 h-8 p-0 ${
                              currentPage === pageNum
                                ? "bg-primary text-primary-foreground"
                                : "bg-transparent border-border"
                            }`}
                          >
                            {pageNum}
                          </Button>
                        )
                      })}
                    </div>

                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => setCurrentPage((prev) => Math.min(prev + 1, totalPages))}
                      disabled={currentPage === totalPages}
                      className="font-mono bg-transparent border-border"
                    >
                      <span>→</span>
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => setCurrentPage(totalPages)}
                      disabled={currentPage === totalPages}
                      className="font-mono bg-transparent border-border"
                    >
                      <span>⇥</span>
                    </Button>
                  </div>
                </div>
              )}
            </CardContent>
          </Card>
        )}
      </div>

      <FlagDetailsModal flag={selectedFlag} isOpen={!!selectedFlag} onClose={() => setSelectedFlag(null)} />
    </div>
  )
}
