// The library's components as the home page lists them: each one's page in
// the gallery (`gallery/catalog.ts` has the same ids), its kind and a line
// about it.

export type Kind = 'Actions' | 'Forms' | 'Overlays' | 'Display';

export const kinds: Kind[] = ['Actions', 'Forms', 'Overlays', 'Display'];

export const components: { id: string; label: string; kind: Kind; line: string }[] = [
  { id: 'alert-dialog', label: 'Alert Dialog', kind: 'Overlays', line: "Confirms what can't be undone." },
  { id: 'avatar', label: 'Avatar', kind: 'Display', line: 'A face or initials, alone or grouped.' },
  { id: 'badge', label: 'Badge', kind: 'Display', line: 'A small pill for a status or a count.' },
  { id: 'button', label: 'Button', kind: 'Actions', line: 'Six variants, four sizes, icons.' },
  { id: 'calendar', label: 'Calendar', kind: 'Forms', line: 'A month to pick a day or a range in.' },
  { id: 'card', label: 'Card', kind: 'Display', line: 'A surface with header, body, actions.' },
  { id: 'checkbox', label: 'Checkbox', kind: 'Forms', line: 'On, off or mixed, with its label.' },
  { id: 'combobox', label: 'Combobox', kind: 'Forms', line: 'Filters a long list as you type.' },
  { id: 'data-table', label: 'Data Table', kind: 'Display', line: 'Sort, select and resize columns.' },
  { id: 'date-picker', label: 'Date Picker', kind: 'Forms', line: 'A field that opens a calendar.' },
  { id: 'dialog', label: 'Dialog', kind: 'Overlays', line: 'A modal panel that holds the focus.' },
  { id: 'dropdown-menu', label: 'Dropdown Menu', kind: 'Actions', line: 'Commands and settings in a menu.' },
  { id: 'field', label: 'Field', kind: 'Forms', line: 'Label, control, help and error.' },
  { id: 'input', label: 'Input', kind: 'Forms', line: 'One line of text, in all its states.' },
  { id: 'input-group', label: 'Input Group', kind: 'Forms', line: 'An input with icons, text or buttons inside.' },
  { id: 'kbd', label: 'Kbd', kind: 'Display', line: 'A key or a shortcut: ⌘K, Ctrl+K.' },
  { id: 'label', label: 'Label', kind: 'Forms', line: 'Names a control; a press focuses it.' },
  { id: 'pagination', label: 'Pagination', kind: 'Actions', line: 'Moves through the pages of a list.' },
  { id: 'popover', label: 'Popover', kind: 'Overlays', line: 'Rich content beside its trigger.' },
  { id: 'progress', label: 'Progress', kind: 'Display', line: 'How far along a task is.' },
  { id: 'radio-group', label: 'Radio Group', kind: 'Forms', line: 'One choice of a few, all in view.' },
  { id: 'select', label: 'Select', kind: 'Forms', line: 'One choice from a list, searchable.' },
  { id: 'selection-bar', label: 'Selection Bar', kind: 'Actions', line: 'What to do with the items picked.' },
  { id: 'separator', label: 'Separator', kind: 'Display', line: 'A hairline between groups of content.' },
  { id: 'skeleton', label: 'Skeleton', kind: 'Display', line: 'The shape of content while it loads.' },
  { id: 'slider', label: 'Slider', kind: 'Forms', line: 'A value or a range along a track.' },
  { id: 'spinner', label: 'Spinner', kind: 'Display', line: 'Work under way, with no known end.' },
  { id: 'switch', label: 'Switch', kind: 'Forms', line: 'A setting that takes effect at once.' },
  { id: 'table', label: 'Table', kind: 'Display', line: 'Rows and columns the app writes.' },
  { id: 'tabs', label: 'Tabs', kind: 'Actions', line: 'Views of one page, one at a time.' },
  { id: 'textarea', label: 'Textarea', kind: 'Forms', line: 'Lines of text that grow as you type.' },
  { id: 'toast', label: 'Toast', kind: 'Overlays', line: 'A brief note, and the time it has left.' },
  { id: 'toggle', label: 'Toggle', kind: 'Actions', line: 'A button that stays on or off.' },
  { id: 'toggle-group', label: 'Toggle Group', kind: 'Actions', line: 'A set of toggles, one or several on.' },
  { id: 'tooltip', label: 'Tooltip', kind: 'Overlays', line: 'A short hint where the pointer rests.' },
];
