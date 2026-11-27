// Every page of the gallery, in the sidebar's groups and order.

import type { JSX } from "nsh";
import { AlertDialogPage } from "./pages/alert-dialog";
import { AvatarPage } from "./pages/avatar";
import { BadgePage } from "./pages/badge";
import { ButtonPage } from "./pages/button";
import { CalendarPage } from "./pages/calendar";
import { CardPage } from "./pages/card";
import { CheckboxPage } from "./pages/checkbox";
import { ColorsPage } from "./pages/colors";
import { ComboboxPage } from "./pages/combobox";
import { CustomizePage } from "./pages/customize";
import { DataTablePage } from "./pages/data-table";
import { DatePickerPage } from "./pages/date-picker";
import { DialogPage } from "./pages/dialog";
import { DropdownMenuPage } from "./pages/dropdown-menu";
import { FieldPage } from "./pages/field";
import { HomePage } from "./pages/home";
import { IconsPage } from "./pages/icons";
import { InputPage } from "./pages/input";
import { InputGroupPage } from "./pages/input-group";
import { KbdPage } from "./pages/kbd";
import { LabelPage } from "./pages/label";
import { PaginationPage } from "./pages/pagination";
import { PopoverPage } from "./pages/popover";
import { ProgressPage } from "./pages/progress";
import { RadioGroupPage } from "./pages/radio-group";
import { SelectPage } from "./pages/select";
import { SelectionBarPage } from "./pages/selection-bar";
import { SeparatorPage } from "./pages/separator";
import { SizesPage } from "./pages/sizes";
import { SkeletonPage } from "./pages/skeleton";
import { SliderPage } from "./pages/slider";
import { SpinnerPage } from "./pages/spinner";
import { SwitchPage } from "./pages/switch";
import { TablePage } from "./pages/table";
import { TabsPage } from "./pages/tabs";
import { TextareaPage } from "./pages/textarea";
import { ToastPage } from "./pages/toast";
import { TogglePage } from "./pages/toggle";
import { ToggleGroupPage } from "./pages/toggle-group";
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
      { id: "alert-dialog", label: "Alert Dialog", page: AlertDialogPage },
      { id: "avatar", label: "Avatar", page: AvatarPage },
      { id: "badge", label: "Badge", page: BadgePage },
      { id: "button", label: "Button", page: ButtonPage },
      { id: "calendar", label: "Calendar", page: CalendarPage },
      { id: "card", label: "Card", page: CardPage },
      { id: "checkbox", label: "Checkbox", page: CheckboxPage },
      { id: "combobox", label: "Combobox", page: ComboboxPage },
      { id: "data-table", label: "Data Table", page: DataTablePage },
      { id: "date-picker", label: "Date Picker", page: DatePickerPage },
      { id: "dialog", label: "Dialog", page: DialogPage },
      { id: "dropdown-menu", label: "Dropdown Menu", page: DropdownMenuPage },
      { id: "field", label: "Field", page: FieldPage },
      { id: "input", label: "Input", page: InputPage },
      { id: "input-group", label: "Input Group", page: InputGroupPage },
      { id: "kbd", label: "Kbd", page: KbdPage },
      { id: "label", label: "Label", page: LabelPage },
      { id: "pagination", label: "Pagination", page: PaginationPage },
      { id: "popover", label: "Popover", page: PopoverPage },
      { id: "progress", label: "Progress", page: ProgressPage },
      { id: "radio-group", label: "Radio Group", page: RadioGroupPage },
      { id: "select", label: "Select", page: SelectPage },
      { id: "selection-bar", label: "Selection Bar", page: SelectionBarPage },
      { id: "separator", label: "Separator", page: SeparatorPage },
      { id: "skeleton", label: "Skeleton", page: SkeletonPage },
      { id: "slider", label: "Slider", page: SliderPage },
      { id: "spinner", label: "Spinner", page: SpinnerPage },
      { id: "switch", label: "Switch", page: SwitchPage },
      { id: "table", label: "Table", page: TablePage },
      { id: "tabs", label: "Tabs", page: TabsPage },
      { id: "textarea", label: "Textarea", page: TextareaPage },
      { id: "toast", label: "Toast", page: ToastPage },
      { id: "toggle", label: "Toggle", page: TogglePage },
      { id: "toggle-group", label: "Toggle Group", page: ToggleGroupPage },
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
