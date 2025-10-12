# SiBears Front Farm 🐻

Фронтенд для системы подачи флагов CTF команды SiBears.

## 🚀 Требования

- **Bun** >= 1.0.0 (рекомендуется)
- **Node.js** >= 18.0.0 (альтернативно)

## 📦 Установка

```bash
# Клонируем репозиторий
git clone <repository-url>
cd sibears-front-farm

# Устанавливаем зависимости с Bun (рекомендуется)
bun install

# Или с npm
npm install
```

## 🔧 Настройка

1. Скопируйте файл конфигурации:

```bash
cp .env.example .env.local
```

2. Отредактируйте `.env.local`:

```env
JWT_SECRET=your-secure-jwt-secret-here
CTF_BACKEND_URL=http://your-backend-url:8080
```

## 🏃‍♂️ Запуск

### Разработка (с Bun)

```bash
bun run dev
```

### Разработка (с npm)

```bash
npm run dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

This project uses [`next/font`](https://nextjs.org/docs/app/building-your-application/optimizing/fonts) to automatically optimize and load [Geist](https://vercel.com/font), a new font family for Vercel.

## Learn More

To learn more about Next.js, take a look at the following resources:

- [Next.js Documentation](https://nextjs.org/docs) - learn about Next.js features and API.
- [Learn Next.js](https://nextjs.org/learn) - an interactive Next.js tutorial.

You can check out [the Next.js GitHub repository](https://github.com/vercel/next.js) - your feedback and contributions are welcome!

## Deploy on Vercel

The easiest way to deploy your Next.js app is to use the [Vercel Platform](https://vercel.com/new?utm_medium=default-template&filter=next.js&utm_source=create-next-app&utm_campaign=create-next-app-readme) from the creators of Next.js.

Check out our [Next.js deployment documentation](https://nextjs.org/docs/app/building-your-application/deploying) for more details.
