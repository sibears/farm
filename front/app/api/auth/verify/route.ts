import { type NextRequest, NextResponse } from "next/server"
import { verifyAuth } from "@/lib/auth-middleware"

// Указываем, что роут динамический
export const dynamic = "force-dynamic"

export async function GET(request: NextRequest) {
  try {
    const authContext = await verifyAuth(request)

    if (!authContext.authenticated) {
      return NextResponse.json({ error: "No authentication cookie found" }, { status: 401 })
    }

    return NextResponse.json({
      authenticated: true,
      message: "Authentication cookie is valid",
    })
  } catch (error) {
    console.error("Token verification error:", error)
    return NextResponse.json({ error: "Failed to verify authentication" }, { status: 401 })
  }
}
