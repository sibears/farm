import { useQuery } from "@tanstack/react-query"
import { FlagStatus } from "@/lib/types"

export type FlagStats = {
  total: number
  accepted: number
  queued: number
  rejected: number
  waiting: number
  skipped: number
  teams: number
}

export function useFlagStats() {
  return useQuery({
    queryKey: ["flagStats"],
    queryFn: async () => {
      const response = await fetch("/api/flags/stats")
      if (!response.ok) throw new Error("Failed to fetch stats")

      const payload = (await response.json()) as [string, number][]

      const aggregated: FlagStats = {
        total: 0,
        accepted: 0,
        queued: 0,
        rejected: 0,
        waiting: 0,
        skipped: 0,
        teams: 0,
      }

      payload.forEach(([status, count]) => {
        aggregated.total += count
        switch (status.toUpperCase()) {
          case FlagStatus.ACCEPTED:
            aggregated.accepted = count
            break
          case FlagStatus.QUEUED:
            aggregated.queued = count
            break
          case FlagStatus.REJECTED:
            aggregated.rejected = count
            break
          case FlagStatus.WAITING:
            aggregated.waiting = count
            break
          case FlagStatus.SKIPPED:
            aggregated.skipped = count
            break
        }
      })

      return aggregated
    },
    refetchInterval: 30000,
  })
}
