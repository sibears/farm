import { Analytics } from "@vercel/analytics/next"
import { GeistMono } from "geist/font/mono"
import { GeistSans } from "geist/font/sans"
import type { Metadata } from "next"
import type React from "react"
import { Suspense } from "react"
import { QueryProvider } from "@/lib/query-provider"
import "./globals.css"

export const metadata: Metadata = {
  title: "SiBears CTF Farm - Flag Submission",
  description: "SiBears CTF Farm - Flag Submission System",
  generator: "Next.js + Bun",
}

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode
}>) {
  return (
    <html lang="en">
      <body className={`font-sans ${GeistSans.variable} ${GeistMono.variable}`}>
        <QueryProvider>
          <Suspense fallback={null}>
            {children}
            <Analytics />
          </Suspense>
        </QueryProvider>
      </body>
    </html>
  )
}
