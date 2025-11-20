"use client"

import { Input } from "@/components/ui/input"
import { FlagStatus } from "@/lib/types"

type FlagFiltersProps = {
  searchTerm: string
  onSearchChange: (value: string) => void
  statusFilter: string
  onStatusChange: (value: string) => void
  teamFilter: string
  onTeamChange: (value: string) => void
  uniqueTeams: string[]
  itemsPerPage: number
  onItemsPerPageChange: (value: number) => void
}

export function FlagFilters({
  searchTerm,
  onSearchChange,
  statusFilter,
  onStatusChange,
  teamFilter,
  onTeamChange,
  uniqueTeams,
  itemsPerPage,
  onItemsPerPageChange,
}: FlagFiltersProps) {
  return (
    <div className="flex flex-col md:flex-row gap-4">
      <div className="flex-1">
        <Input
          placeholder="Search flags, teams, or exploits..."
          value={searchTerm}
          onChange={(e) => onSearchChange(e.target.value)}
          className="font-mono bg-input border-border text-foreground"
        />
      </div>

      <div className="flex gap-2">
        <select
          value={statusFilter}
          onChange={(e) => onStatusChange(e.target.value)}
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
          onChange={(e) => onTeamChange(e.target.value)}
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
          onChange={(e) => onItemsPerPageChange(Number(e.target.value))}
          className="w-20 px-3 py-2 font-mono bg-input border border-border rounded-md text-foreground cursor-pointer"
        >
          <option value="5">5</option>
          <option value="10">10</option>
          <option value="25">25</option>
          <option value="50">50</option>
        </select>
      </div>
    </div>
  )
}
