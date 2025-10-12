const isProduction = process.env.NODE_ENV === 'production'
const isDevModeForced = process.env.FORCE_DEV_MODE === 'true' || 
                        process.env.BUILD_MODE === 'development' ||
                        process.env.CTF_BACKEND_URL?.includes('localhost')

const nextConfig = {
  swcMinify: true,
  // Используем standalone только в настоящем production (не в форсированном dev режиме)
  ...(isProduction && !isDevModeForced && { output: 'standalone' }),
  
  // Настройки для разработки
  ...(process.env.NODE_ENV === 'development' && {
    eslint: {
      ignoreDuringBuilds: false,
    },
    typescript: {
      ignoreBuildErrors: false,
    },
  }),
};

export default nextConfig;
