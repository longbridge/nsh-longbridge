// The website's home page: static HTML and CSS around the live demos, which
// are the gallery's web build in iframes (`embed/<name>`). `scripts/build-site.sh`
// puts this build and the gallery's together into the Pages site.
import { defineConfig } from 'astro/config';
import tailwindcss from '@tailwindcss/vite';

const configured = process.env.PUBLIC_SITE_BASE || '/nsh-longbridge/';
const BASE = configured === '/' ? '/' : `/${configured.replace(/^\/+|\/+$/g, '')}/`;

export default defineConfig({
  site: process.env.PUBLIC_SITE_URL || 'https://longbridge.github.io',
  base: BASE,
  outDir: process.env.SITE_OUT_DIR || './dist',
  output: 'static',
  trailingSlash: 'ignore',
  // The gallery's files share the site's root; Astro's go under `_site/`.
  build: { assets: '_site', inlineStylesheets: 'always' },
  vite: { plugins: [tailwindcss()] },
});
