import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"
import type { FlagType } from "@/lib/types"

export function useFlags() {
  return useQuery({
    queryKey: ["flags"],
    queryFn: async () => {
      const response = await fetch("/api/flags")
      if (!response.ok) throw new Error("Failed to fetch flags")
      return response.json() as Promise<FlagType[]>
    },
  })
}

export function usePaginatedFlags(page: number, pageSize: number) {
  return useQuery({
    queryKey: ["flags", "paginated", page, pageSize],
    queryFn: async () => {
      const params = new URLSearchParams({
        limit: String(pageSize),
        offset: String((page - 1) * pageSize),
      })
      const response = await fetch(`/api/flags?${params}`)
      if (!response.ok) throw new Error("Failed to fetch flags")
      return response.json() as Promise<{ flags: FlagType[]; total: number }>
    },
    placeholderData: (previousData) => previousData,
  })
}

export function useFlagSubmit() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (data: { flag: string; sploit?: string; team?: string }) => {
      const response = await fetch("/api/flags", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(data),
      })

      if (!response.ok) {
        const error = await response.json()
        throw new Error((error as { error?: string }).error || "Failed to submit flag")
      }

      return response.json()
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["flags"] })
      queryClient.invalidateQueries({ queryKey: ["flagStats"] })
    },
  })
}
