/** @type {import('next').NextConfig} */
const nextConfig = {
    reactStrictMode: true,
    trailingSlash: true,

    // Export settings
    output: "export",
    distDir: "./dist",

    // Optimization settings
    images: {
        unoptimized: true,
    },
}

// Bundle analyzer settings
const withBundleAnalyzer = require('@next/bundle-analyzer')({
    enabled: process.env.ANALYZE === 'true',
});

// Export the configuration
module.exports = withBundleAnalyzer(nextConfig);
