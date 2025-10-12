import { validateEnvironment, ENV_CONFIG } from '@/lib/env-config'


export function initializeEnvironment() {
  console.log("🔧 Инициализация конфигурации...")
  
  // Выводим текущую конфигурацию (без секретов)
  console.log("📊 Текущие настройки:")
  console.log(`  • Режим: ${ENV_CONFIG.NODE_ENV} ${ENV_CONFIG.IS_DEV_MODE_FORCED ? "(форсирован dev)" : ""}`)
  console.log(`  • Backend URL: ${ENV_CONFIG.CTF_BACKEND_URL}`)
  console.log(`  • API Timeout: ${ENV_CONFIG.API_TIMEOUT}ms`)
  console.log(`  • Cookie Name: ${ENV_CONFIG.COOKIE_NAME}`)
  console.log(`  • Telemetry Disabled: ${ENV_CONFIG.TELEMETRY_DISABLED}`)
  
  // Маскируем секреты для безопасности
  const maskSecret = (secret: string) => {
    if (secret.length <= 8) return "*".repeat(secret.length)
    return secret.slice(0, 4) + "*".repeat(secret.length - 8) + secret.slice(-4)
  }
  
  console.log("🔐 Секреты (маскированные):")
  console.log(`  • JWT Secret: ${maskSecret(ENV_CONFIG.JWT_SECRET)}`)
  
  // Валидируем окружение
  try {
    validateEnvironment()
    console.log("🚀 Готов к запуску!")
    return true
  } catch (error) {
    console.error("❌ Критическая ошибка конфигурации:", error)
    return false
  }
}

// Инициализируем при импорте модуля (только на сервере и только один раз)
if (typeof window === 'undefined') {
  // Server-side: выводим информацию о конфигурации при старте сервера
  console.log("🌟 Загрузка конфигурации SiBears CTF Farm...")
  initializeEnvironment()
}