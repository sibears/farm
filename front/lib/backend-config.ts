import { config } from "./config"

export const BACKEND_CONFIG = {
  BASE_URL: config.api.baseUrl,
  ENDPOINTS: config.api.endpoints,
  TIMEOUT: config.api.timeout,

  getHeaders: (token?: string) => ({
    "Content-Type": "application/json",
    ...(token && { Authorization: `Bearer ${token}` }),
  }),
}

type BackendAuthOptions = {
  bearerToken?: string
  passwordHash?: string
}

export async function callBackendAPI(endpoint: string, options: RequestInit = {}, auth?: BackendAuthOptions) {
  const url = `${BACKEND_CONFIG.BASE_URL}${endpoint}`

  try {
    const response = await fetch(url, {
      ...options,
      headers: {
        ...BACKEND_CONFIG.getHeaders(auth?.bearerToken),
        ...(auth?.passwordHash ? { "X-Authorization": auth.passwordHash } : {}),
        ...options.headers,
      },
      signal: AbortSignal.timeout(BACKEND_CONFIG.TIMEOUT),
    })

    if (!response.ok) {
      throw new Error(`Backend API error: ${response.status} ${response.statusText}`)
    }

    const data = await response.json()

    if (typeof data === "string") {
      return { success: data === "ok", data }
    }

    return { success: true, data }
  } catch (error) {
    console.error("Backend API call failed:", error)
    return { success: false, error: error instanceof Error ? error.message : "Unknown error" }
  }
}
