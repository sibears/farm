import type { NextRequest } from "next/server"
import { config } from "./config"

export interface AuthContext {
  authenticated: boolean
  password?: string
}

export async function verifyAuth(request: NextRequest): Promise<AuthContext> {
  try {
    const storedPassword = request.cookies.get(config.auth.cookieName)?.value

    if (!storedPassword) {
      return { authenticated: false }
    }

    return {
      authenticated: true,
      password: storedPassword,
    }
  } catch (error) {
    console.error("Auth verification failed:", error)
    return { authenticated: false }
  }
}
