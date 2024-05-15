/** @type {import('next').NextConfig} */
const nextConfig = {
    reactStrictMode: true,
    trailingSlash: false, // Enable trailing slash (e.g. /about/)

    // Export settings
    output: process.env.NODE_ENV === 'development' ? undefined : 'export',
    distDir: './dist',

    // Image settings
    images: {
        unoptimized: true, // Disable image optimization
    },

    // Rewrites settings
    ...(process.env.NODE_ENV === 'development'
        ? {
            rewrites: async () => {
                const rewrites = {
                    afterFiles: [],
                    fallback: [],
                };

                // Local API proxy
                rewrites.fallback.push({
                    source: '/api/:path*',
                    destination: 'http://localhost:8000/api/:path*',
                });

                return rewrites;
            },
        }
        : {}),
};

// Bundle analyzer settings
const withBundleAnalyzer = require('@next/bundle-analyzer')({
    enabled: process.env.ANALYZE === 'true',
});

// Export the configuration
module.exports = withBundleAnalyzer(nextConfig);
