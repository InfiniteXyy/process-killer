const AutoImport = require('unplugin-auto-import/webpack');
const { i18n } = require('./next-i18next.config');

/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  webpack(config) {
    config.plugins.push(AutoImport({ imports: ['react'] }));
    return config;
  },
  i18n,
};

module.exports = nextConfig;
