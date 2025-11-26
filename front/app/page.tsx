"use client"

import { useEffect, useState } from "react"
import { FlagDashboard } from "@/components/flag-dashboard"
import { LoginForm } from "@/components/login-form"

export default function Home() {
  const [isAuthenticated, setIsAuthenticated] = useState<boolean | null>(null)

  useEffect(() => {
    const verifyAuth = async () => {
      try {
        const response = await fetch("/api/auth/verify")
        setIsAuthenticated(response.ok)
      } catch (error) {
        console.error("Auth verification failed:", error)
        setIsAuthenticated(false)
      }
    }

    verifyAuth()
  }, [])

  if (isAuthenticated === null) {
    return (
      <div className="min-h-screen bg-background flex items-center justify-center">
        <div className="text-center space-y-4">
          <div className="w-8 h-8 border-2 border-primary border-t-transparent rounded-full animate-spin mx-auto" />
          <p className="font-mono text-muted-foreground">Verifying authentication...</p>
        </div>
      </div>
    )
  }

  if (!isAuthenticated) {
    return <LoginForm onLogin={() => setIsAuthenticated(true)} />
  }

  return <FlagDashboard />
}
