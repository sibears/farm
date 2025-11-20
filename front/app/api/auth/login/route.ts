import { createHash } from "node:crypto"
import { cookies } from "next/headers"
import { type NextRequest, NextResponse } from "next/server"
import { BACKEND_CONFIG, callBackendAPI } from "@/lib/backend-config"
import { AUTH_CONFIG, SECURITY_CONFIG } from "@/lib/env-config"

// Указываем, что роут динамический
export const dynamic = "force-dynamic"

export async function POST(request: NextRequest) {
  try {
    const { password } = await request.json()
    const cookieStore = cookies()

    const persistAuthHash = (rawPassword: string) => {
      const passwordHash = createHash("sha256").update(rawPassword).digest("hex")
      cookieStore.set(AUTH_CONFIG.COOKIE_NAME, passwordHash, {
        httpOnly: true,
        secure: SECURITY_CONFIG.SECURE_COOKIES,
        sameSite: SECURITY_CONFIG.SAME_SITE as "strict" | "lax",
        maxAge: AUTH_CONFIG.COOKIE_MAX_AGE,
        path: "/",
      })
      return passwordHash
    }

    try {
      const backendResponse = await callBackendAPI(BACKEND_CONFIG.ENDPOINTS.auth, {
        method: "POST",
        body: JSON.stringify({ passwd: password }), // Use 'passwd' key as expected by Rust backend
      })

      if (backendResponse.success && backendResponse.data === "ok") {
        persistAuthHash(password)

        return NextResponse.json({
          success: true,
          message: "Authentication successful (backend)",
        })
      }
    } catch (backendError) {
      console.error("Backend authentication failed:", backendError)
    }

    // Hardcoded fallback (deprecated, лучше использовать backend)
    const fallbackPassword = "sibears1cool"

    if (password === fallbackPassword) {
      persistAuthHash(password)

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
