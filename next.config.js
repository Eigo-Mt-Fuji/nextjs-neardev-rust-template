/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  env: {
     NEXT_PUBLIC_CONTRACT_NAME: process.env.NEXT_PUBLIC_CONTRACT_NAME,
  }
}

module.exports = nextConfig
