const env = {
  backendUrl: process.env.CTF_BACKEND_URL || process.env.NEXT_PUBLIC_CTF_BACKEND_URL || "http://localhost:8777",
  timeout: parseInt(process.env.API_TIMEOUT || process.env.NEXT_PUBLIC_API_TIMEOUT || "10000", 10),
  useMockData: process.env.USE_MOCK_DATA === "true" || process.env.NEXT_PUBLIC_USE_MOCK_DATA === "true",
  cookieName: process.env.COOKIE_NAME || "ctf-auth-token",
  cookieMaxAge: parseInt(process.env.COOKIE_MAX_AGE || "86400", 10),
  nodeEnv: process.env.NODE_ENV || "development",
} as const

export const config = {
  api: {
    baseUrl: env.backendUrl,
    timeout: env.timeout,
    useMockData: env.useMockData,
    endpoints: {
      auth: "/api/check_auth",
      flags: "/api/flags",
      flagsLimit: "/api/flags_limit",
      flagsTotal: "/api/flags/total",
      flagsStats: "/api/flags/stats",
      submitFlag: "/api/flag",
      config: "/api/config",
    },
  },

  auth: {
    cookieName: env.cookieName,
    cookieMaxAge: env.cookieMaxAge,
  },

  security: {
    secureCookies: env.nodeEnv === "production",
    sameSite: (env.nodeEnv === "production" ? "strict" : "lax") as "strict" | "lax",
  },
} as const
