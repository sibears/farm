"use client"

import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { FlagStatus, type FlagType } from "@/lib/types"

type SortField = "id" | "flag" | "team" | "sploit" | "status" | "checksystem_response" | "created_time"
type SortDirection = "asc" | "desc"

type FlagTableProps = {
  flags: FlagType[]
  onFlagClick: (flag: FlagType) => void
  sortField: SortField
  sortDirection: SortDirection
  onSort: (field: SortField) => void
}

export function FlagTable({ flags, onFlagClick, sortField, sortDirection, onSort }: FlagTableProps) {
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
      default:
        return "text-blue-500"
    }
  }

  const SortButton = ({ field, label }: { field: SortField; label: string }) => (
    <Button
      variant="ghost"
      onClick={() => onSort(field)}
      className="font-mono text-muted-foreground hover:text-foreground p-0 h-auto"
    >
      {label} {sortField === field && (sortDirection === "asc" ? "↑" : "↓")}
    </Button>
  )

  return (
    <div className="overflow-x-auto">
      <table className="w-full">
        <thead>
          <tr className="border-b border-border">
            <th className="text-left p-3 font-mono text-muted-foreground">
              <SortButton field="id" label="ID" />
            </th>
            <th className="text-left p-3 font-mono text-muted-foreground">
              <SortButton field="flag" label="Flag" />
            </th>
            <th className="text-left p-3 font-mono text-muted-foreground">
              <SortButton field="team" label="Team" />
            </th>
            <th className="text-left p-3 font-mono text-muted-foreground">
              <SortButton field="sploit" label="Sploit" />
            </th>
            <th className="text-left p-3 font-mono text-muted-foreground">
              <SortButton field="status" label="Status" />
            </th>
            <th className="text-left p-3 font-mono text-muted-foreground">
              <SortButton field="checksystem_response" label="Response" />
            </th>
            <th className="text-left p-3 font-mono text-muted-foreground">
              <SortButton field="created_time" label="Submitted" />
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
                  onClick={() => onFlagClick(flag)}
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
  )
}
