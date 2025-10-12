// Enum для статусов флагов
export enum FlagStatus {
  QUEUED = "QUEUED",
  WAITING = "WAITING", 
  SKIPPED = "SKIPPED",
  ACCEPTED = "ACCEPTED",
  REJECTED = "REJECTED"
}

// Интерфейс для флага
export interface FlagType {
  id: number
  flag: string
  sploit?: string
  team?: string
  created_time: string
  start_waiting_time?: string
  status: FlagStatus
  checksystem_response?: string
}