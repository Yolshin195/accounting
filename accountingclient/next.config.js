/** @type {import('next').NextConfig} */
const nextConfig = {
  eslint: {
    ignoreDuringBuilds: true,
  },
  typescript: {
    ignoreBuildErrors: true,
  },
  images: {
    unoptimized: true,
  },
  // Добавляем поддержку standalone для Docker
  output: "standalone",
  async rewrites() {
    return [
      {
        source: "/api/backend/:path*",
        destination: (process.env.BACKEND_INTERNAL_URL + "/:path*") || "http://localhost:8888/:path*", // Проксирование на ваш бэкенд-сервис
      },
    ]
  },
}

module.exports = nextConfig
