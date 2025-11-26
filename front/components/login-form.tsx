"use client"

import Image from "next/image"
import type React from "react"
import { useState } from "react"
import { Alert, AlertDescription } from "@/components/ui/alert"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"

interface LoginFormProps {
  onLogin: () => void
}

export function LoginForm({ onLogin }: LoginFormProps) {
  const [password, setPassword] = useState("")
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState("")

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setIsLoading(true)
    setError("")

    try {
      const response = await fetch("/api/auth/login", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ password }),
      })

      if (response.ok) {
        await response.json()
        onLogin()
      } else {
        await response.json()
        setError("Invalid password. Access denied.")
      }
    } catch {
      setError("Connection failed. Check your network.")
    } finally {
      setIsLoading(false)
    }
  }

  return (
    <div className="min-h-screen bg-background flex items-center justify-center p-4">
      <div className="w-full max-w-md space-y-6">
        <div className="text-center space-y-4">
          <div className="flex items-center justify-center">
            <Image
              src="/images/sibears-paw.jpg"
              alt="SiBears Logo"
              width={128}
              height={128}
              className="object-contain red-glow-strong"
            />
          </div>
          <h1 className="text-4xl font-bold font-mono glitch-effect text-primary" data-text="SiBEARS FARM">
            SiBEARS FARM
          </h1>
          <p className="text-muted-foreground font-mono">
            <span className="terminal-cursor">Flag Submission System</span>
          </p>
        </div>

        <Card className="bg-card border-border shadow-2xl red-glow">
          <CardHeader className="space-y-1">
            <CardTitle className="text-2xl font-mono flex items-center space-x-2">
              <span>Login</span>
            </CardTitle>
            <CardDescription className="font-mono text-muted-foreground">
              Enter password to access the system
            </CardDescription>
          </CardHeader>
          <CardContent>
            <form onSubmit={handleSubmit} className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="password" className="font-mono text-foreground">
                  Password
                </Label>
                <Input
                  id="password"
                  type="password"
                  placeholder="Enter your password..."
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  className="font-mono bg-input border-border text-foreground placeholder:text-muted-foreground focus:ring-primary"
                  required
                />
              </div>

              {error && (
                <Alert className="border-destructive bg-destructive/10">
                  <AlertDescription className="font-mono text-destructive">{error}</AlertDescription>
                </Alert>
              )}

              <Button
                type="submit"
                className="w-full font-mono bg-primary hover:bg-primary/90 text-primary-foreground"
                disabled={isLoading}
              >
                {isLoading ? (
                  <span className="flex items-center space-x-2">
                    <div className="w-4 h-4 border-2 border-primary-foreground border-t-transparent rounded-full animate-spin" />
                    <span>Authenticating...</span>
                  </span>
                ) : (
                  "Access System"
                )}
              </Button>
            </form>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
