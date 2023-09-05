/** @type {import('next').NextConfig} */
const nextConfig = {
	experimental: {
    serverActions: true,
  },
  images: { domains: ['api.dicebear.com'], },
}

module.exports = nextConfig
