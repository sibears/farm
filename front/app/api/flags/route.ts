import { type NextRequest, NextResponse } from "next/server"
import { verifyAuth } from "@/lib/auth-middleware"
import { callBackendAPI, BACKEND_CONFIG } from "@/lib/backend-config"

// Указываем, что роут динамический
export const dynamic = 'force-dynamic'

export async function GET(request: NextRequest) {
  try {
    // Verify authentication
    const isAuthenticated = await verifyAuth(request)
    if (!isAuthenticated) {
      return NextResponse.json({ error: "Authentication required" }, { status: 401 })
    }

    // Parse query parameters for filtering and pagination
    const { searchParams } = new URL(request.url)
    const queryParams = new URLSearchParams()

    // Forward all query parameters to backend
    searchParams.forEach((value, key) => {
      queryParams.append(key, value)
    })

    try {
      const backendResponse = await callBackendAPI(`${BACKEND_CONFIG.ENDPOINTS.FLAGS}?${queryParams.toString()}`)

      return NextResponse.json(backendResponse)
    } catch (backendError) {
      console.error("Backend API error:", backendError)

      console.log("Falling back to mock data")
      return getMockFlags(request)
    }
  } catch (error) {
    console.error("Error fetching flags:", error)
    return NextResponse.json({ error: "Internal server error" }, { status: 500 })
  }
}

export async function POST(request: NextRequest) {
  try {
    // Verify authentication
    const isAuthenticated = await verifyAuth(request)
    if (!isAuthenticated) {
      return NextResponse.json({ error: "Authentication required" }, { status: 401 })
    }

    const requestBody = await request.json()
    const { flag } = requestBody

    // Basic validation
    if (!flag || !flag.startsWith("CTF{") || !flag.endsWith("}")) {
      return NextResponse.json({ error: "Flag must be in CTF{...} format" }, { status: 400 })
    }

    try {
      const backendResponse = await callBackendAPI(BACKEND_CONFIG.ENDPOINTS.SUBMIT_FLAG, {
        method: "POST",
        body: JSON.stringify(requestBody),
      })

      return NextResponse.json(backendResponse, { status: 201 })
    } catch (backendError) {
      console.error("Backend submission error:", backendError)

      console.log("Falling back to mock submission")
      return submitMockFlag(requestBody)
    }
  } catch (error) {
    console.error("Error submitting flag:", error)
    return NextResponse.json({ error: "Internal server error" }, { status: 500 })
  }
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
async function getMockFlags(_request: NextRequest) {
  const mockFlags = [
    {
      id: 1,
      flag: "CTF{example_flag_1}",
      sploit: "exploit_script.py",
      team: "Team Alpha",
      created_time: "2024-01-15T10:30:00",
      start_waiting_time: "2024-01-15T10:25:00",
      status: "Accepted",
      checksystem_response: "Flag verified successfully",
    },
    {
      id: 2,
      flag: "CTF{example_flag_2}",
      team: "Team Beta",
      created_time: "2024-01-15T11:15:00",
      status: "Pending",
    },
  ]

  return NextResponse.json({
    flags: mockFlags,
    pagination: { page: 1, limit: 10, total: 2, totalPages: 1 },
    stats: { total: 2, accepted: 1, pending: 1, rejected: 0, processing: 0, teams: 2 },
  })
}

async function submitMockFlag(requestBody: { flag: string; sploit?: string; team?: string }) {
  const newFlag = {
    id: Date.now(),
    ...requestBody,
    created_time: new Date().toISOString(),
    start_waiting_time: new Date().toISOString(),
    status: "Pending",
    checksystem_response: undefined,
  }

  return NextResponse.json(
    {
      success: true,
      flag: newFlag,
      message: "Flag submitted successfully (mock mode)",
    },
    { status: 201 },
  )
}
