// The gallery's one `Toaster`, at the root of the shell: the Toast page
// changes where it is, whether toasts show the "×" and which show the
// remaining-time bar.

import { signal } from "nsh";
import { Toaster, type ToastPlacement } from "nsh-longbridge";

export const toasterPlacement = signal<ToastPlacement>("bottomRight");
export const toasterCloseButton = signal(false);
/** The `Toaster`'s `timer`: left out (`undefined`), on or off. */
export const toasterTimer = signal<boolean | undefined>(undefined);

export function GalleryToaster() {
  return <Toaster placement={toasterPlacement()} closeButton={toasterCloseButton()} timer={toasterTimer()} />;
}
