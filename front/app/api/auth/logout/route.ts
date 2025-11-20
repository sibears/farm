import { cookies } from "next/headers"
import { NextResponse } from "next/server"
import { AUTH_CONFIG } from "@/lib/env-config"

// Указываем, что роут динамический
export const dynamic = "force-dynamic"

export async function POST() {
  try {
    const cookieStore = await cookies()
    cookieStore.delete(AUTH_CONFIG.COOKIE_NAME)

    return NextResponse.json({
      success: true,
      message: "Logged out successfully",
    })
  } catch (error) {
    console.error("Logout error:", error)
    return NextResponse.json({ error: "Internal server error" }, { status: 500 })
  }
}
