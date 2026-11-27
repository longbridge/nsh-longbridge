// The website's constants: where it lives and what it links to.

/** The site's base path (`/nsh-longbridge/`), always ending in a slash. */
export const BASE = import.meta.env.BASE_URL.endsWith('/') ? import.meta.env.BASE_URL : `${import.meta.env.BASE_URL}/`;

export const SITE_TITLE = 'NSH Longbridge';
export const SITE_DESCRIPTION =
  'Components and design tokens for NSH apps, in TSX or Rust. One theme file restyles every one of them.';
export const REPOSITORY = 'https://github.com/longbridge/nsh-longbridge';
export const NSH_SITE = 'https://nsh.dev';
export const INSTALL = 'nsh add github:longbridge/nsh-longbridge';

/** A page of the gallery (`button`), or the site's root for `''`. */
export const href = (id = '') => `${BASE}${id}`;
