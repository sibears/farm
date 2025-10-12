import type { NextRequest } from "next/server"
import { jwtVerify } from "jose"
import { AUTH_CONFIG } from './env-config'

const JWT_SECRET = new TextEncoder().encode(AUTH_CONFIG.JWT_SECRET)

export async function verifyAuth(request: NextRequest): Promise<boolean> {
  try {
    const token = request.cookies.get(AUTH_CONFIG.COOKIE_NAME)?.value

    if (!token) {
      return false
    }

    await jwtVerify(token, JWT_SECRET)
    return true
  } catch (error) {
    console.error("Auth verification failed:", error)
    return false
  }
}
