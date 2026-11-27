// Every page of the gallery, in the sidebar's groups and order.

import type { JSX } from "nsh";
import { BadgePage } from "./pages/badge";
import { ButtonPage } from "./pages/button";
import { CardPage } from "./pages/card";
import { CheckboxPage } from "./pages/checkbox";
import { ColorsPage } from "./pages/colors";
import { CustomizePage } from "./pages/customize";
import { DropdownMenuPage } from "./pages/dropdown-menu";
import { HomePage } from "./pages/home";
import { IconsPage } from "./pages/icons";
import { InputPage } from "./pages/input";
import { KbdPage } from "./pages/kbd";
import { LabelPage } from "./pages/label";
import { RadioGroupPage } from "./pages/radio-group";
import { SelectPage } from "./pages/select";
import { SeparatorPage } from "./pages/separator";
import { SizesPage } from "./pages/sizes";
import { SkeletonPage } from "./pages/skeleton";
import { SpinnerPage } from "./pages/spinner";
import { SwitchPage } from "./pages/switch";
import { TabsPage } from "./pages/tabs";
import { TextareaPage } from "./pages/textarea";
import { TooltipPage } from "./pages/tooltip";
import { TypographyPage } from "./pages/typography";

export interface PageEntry {
  id: string;
  label: string;
  page: () => JSX.Element;
  /** The page takes the window's width (the home page, the theme editor). */
  wide?: boolean;
}

export interface PageGroup {
  title: string;
  pages: PageEntry[];
}

export const catalog: PageGroup[] = [
  {
    title: "Overview",
    pages: [
      { id: "home", label: "Home", page: HomePage, wide: true },
      { id: "customize", label: "Customize", page: CustomizePage, wide: true },
    ],
  },
  {
    title: "Components",
    pages: [
      { id: "badge", label: "Badge", page: BadgePage },
      { id: "button", label: "Button", page: ButtonPage },
      { id: "card", label: "Card", page: CardPage },
      { id: "checkbox", label: "Checkbox", page: CheckboxPage },
      { id: "dropdown-menu", label: "Dropdown Menu", page: DropdownMenuPage },
      { id: "input", label: "Input", page: InputPage },
      { id: "kbd", label: "Kbd", page: KbdPage },
      { id: "label", label: "Label", page: LabelPage },
      { id: "radio-group", label: "Radio Group", page: RadioGroupPage },
      { id: "select", label: "Select", page: SelectPage },
      { id: "separator", label: "Separator", page: SeparatorPage },
      { id: "skeleton", label: "Skeleton", page: SkeletonPage },
      { id: "spinner", label: "Spinner", page: SpinnerPage },
      { id: "switch", label: "Switch", page: SwitchPage },
      { id: "tabs", label: "Tabs", page: TabsPage },
      { id: "textarea", label: "Textarea", page: TextareaPage },
      { id: "tooltip", label: "Tooltip", page: TooltipPage },
    ],
  },
  {
    title: "Foundation",
    pages: [
      { id: "colors", label: "Colors", page: ColorsPage },
      { id: "typography", label: "Typography", page: TypographyPage },
      { id: "sizes", label: "Radius and sizes", page: SizesPage },
      { id: "icons", label: "Icons", page: IconsPage },
    ],
  },
];

export const pages = catalog.flatMap((g) => g.pages);
export const pageById = (id: string) => pages.find((p) => p.id === id) ?? pages[0];
export const componentPages = catalog[1].pages;
