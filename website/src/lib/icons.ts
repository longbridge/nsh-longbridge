// The icons the page draws: Lucide's (ISC, the set the gallery's chrome uses)
// as their SVG bodies, and GitHub's mark.

const lucide = {
  arrowRight: "<path d='M5 12h14'/><path d='m12 5 7 7-7 7'/>",
  arrowUpRight: "<path d='M7 7h10v10'/><path d='M7 17 17 7'/>",
  calendar: "<path d='M8 2v3'/><path d='M16 2v3'/><rect x='3' y='3' width='18' height='18' rx='2'/><path d='M3 9h18'/>",
  check: "<path d='M20 6 9 17l-5-5'/>",
  chevronDown: "<path d='m6 9 6 6 6-6'/>",
  chevronsUpDown: "<path d='m7 15 5 5 5-5'/><path d='m7 9 5-5 5 5'/>",
  copy: "<rect width='14' height='14' x='8' y='8' rx='2' ry='2'/><path d='M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2'/>",
  monitor: "<rect width='20' height='14' x='2' y='3' rx='2'/><line x1='8' x2='16' y1='21' y2='21'/><line x1='12' x2='12' y1='17' y2='21'/>",
  moon: "<path d='M20.985 12.486a9 9 0 1 1-9.473-9.472c.405-.022.617.46.402.803a6 6 0 0 0 8.268 8.268c.344-.215.825-.004.803.401'/>",
  pause: "<rect x='14' y='3' width='5' height='18' rx='1'/><rect x='5' y='3' width='5' height='18' rx='1'/>",
  sun: "<circle cx='12' cy='12' r='4'/><path d='M12 2v2'/><path d='M12 20v2'/><path d='m4.93 4.93 1.41 1.41'/><path d='m17.66 17.66 1.41 1.41'/><path d='M2 12h2'/><path d='M20 12h2'/><path d='m6.34 17.66-1.41 1.41'/><path d='m19.07 4.93-1.41 1.41'/>",
};

export type IconName = keyof typeof lucide;

/** A Lucide icon as inline SVG, `size` pixels, in the text's color. */
export const icon = (name: IconName, size = 16, cls = '') =>
  `<svg class="icon ${cls}" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">${lucide[name]}</svg>`;

/** GitHub's mark, filled in the text's color. */
export const github = (size = 16) =>
  `<svg class="icon" width="${size}" height="${size}" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M12 .3a12 12 0 0 0-3.8 23.4c.6.1.8-.3.8-.6v-2c-3.3.7-4-1.6-4-1.6-.6-1.4-1.4-1.8-1.4-1.8-1-.7.1-.7.1-.7 1.2.1 1.8 1.2 1.8 1.2 1 1.8 2.8 1.3 3.5 1 0-.8.4-1.3.7-1.6-2.7-.3-5.5-1.3-5.5-6 0-1.2.5-2.3 1.3-3.1-.2-.4-.6-1.6 0-3.2 0 0 1-.3 3.4 1.2a11.5 11.5 0 0 1 6 0c2.3-1.5 3.3-1.2 3.3-1.2.6 1.6.2 2.8 0 3.2.9.8 1.3 1.9 1.3 3.2 0 4.6-2.8 5.6-5.5 5.9.5.4.9 1 .9 2.2v3.3c0 .3.1.7.8.6A12 12 0 0 0 12 .3"/></svg>`;
