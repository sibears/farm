import { type NextRequest, NextResponse } from "next/server"
import { verifyAuth } from "@/lib/auth-middleware"
import { BACKEND_CONFIG, callBackendAPI } from "@/lib/backend-config"
import { API_CONFIG } from "@/lib/env-config"
import { FlagStatus, type FlagType } from "@/lib/types"

export const dynamic = "force-dynamic"

type RawFlag = {
  id?: number
  flag?: string
  sploit?: string | null
  team?: string | null
  created_time?: string
  start_waiting_time?: string | null
  status?: FlagStatus | string
  checksystem_response?: string | null
}

const VALID_FLAG_STATUSES = new Set(Object.values(FlagStatus))

function isFlagStatus(value: unknown): value is FlagStatus {
  return typeof value === "string" && VALID_FLAG_STATUSES.has(value as FlagStatus)
}

function normalizeFlagRecord(raw: RawFlag): FlagType | null {
  if (
    typeof raw?.id !== "number" ||
    typeof raw.flag !== "string" ||
    typeof raw.created_time !== "string" ||
    raw.status === undefined
  ) {
    return null
  }

  return {
    id: raw.id,
    flag: raw.flag,
    created_time: raw.created_time,
    status: isFlagStatus(raw.status) ? (raw.status as FlagStatus) : FlagStatus.QUEUED,
    sploit: raw.sploit ?? undefined,
    team: raw.team ?? undefined,
    start_waiting_time: raw.start_waiting_time ?? undefined,
    checksystem_response: raw.checksystem_response ?? undefined,
  }
}

function normalizeFlagsResponse(data: unknown): { flags: FlagType[]; total?: number } {
  if (Array.isArray(data)) {
    return {
      flags: data
        .map((entry) => normalizeFlagRecord(entry as RawFlag))
        .filter((flag): flag is FlagType => Boolean(flag)),
    }
  }

  if (data && typeof data === "object") {
    const payload = data as { flags?: unknown[]; total?: unknown }
    if (Array.isArray(payload.flags)) {
      const nested = normalizeFlagsResponse(payload.flags)
      return {
        flags: nested.flags,
        total: typeof payload.total === "number" ? payload.total : nested.total,
      }
    }
  }

  return { flags: [] }
}

function buildFlagsEndpoint(searchParams: URLSearchParams) {
  const queryString = searchParams.toString()
  const normalizedQuery = queryString ? `?${queryString}` : ""
  const hasPagination = searchParams.has("limit") || searchParams.has("offset")
  return hasPagination
    ? `${BACKEND_CONFIG.ENDPOINTS.flagsLimit}${normalizedQuery}`
    : `${BACKEND_CONFIG.ENDPOINTS.flags}${normalizedQuery}`
}

// biome-ignore lint/complexity/noExcessiveCognitiveComplexity: API route handler with multiple conditions for backwards compatibility
export async function GET(request: NextRequest) {
  try {
    const authContext = await verifyAuth(request)
    if (!authContext.authenticated) {
      return NextResponse.json({ error: "Authentication required" }, { status: 401 })
    }

    if (API_CONFIG.USE_MOCK_DATA) {
      return getMockFlags()
    }

    if (!authContext.passwordHash) {
      return NextResponse.json({ error: "Missing authentication hash" }, { status: 403 })
    }

    const requestUrl = new URL(request.url)
    const endpoint = buildFlagsEndpoint(requestUrl.searchParams)
    const backendResponse = await callBackendAPI(endpoint, {}, { passwordHash: authContext.passwordHash })

    if (backendResponse.success && backendResponse.data) {
      const normalized = normalizeFlagsResponse(backendResponse.data)

      if (requestUrl.searchParams.has("limit") || requestUrl.searchParams.has("offset")) {
        let total: number | undefined
        try {
          const totalResponse = await callBackendAPI(
            BACKEND_CONFIG.ENDPOINTS.flagsTotal,
            {},
            { passwordHash: authContext.passwordHash },
          )
          if (totalResponse.success && typeof totalResponse.data === "number") {
            total = totalResponse.data
          }
        } catch (error) {
          console.error("Failed to fetch flags total:", error)
        }

        return NextResponse.json({
          flags: normalized.flags,
          total: total ?? normalized.total ?? normalized.flags.length,
        })
      }

      return NextResponse.json(normalized.flags)
    }

    throw new Error(backendResponse.error || "Backend response missing data")
  } catch (error) {
    console.error("Error fetching flags:", error)
    if (API_CONFIG.USE_MOCK_DATA) {
      return getMockFlags()
    }

    return NextResponse.json(
      { error: error instanceof Error ? error.message : "Internal server error" },
      { status: 502 },
    )
  }
}

async function validateFlagFormat(
  flag: string,
  authContext: { passwordHash?: string },
): Promise<{ valid: boolean; error?: string }> {
  if (!flag) {
    return { valid: false, error: "Flag is required" }
  }

  if (API_CONFIG.USE_MOCK_DATA) {
    return { valid: true }
  }

  if (!authContext.passwordHash) {
    return { valid: false, error: "Missing authentication hash" }
  }

  try {
    const configResponse = await callBackendAPI(
      BACKEND_CONFIG.ENDPOINTS.config,
      {},
      { passwordHash: authContext.passwordHash },
    )

    if (!configResponse.success || !configResponse.data || typeof configResponse.data !== "object") {
      console.error("Failed to get config from backend")
      return { valid: false, error: "Failed to get configuration" }
    }

    const config = configResponse.data as { ctf?: { flag_format?: string } }
    const flagFormat = config?.ctf?.flag_format

    if (!flagFormat || typeof flagFormat !== "string") {
      console.error("Flag format not found in config")
      return { valid: false, error: "Flag format not found in configuration" }
    }

    const flagRegex = new RegExp(flagFormat)
    if (!flagRegex.test(flag)) {
      return { valid: false, error: `Flag must match format: ${flagFormat}` }
    }

    return { valid: true }
  } catch (error) {
    console.error("Error validating flag format:", error)
    return { valid: false, error: "Error validating flag format" }
  }
}

// biome-ignore lint/complexity/noExcessiveCognitiveComplexity: API route handler with validation and error handling logic
export async function POST(request: NextRequest) {
  let requestBody: { flag?: string; sploit?: string; team?: string } | undefined
  try {
    const authContext = await verifyAuth(request)
    if (!authContext.authenticated) {
      return NextResponse.json({ error: "Authentication required" }, { status: 401 })
    }

    requestBody = await request.json()
    if (!requestBody || typeof requestBody !== "object") {
      return NextResponse.json({ error: "Invalid request body" }, { status: 400 })
    }

    const { flag } = requestBody
    if (!flag || typeof flag !== "string") {
      return NextResponse.json({ error: "Flag is required" }, { status: 400 })
    }

    const validation = await validateFlagFormat(flag, authContext)
    if (!validation.valid) {
      return NextResponse.json({ error: validation.error || "Invalid flag format" }, { status: 400 })
    }

    if (API_CONFIG.USE_MOCK_DATA) {
      return submitMockFlag({ flag, sploit: requestBody.sploit, team: requestBody.team })
    }

    if (!authContext.passwordHash) {
      return NextResponse.json({ error: "Missing authentication hash" }, { status: 403 })
    }

    const backendPayload = {
      flag,
      sploit: requestBody.sploit || null,
      team: requestBody.team || null,
    }

    const backendResponse = await callBackendAPI(
      BACKEND_CONFIG.ENDPOINTS.submitFlag,
      {
        method: "POST",
        body: JSON.stringify(backendPayload),
      },
      { passwordHash: authContext.passwordHash },
    )

    if (backendResponse.success) {
      return NextResponse.json(
        {
          success: true,
          message: "Flag submitted successfully",
          data: backendResponse.data,
        },
        { status: 201 },
      )
    }

    throw new Error(backendResponse.error || "Backend submission failed")
  } catch (error) {
    console.error("Error submitting flag:", error)

    if (API_CONFIG.USE_MOCK_DATA && requestBody?.flag) {
      return submitMockFlag(requestBody as { flag: string; sploit?: string; team?: string })
    }

    return NextResponse.json(
      { error: error instanceof Error ? error.message : "Internal server error" },
      { status: 502 },
    )
  }
}

function getMockFlags() {
  const mockFlags: FlagType[] = [
    {
      id: 1,
      flag: "EXAMPLE_FLAG_1",
      sploit: "exploit_script.py",
      team: "Team Alpha",
      created_time: "2024-01-15T10:30:00",
      start_waiting_time: "2024-01-15T10:25:00",
      status: FlagStatus.ACCEPTED,
      checksystem_response: "Flag verified successfully",
    },
    {
      id: 2,
      flag: "EXAMPLE_FLAG_2",
      team: "Team Beta",
      created_time: "2024-01-15T11:15:00",
      status: FlagStatus.QUEUED,
      checksystem_response: undefined,
    },
  ]

  return NextResponse.json(mockFlags)
}

function submitMockFlag(requestBody: { flag: string; sploit?: string; team?: string }) {
  const newFlag: FlagType = {
    id: Date.now(),
    flag: requestBody.flag,
    sploit: requestBody.sploit,
    team: requestBody.team,
    created_time: new Date().toISOString(),
    start_waiting_time: undefined,
    status: FlagStatus.QUEUED,
    checksystem_response: undefined,
  }

  return NextResponse.json(
    {
      success: true,
      data: newFlag,
      message: "Flag submitted successfully (mock mode)",
    },
    { status: 201 },
  )
}
