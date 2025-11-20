import { type NextRequest, NextResponse } from "next/server"
import { verifyAuth } from "@/lib/auth-middleware"

// Указываем, что роут динамический
export const dynamic = "force-dynamic"

interface Flag {
  id: number
  flag: string
  sploit?: string
  team?: string
  created_time?: string
  start_waiting_time?: string
  status?: string
  checksystem_response?: string
}

// This would be replaced with actual database in production
const flagsDatabase: Flag[] = [
  // Same mock data as in flags/route.ts - could be populated from shared source
]

export async function GET(request: NextRequest, { params }: { params: { id: string } }) {
  try {
    // Verify authentication
    const authContext = await verifyAuth(request)
    if (!authContext.authenticated) {
      return NextResponse.json({ error: "Authentication required" }, { status: 401 })
    }

    const flagId = Number.parseInt(params.id, 10)
    const flag = flagsDatabase.find((f) => f.id === flagId)

    if (!flag) {
      return NextResponse.json({ error: "Flag not found" }, { status: 404 })
    }

    return NextResponse.json({ flag })
  } catch (error) {
    console.error("Error fetching flag:", error)
    return NextResponse.json({ error: "Internal server error" }, { status: 500 })
  }
}

export async function PATCH(request: NextRequest, { params }: { params: { id: string } }) {
  try {
    // Verify authentication
    const authContext = await verifyAuth(request)
    if (!authContext.authenticated) {
      return NextResponse.json({ error: "Authentication required" }, { status: 401 })
    }

    const flagId = Number.parseInt(params.id, 10)
    const flagIndex = flagsDatabase.findIndex((f) => f.id === flagId)

    if (flagIndex === -1) {
      return NextResponse.json({ error: "Flag not found" }, { status: 404 })
    }

    const { status, checksystem_response } = await request.json()

    // Update flag
    if (status) {
      flagsDatabase[flagIndex].status = status
    }
    if (checksystem_response !== undefined) {
      flagsDatabase[flagIndex].checksystem_response = checksystem_response
    }

    return NextResponse.json({
      success: true,
      flag: flagsDatabase[flagIndex],
      message: "Flag updated successfully",
    })
  } catch (error) {
    console.error("Error updating flag:", error)
    return NextResponse.json({ error: "Internal server error" }, { status: 500 })
  }
}
