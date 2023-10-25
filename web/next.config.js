/** @type {import('next').NextConfig} */
const basePath = process.env.NEXT_PUBLIC_BASEPATH;
const nextConfig = {
  images: { domains: ["api.dicebear.com", "127.0.0.1"] },
  basePath,
};

module.exports = nextConfig;
