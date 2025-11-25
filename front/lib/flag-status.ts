import { FlagStatus } from "./types"

export function getStatusIcon(status: FlagStatus): string {
  switch (status) {
    case FlagStatus.ACCEPTED:
      return "✓"
    case FlagStatus.REJECTED:
      return "✗"
    case FlagStatus.WAITING:
      return "⚠"
    case FlagStatus.QUEUED:
      return "⏱"
    case FlagStatus.SKIPPED:
      return "⊘"
    default:
      return "⏱"
  }
}

export function getStatusColor(status: FlagStatus): string {
  switch (status) {
    case FlagStatus.ACCEPTED:
      return "text-green-500"
    case FlagStatus.REJECTED:
      return "text-destructive"
    case FlagStatus.WAITING:
      return "text-yellow-500"
    case FlagStatus.QUEUED:
      return "text-blue-500"
    case FlagStatus.SKIPPED:
      return "text-muted-foreground"
    default:
      return "text-blue-500"
  }
}

