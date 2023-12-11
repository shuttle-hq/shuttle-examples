/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  output: "export",
  trailingSlash: true,
  distDir: "./dist",
    images: {
    unoptimized: true,
  },
}

module.exports = nextConfig
