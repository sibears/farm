import { cookies } from "next/headers"
import { type NextRequest, NextResponse } from "next/server"
import { BACKEND_CONFIG, callBackendAPI } from "@/lib/backend-config"
import { AUTH_CONFIG, SECURITY_CONFIG } from "@/lib/env-config"

export const dynamic = "force-dynamic"

export async function POST(request: NextRequest) {
  try {
    const { password } = await request.json()
    const cookieStore = cookies()

    const persistAuth = (rawPassword: string) => {
      cookieStore.set(AUTH_CONFIG.COOKIE_NAME, rawPassword, {
        httpOnly: true,
        secure: SECURITY_CONFIG.SECURE_COOKIES,
        sameSite: SECURITY_CONFIG.SAME_SITE as "strict" | "lax",
        maxAge: AUTH_CONFIG.COOKIE_MAX_AGE,
        path: "/",
      })
    }

    try {
      const backendResponse = await callBackendAPI(BACKEND_CONFIG.ENDPOINTS.auth, {
        method: "POST",
        body: JSON.stringify({ passwd: password }),
      })

      if (backendResponse.success && backendResponse.data === "ok") {
        persistAuth(password)

        return NextResponse.json({
          success: true,
          message: "Authentication successful (backend)",
        })
      }
    } catch (backendError) {
      console.error("Backend authentication failed:", backendError)
    }

    const fallbackPassword = "sibears1cool"

    if (password === fallbackPassword) {
      persistAuth(password)

      return NextResponse.json({
        success: true,
        message: "Authentication successful (local)",
      })
    } else {
      return NextResponse.json({ error: "Invalid password. Access denied." }, { status: 401 })
    }
  } catch (error) {
    console.error("Authentication error:", error)
    return NextResponse.json(
      {
        error: "Internal server error",
        details: error instanceof Error ? error.message : "Unknown error",
      },
      { status: 500 },
    )
  }
}
