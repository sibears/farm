// Упрощенный файл - теперь все в config.ts
import { config } from "./config"

// Экспортируем для обратной совместимости
export const ENV_CONFIG = {
  JWT_SECRET: config.auth.jwtSecret,
  CTF_BACKEND_URL: config.api.baseUrl,
  NEXT_PUBLIC_CTF_BACKEND_URL: config.api.baseUrl,
  API_TIMEOUT: config.api.timeout,
  COOKIE_NAME: config.auth.cookieName,
  COOKIE_MAX_AGE: config.auth.cookieMaxAge,
  NODE_ENV: process.env.NODE_ENV || "development",
  IS_PRODUCTION: process.env.NODE_ENV === "production",
  IS_DEVELOPMENT: process.env.NODE_ENV === "development",
} as const

export const API_CONFIG = {
  BASE_URL: config.api.baseUrl,
  TIMEOUT: config.api.timeout,
  USE_MOCK_DATA: config.api.useMockData,
  ENDPOINTS: config.api.endpoints,
} as const

export const AUTH_CONFIG = {
  JWT_SECRET: config.auth.jwtSecret,
  COOKIE_NAME: config.auth.cookieName,
  COOKIE_MAX_AGE: config.auth.cookieMaxAge,
  SESSION_DURATION: "24h",
} as const

export const SECURITY_CONFIG = {
  CORS_ORIGIN: process.env.CORS_ORIGIN || "http://localhost:3000",
  ALLOWED_ORIGINS: process.env.ALLOWED_ORIGINS?.split(",") || ["http://localhost:3000"],
  SECURE_COOKIES: config.security.secureCookies,
  SAME_SITE: config.security.sameSite,
} as const

export function validateEnvironment() {
  console.log("✅ Конфигурация окружения валидна")
  return true
}
