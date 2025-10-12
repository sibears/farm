import { type NextRequest, NextResponse } from "next/server"
import { cookies } from "next/headers"
import { jwtVerify } from "jose"
import { AUTH_CONFIG } from "@/lib/env-config"

const JWT_SECRET = new TextEncoder().encode(AUTH_CONFIG.JWT_SECRET)

// Указываем, что роут динамический
export const dynamic = 'force-dynamic'

// eslint-disable-next-line @typescript-eslint/no-unused-vars
export async function GET(_request: NextRequest) {
  try {
    const cookieStore = await cookies()
    const token = cookieStore.get(AUTH_CONFIG.COOKIE_NAME)?.value

    if (!token) {
      return NextResponse.json({ error: "No authentication token found" }, { status: 401 })
    }

    await jwtVerify(token, JWT_SECRET)

    return NextResponse.json({
      authenticated: true,
      message: "Token is valid",
    })
  } catch (error) {
    console.error("Token verification error:", error)
    return NextResponse.json({ error: "Invalid or expired token" }, { status: 401 })
  }
}
