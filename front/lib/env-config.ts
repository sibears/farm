// Определяем реальный режим работы
const actualNodeEnv = process.env.NODE_ENV || "development"
// Форсируем dev режим если явно задано или если используем localhost backend
const isDevModeForced = process.env.FORCE_DEV_MODE === "true" || 
                        process.env.BUILD_MODE === "development" ||
                        (process.env.CTF_BACKEND_URL?.includes("localhost") && process.env.NODE_ENV !== "production")

export const ENV_CONFIG = {
  // JWT и аутентификация
  JWT_SECRET: process.env.JWT_SECRET || "ctf-farm-secret-key-change-in-production",
  
  // Backend API
  CTF_BACKEND_URL: process.env.CTF_BACKEND_URL || process.env.NEXT_PUBLIC_CTF_BACKEND_URL || "http://localhost:8777",
  NEXT_PUBLIC_CTF_BACKEND_URL: process.env.NEXT_PUBLIC_CTF_BACKEND_URL || "http://localhost:8777",
  
  // API конфигурация
  API_TIMEOUT: parseInt(process.env.API_TIMEOUT || process.env.NEXT_PUBLIC_API_TIMEOUT || "10000"),
  
  COOKIE_NAME: process.env.COOKIE_NAME || "ctf-auth-token",
  COOKIE_MAX_AGE: parseInt(process.env.COOKIE_MAX_AGE || "86400"), // 24 часа
  
  // Безопасность
  CORS_ORIGIN: process.env.CORS_ORIGIN || "http://localhost:3000",
  ALLOWED_ORIGINS: process.env.ALLOWED_ORIGINS?.split(",") || ["http://localhost:3000"],
  
  NODE_ENV: actualNodeEnv,
  PORT: parseInt(process.env.PORT || "3000"),
  HOSTNAME: process.env.HOSTNAME || "0.0.0.0",
  
  // Флаги (учитываем форсированный dev режим)
  IS_PRODUCTION: actualNodeEnv === "production" && !isDevModeForced,
  IS_DEVELOPMENT: actualNodeEnv === "development" || isDevModeForced,
  IS_DEV_MODE_FORCED: isDevModeForced,
  TELEMETRY_DISABLED: process.env.NEXT_TELEMETRY_DISABLED === "1",
} as const

// Валидация критических переменных окружения
export function validateEnvironment() {
  const errors: string[] = []
  
  // Строгая проверка только для настоящего production режима
  if (ENV_CONFIG.IS_PRODUCTION && !ENV_CONFIG.IS_DEV_MODE_FORCED) {
    if (!ENV_CONFIG.CTF_BACKEND_URL || ENV_CONFIG.CTF_BACKEND_URL.includes("localhost")) {
      errors.push(`CTF_BACKEND_URL: ${ENV_CONFIG.CTF_BACKEND_URL} должен быть настроен для продакшена!`)
    }
  }
  
  if (errors.length > 0) {
    console.error("❌ Ошибки конфигурации окружения:")
    errors.forEach(error => console.error(`  - ${error}`))
    
    // Ошибка только в настоящем production режиме
    if (ENV_CONFIG.IS_PRODUCTION && !ENV_CONFIG.IS_DEV_MODE_FORCED) {
      throw new Error("Критические ошибки конфигурации в продакшене!")
    } else {
      console.warn("⚠️ Предупреждения конфигурации (игнорируются в режиме разработки)")
    }
  }
  
  console.log("✅ Конфигурация окружения валидна")
  return true
}

// Экспорт отдельных конфигураций для удобства
export const API_CONFIG = {
  BASE_URL: ENV_CONFIG.CTF_BACKEND_URL,
  TIMEOUT: ENV_CONFIG.API_TIMEOUT,
  ENDPOINTS: {
    AUTH: "/api/check_auth",
    FLAGS: "/api/flags",
    SUBMIT_FLAG: "/api/flags/submit",
    FLAG_STATUS: "/api/flags/status",
    CONFIG: "/api/config",
  }
} as const

export const AUTH_CONFIG = {
  JWT_SECRET: ENV_CONFIG.JWT_SECRET,
  COOKIE_NAME: ENV_CONFIG.COOKIE_NAME,
  COOKIE_MAX_AGE: ENV_CONFIG.COOKIE_MAX_AGE,
  SESSION_DURATION: "24h",
} as const

export const SECURITY_CONFIG = {
  CORS_ORIGIN: ENV_CONFIG.CORS_ORIGIN,
  ALLOWED_ORIGINS: ENV_CONFIG.ALLOWED_ORIGINS,
  SECURE_COOKIES: ENV_CONFIG.IS_PRODUCTION,
  SAME_SITE: ENV_CONFIG.IS_PRODUCTION ? "strict" : "lax",
} as const