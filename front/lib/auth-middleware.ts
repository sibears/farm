import type { NextRequest } from "next/server"
import { config } from "./config"

export interface AuthContext {
  authenticated: boolean
  passwordHash?: string
}

export async function verifyAuth(request: NextRequest): Promise<AuthContext> {
  try {
    const storedHash = request.cookies.get(config.auth.cookieName)?.value

    if (!storedHash) {
      return { authenticated: false }
    }

    return {
      authenticated: true,
      passwordHash: storedHash,
    }
  } catch (error) {
    console.error("Auth verification failed:", error)
    return { authenticated: false }
  }
}
