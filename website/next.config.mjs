import path from 'node:path';

const nextConfig = {
  output: 'export',
  outputFileTracingRoot: path.join(process.cwd()),
  trailingSlash: true,
  images: { unoptimized: true }
};

export default nextConfig;
