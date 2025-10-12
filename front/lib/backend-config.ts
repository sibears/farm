import { API_CONFIG } from './env-config'

export const BACKEND_CONFIG = {
  // URL Rust бэкенда из переменных окружения
  BASE_URL: API_CONFIG.BASE_URL,

  // API endpoints
  ENDPOINTS: API_CONFIG.ENDPOINTS,

  // Request configuration
  TIMEOUT: API_CONFIG.TIMEOUT,

  // Headers for API requests
  getHeaders: (token?: string) => ({
    "Content-Type": "application/json",
    ...(token && { Authorization: `Bearer ${token}` }),
  }),
}

// Helper function to make API calls to your backend
export async function callBackendAPI(endpoint: string, options: RequestInit = {}, token?: string) {
  const url = `${BACKEND_CONFIG.BASE_URL}${endpoint}`
  console.log(`url: ${url}`)
  try {
    const response = await fetch(url, {
      ...options,
      headers: {
        ...BACKEND_CONFIG.getHeaders(token),
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
