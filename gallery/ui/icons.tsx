// The gallery's own icons: Lucide's through `nsh-icon` for the chrome
// (search, appearance, menu, copy), and GitHub's mark. The library's `Icon`
// draws only the few icons its components need; these are the gallery's.

import { Icon } from "nsh-icon";
export {
  ArrowUpRight,
  Bell,
  Check,
  Copy,
  CreditCard,
  FileText,
  House,
  Info,
  Lock,
  Mail,
  Menu,
  Monitor,
  Moon,
  Palette,
  RotateCcw,
  Search,
  SlidersHorizontal,
  Sun,
  X,
} from "nsh-icon-lucide";

/** GitHub's mark (a filled path, in the text color). */
export const GitHub =
  "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='currentColor'%3E%3Cpath d='M12 .3a12 12 0 0 0-3.8 23.4c.6.1.8-.3.8-.6v-2c-3.3.7-4-1.6-4-1.6-.6-1.4-1.4-1.8-1.4-1.8-1-.7.1-.7.1-.7 1.2.1 1.8 1.2 1.8 1.2 1 1.8 2.8 1.3 3.5 1 0-.8.4-1.3.7-1.6-2.7-.3-5.5-1.3-5.5-6 0-1.2.5-2.3 1.3-3.1-.2-.4-.6-1.6 0-3.2 0 0 1-.3 3.4 1.2a11.5 11.5 0 0 1 6 0c2.3-1.5 3.3-1.2 3.3-1.2.6 1.6.2 2.8 0 3.2.9.8 1.3 1.9 1.3 3.2 0 4.6-2.8 5.6-5.5 5.9.5.4.9 1 .9 2.2v3.3c0 .3.1.7.8.6A12 12 0 0 0 12 .3'/%3E%3C/svg%3E";

/** An icon of the chrome, 16 by default, in the text color around it. */
export function Glyph(props: { icon: string; size?: number; class?: string; label?: string }) {
  return <Icon icon={props.icon} size={props.size ?? 16} class={props.class} label={props.label} />;
}
