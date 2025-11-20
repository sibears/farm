"use client"

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { useFlagStats } from "@/hooks/useFlagStats"

export function FlagStats() {
  const { data: stats, isLoading } = useFlagStats()

  if (isLoading) {
    return <div className="text-center font-mono text-muted-foreground">Loading statistics...</div>
  }

  if (!stats) return null

  return (
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
    </div>
  )
}
