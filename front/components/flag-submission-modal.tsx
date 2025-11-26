"use client"

import type React from "react"

import { useState } from "react"
import { Alert, AlertDescription } from "@/components/ui/alert"
import { Button } from "@/components/ui/button"
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from "@/components/ui/dialog"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"

interface FlagSubmissionModalProps {
  isOpen: boolean
  onClose: () => void
  onSubmit: (flag: {
    flag: string
    sploit?: string
    team?: string
    start_waiting_time?: string
    checksystem_response?: string
  }) => void
}

export function FlagSubmissionModal({ isOpen, onClose, onSubmit }: FlagSubmissionModalProps) {
  const [formData, setFormData] = useState({
    flag: "",
    sploit: "",
    team: "",
  })
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [error, setError] = useState("")

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setIsSubmitting(true)
    setError("")

    try {
      await onSubmit({
        flag: formData.flag,
        sploit: formData.sploit || undefined,
        team: formData.team || undefined,
        start_waiting_time: new Date().toISOString(),
      })

      setFormData({ flag: "", sploit: "", team: "" })
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to submit flag")
    } finally {
      setIsSubmitting(false)
    }
  }

  const handleClose = () => {
    if (!isSubmitting) {
      setFormData({ flag: "", sploit: "", team: "" })
      setError("")
      onClose()
    }
  }

  return (
    <Dialog open={isOpen} onOpenChange={handleClose}>
      <DialogContent className="bg-card border-border max-w-md">
        <DialogHeader>
          <DialogTitle className="font-mono text-foreground flex items-center space-x-2">
            <span>Submit New Flag</span>
          </DialogTitle>
          <DialogDescription className="font-mono text-muted-foreground">
            Submit a captured flag to the CTF Farm system
          </DialogDescription>
        </DialogHeader>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="flag" className="font-mono text-foreground">
              Flag *
            </Label>
            <Input
              id="flag"
              placeholder="Enter flag"
              value={formData.flag}
              onChange={(e) => setFormData((prev) => ({ ...prev, flag: e.target.value }))}
              className="font-mono bg-input border-border text-foreground"
              required
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="team" className="font-mono text-foreground">
              Team Name
            </Label>
            <Input
              id="team"
              placeholder="Your team name"
              value={formData.team}
              onChange={(e) => setFormData((prev) => ({ ...prev, team: e.target.value }))}
              className="font-mono bg-input border-border text-foreground"
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="sploit" className="font-mono text-foreground">
              Exploit Script
            </Label>
            <Input
              id="sploit"
              placeholder="exploit.py, payload.sh, etc."
              value={formData.sploit}
              onChange={(e) => setFormData((prev) => ({ ...prev, sploit: e.target.value }))}
              className="font-mono bg-input border-border text-foreground"
            />
          </div>

          {error && (
            <Alert className="border-destructive bg-destructive/10">
              <AlertDescription className="font-mono text-destructive">{error}</AlertDescription>
            </Alert>
          )}

          <div className="flex space-x-3 pt-4">
            <Button
              type="button"
              variant="outline"
              onClick={handleClose}
              disabled={isSubmitting}
              className="flex-1 font-mono border-border hover:bg-muted bg-transparent"
            >
              Cancel
            </Button>
            <Button type="submit" disabled={isSubmitting} className="flex-1 font-mono bg-primary hover:bg-primary/90">
              {isSubmitting ? (
                <span className="flex items-center space-x-2">
                  <div className="w-4 h-4 border-2 border-primary-foreground border-t-transparent rounded-full animate-spin" />
                  <span>Submitting...</span>
                </span>
              ) : (
                "Submit Flag"
              )}
            </Button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  )
}
