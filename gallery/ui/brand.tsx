// The gallery's brand: NSH's mark (the shell as a lit citrus tube holding a
// bead, nsh.dev's logo, in its light and dark colorings) and the family's
// name, which the header follows with the product's: "NSH / Longbridge".

import { Image, Text, View } from "nsh";
import logoDark from "../brand/nsh-logo-dark.svg";
import logoLight from "../brand/nsh-logo-light.svg";
import { scheme } from "../theme";
import { DISPLAY, sites } from "./site";

/** The appearance a brand is drawn for: `dark` when given, else the shown one. */
const isDark = (dark?: boolean) => dark ?? scheme() === "dark";

/** NSH's mark, `size` points square, in the shown appearance's coloring (or `dark`'s). */
export function Logo(props: { size?: number; dark?: boolean }) {
  const s = props.size ?? 24;
  return (
    <Image
      src={isDark(props.dark) ? logoDark : logoLight}
      style={{ width: s, height: s }}
      accessible={false}
    />
  );
}

/**
 * The family's mark and name, "NSH", as the header shows it before the
 * product ("NSH / Longbridge"): Figtree, semibold. `name`: false for the
 * mark alone (a narrow header).
 */
export function Brand(props: { scale?: number; dark?: boolean; name?: boolean }) {
  const k = props.scale ?? 1;
  const site = () => sites[isDark(props.dark) ? "dark" : "light"];
  return (
    <View class="flex-row items-center" style={{ gap: 9 * k }}>
      <Logo size={22 * k} dark={props.dark} />
      {props.name !== false ? (
        <Text style={{ fontFamily: DISPLAY, color: site().ink, fontSize: 16 * k, fontWeight: "650", letterSpacing: -0.16 * k }}>NSH</Text>
      ) : null}
    </View>
  );
}

/** The product's name, the second half of the header's brand. */
export const PRODUCT = "NSH Longbridge";
