"use client"

import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Card, CardContent } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { useFlagSubmit } from "@/hooks/useFlags"

export function FlagSubmitForm() {
  const [flagInput, setFlagInput] = useState("")
  const { mutate: submitFlag, isPending, error } = useFlagSubmit()

  const handleSubmit = () => {
    if (!flagInput.trim()) return

    submitFlag(
      { flag: flagInput.trim() },
      {
        onSuccess: () => {
          setFlagInput("")
        },
      },
    )
  }

  return (
    <Card className="bg-card border-border red-glow">
      <CardContent className="p-4">
        <div className="flex gap-3">
          <Input
            placeholder="Enter flag to submit"
            value={flagInput}
            onChange={(e) => setFlagInput(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter") handleSubmit()
            }}
            className="flex-1 font-mono bg-input border-border text-foreground focus:ring-primary"
            disabled={isPending}
          />
          <Button
            onClick={handleSubmit}
            disabled={!flagInput.trim() || isPending}
            className="font-mono bg-primary hover:bg-primary/90 px-6"
          >
            {isPending ? "Submitting..." : "Submit Flag"}
          </Button>
        </div>
        {error && <p className="font-mono text-sm text-destructive mt-3">Submission failed: {error.message}</p>}
      </CardContent>
    </Card>
  )
}
