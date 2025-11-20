import { type NextRequest, NextResponse } from "next/server"
import { verifyAuth } from "@/lib/auth-middleware"
import { BACKEND_CONFIG, callBackendAPI } from "@/lib/backend-config"

export const dynamic = "force-dynamic"

export async function GET(request: NextRequest) {
  try {
    const authContext = await verifyAuth(request)
    if (!authContext.authenticated) {
      return NextResponse.json({ error: "Authentication required" }, { status: 401 })
    }

    if (!authContext.passwordHash) {
      return NextResponse.json({ error: "Missing authentication hash" }, { status: 403 })
    }

    const backendResponse = await callBackendAPI(
      BACKEND_CONFIG.ENDPOINTS.flagsTotal,
      {},
      { passwordHash: authContext.passwordHash },
    )

    if (backendResponse.success) {
      return NextResponse.json(backendResponse.data)
    }

    throw new Error(backendResponse.error || "Failed to fetch total flags count")
  } catch (error) {
    console.error("Error fetching total flags:", error)
    return NextResponse.json(
      { error: error instanceof Error ? error.message : "Internal server error" },
      { status: 502 },
    )
  }
}
