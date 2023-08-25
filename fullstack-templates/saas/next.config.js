/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  output: "export",
  trailingSlash: true,
  distDir: "./backend/public",
    images: {
    unoptimized: true,
  },
}

module.exports = nextConfig
