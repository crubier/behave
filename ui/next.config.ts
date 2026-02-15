import type { NextConfig } from "next";
import path from "path";

const nextConfig: NextConfig = {
  // Extend the file-tracing root to the repo root so Next.js can
  // resolve imports from the colocated actions/ directory.
  outputFileTracingRoot: path.resolve(__dirname, ".."),
  turbopack: {},
};

export default nextConfig;
