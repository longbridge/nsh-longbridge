// The gallery's one `Toaster`, at the root of the shell: the Toast page
// changes where it is and whether toasts show the "×".

import { signal } from "nsh";
import { Toaster, type ToastPlacement } from "nsh-longbridge";

export const toasterPlacement = signal<ToastPlacement>("bottomRight");
export const toasterCloseButton = signal(false);

export function GalleryToaster() {
  return <Toaster placement={toasterPlacement()} closeButton={toasterCloseButton()} />;
}
