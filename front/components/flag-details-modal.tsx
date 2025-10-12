"use client"

import type React from "react"

import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog"
import { Badge } from "@/components/ui/badge"
import { FlagStatus, type FlagType } from "@/lib/types"

interface FlagDetailsModalProps {
  flag: FlagType | null
  isOpen: boolean
  onClose: () => void
}

export function FlagDetailsModal({ flag, isOpen, onClose }: FlagDetailsModalProps) {
  if (!flag) return null

  const getStatusIcon = (status: FlagStatus) => {
    switch (status) {
      case FlagStatus.ACCEPTED:
        return <span className="text-xl text-green-500">✓</span>
      case FlagStatus.REJECTED:
        return <span className="text-xl text-destructive">✗</span>
      case FlagStatus.WAITING:
        return <span className="text-xl text-yellow-500">⚠</span>
      case FlagStatus.QUEUED:
        return <span className="text-xl text-blue-500">⏱</span>
      case FlagStatus.SKIPPED:
        return <span className="text-xl text-muted-foreground">⊘</span>
      default:
        return <span className="text-xl text-blue-500">⏱</span>
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
      case FlagStatus.QUEUED:
        return "text-blue-500"
      case FlagStatus.SKIPPED:
        return "text-muted-foreground"
      default:
        return "text-blue-500"
    }
  }

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent className="bg-card border-border max-w-lg max-h-[90vh] overflow-y-auto scrollbar-dark">
        <DialogHeader className="pb-2">
          <DialogTitle className="font-mono text-foreground flex items-center space-x-2">
            <span className="text-primary">▸</span>
            <span>Flag #{flag.id}</span>
          </DialogTitle>
        </DialogHeader>

        <div className="space-y-4">
          {/* Status */}
          <div className={`flex items-center space-x-2 ${getStatusColor(flag.status)}`}>
            {getStatusIcon(flag.status)}
            <span className="font-mono text-sm font-bold">{flag.status}</span>
          </div>
          {/* Flag Value */}
          <div>
            <div className="flex items-center space-x-2 mb-2">
              <span className="text-lg text-primary">▸</span>
              <span className="font-mono text-sm text-muted-foreground">Flag</span>
            </div>
            <p className="font-mono text-foreground bg-muted/20 p-2 rounded border break-all text-sm">{flag.flag}</p>
          </div>

          {/* Team and Sploit in one row */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {flag.team && (
              <div>
                <div className="flex items-center space-x-2 mb-1">
                  <span className="text-lg text-primary">▸</span>
                  <span className="font-mono text-xs text-muted-foreground">Team</span>
                </div>
                <span className="font-mono text-sm text-foreground">{flag.team}</span>
              </div>
            )}

            {flag.sploit && (
              <div>
                <div className="flex items-center space-x-2 mb-1">
                  <span className="text-lg text-primary">▸</span>
                  <span className="font-mono text-xs text-muted-foreground">Exploit</span>
                </div>
                <Badge variant="secondary" className="font-mono text-xs">
                  {flag.sploit}
                </Badge>
              </div>
            )}
          </div>

          {/* Timestamps in compact format */}
          <div>
            <div className="flex items-center space-x-2 mb-2">
              <span className="text-lg text-primary">▸</span>
              <span className="font-mono text-sm text-muted-foreground">Timeline</span>
            </div>
            <div className="space-y-1 text-sm">
              <div className="flex items-center justify-between">
                <span className="font-mono text-muted-foreground">Submitted:</span>
                <span className="font-mono text-foreground">{new Date(flag.created_time).toLocaleString()}</span>
              </div>
              {flag.start_waiting_time && (
                <div className="flex items-center justify-between">
                  <span className="font-mono text-muted-foreground">Processing:</span>
                  <span className="font-mono text-foreground">{new Date(flag.start_waiting_time).toLocaleString()}</span>
                </div>
              )}
            </div>
          </div>

          {/* System Response */}
          {flag.checksystem_response && (
            <div>
              <div className="flex items-center space-x-2 mb-2">
                <span className="text-lg text-primary">▸</span>
                <span className="font-mono text-sm text-muted-foreground">System Response</span>
              </div>
              <p className="font-mono text-foreground bg-muted/20 p-2 rounded border text-sm">{flag.checksystem_response}</p>
            </div>
          )}
        </div>
      </DialogContent>
    </Dialog>
  )
}
