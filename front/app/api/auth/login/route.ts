import { type NextRequest, NextResponse } from "next/server"
import { cookies } from "next/headers"
import { SignJWT } from "jose"
import { callBackendAPI, BACKEND_CONFIG } from "@/lib/backend-config"
import { AUTH_CONFIG, SECURITY_CONFIG } from "@/lib/env-config"

const JWT_SECRET = new TextEncoder().encode(AUTH_CONFIG.JWT_SECRET)

// Указываем, что роут динамический
export const dynamic = 'force-dynamic'

export async function POST(request: NextRequest) {
  try {
    const { password } = await request.json()

    try {
      const backendResponse = await callBackendAPI(BACKEND_CONFIG.ENDPOINTS.AUTH, {
        method: "POST",
        body: JSON.stringify({ passwd: password }), // Use 'passwd' key as expected by Rust backend
      })

      if (backendResponse.success && backendResponse.data === "ok") {
        // Generate local JWT token
        const token = await new SignJWT({
          authenticated: true,
          timestamp: Date.now(),
          backend_auth: true,
        })
          .setProtectedHeader({ alg: "HS256" })
          .setIssuedAt()
          .setExpirationTime("24h")
          .sign(JWT_SECRET)

        const cookieStore = await cookies()
        cookieStore.set(AUTH_CONFIG.COOKIE_NAME, token, {
          httpOnly: true,
          secure: SECURITY_CONFIG.SECURE_COOKIES,
          sameSite: SECURITY_CONFIG.SAME_SITE as "strict" | "lax",
          maxAge: AUTH_CONFIG.COOKIE_MAX_AGE,
          path: "/",
        })

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
      const token = await new SignJWT({
        authenticated: true,
        timestamp: Date.now(),
        backend_auth: false,
      })
        .setProtectedHeader({ alg: "HS256" })
        .setIssuedAt()
        .setExpirationTime("24h")
        .sign(JWT_SECRET)

      const cookieStore = await cookies()
      cookieStore.set(AUTH_CONFIG.COOKIE_NAME, token, {
        httpOnly: true,
        secure: SECURITY_CONFIG.SECURE_COOKIES,
        sameSite: SECURITY_CONFIG.SAME_SITE as "strict" | "lax",
        maxAge: AUTH_CONFIG.COOKIE_MAX_AGE,
        path: "/",
      })

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
