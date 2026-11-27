// The DropdownMenu page: an account menu in an app's header, a view menu of
// checkbox and radio items, a file's overflow menu, sizes and sides.

import { keymap, signal, Text, View } from "nsh";
import {
  Badge,
  Button,
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuSeparator,
  DropdownMenuShortcut,
  DropdownMenuSub,
  DropdownMenuSubTrigger,
  type Placement,
  type Size,
} from "nsh-longbridge";
import { Demo, Demos, Page, Row, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

const sizes: Size[] = ["xs", "sm", "md", "lg"];
const sides: Placement[] = ["bottom", "top", "right", "left"];

const files = [
  { name: "Q3 roadmap.pdf", note: "2.4 MB · Edited 2 hours ago" },
  { name: "Brand guidelines.fig", note: "18 MB · Edited yesterday" },
  { name: "Customer interviews.docx", note: "640 KB · Edited last week" },
];
const folders = ["Design", "Planning", "Archive"];

export function DropdownMenuPage() {
  return (
    <Page
      eyebrow="Components"
      title="Dropdown Menu"
      description="Commands and settings a button opens: items with icons and shortcuts, checkbox and radio items, labels, separators and submenus. A popup at the trigger with a pointer, a sheet at the bottom of a phone."
    >
      <Demo hero class="items-stretch justify-start">
        <AppHeader />
      </Demo>

      <Section
        title="Checkbox and radio items"
        description="A checkbox item asks for the other value, a radio item for its own; the app sets them, and the menu stays open while it does."
      >
        <ViewMenuDemo />
      </Section>

      <Section
        title="An overflow menu"
        description="An icon-only button opens a row's actions. A submenu opens beside its item (a page of the sheet on a phone); a destructive item turns red when highlighted."
      >
        <Demo class="items-stretch">
          <View class="w-full max-w-[560px] self-center">
            {files.map((f, i) => (
              <FileRow name={f.name} note={f.note} last={i === files.length - 1} />
            ))}
          </View>
        </Demo>
      </Section>

      <Section
        title="Sizes"
        description="size is set on the menu: its rows' height and text, as a Button of that size. Touch screens get 44 point rows whatever it is."
      >
        <Demo row>
          {sizes.map((size) => (
            <DropdownMenu
              size={size}
              trigger={
                <Button variant="outline" size={size} iconEnd="chevron-down">
                  <Text>{size}</Text>
                </Button>
              }
            >
              <DropdownMenuItem icon="settings">
                <Text>Settings</Text>
              </DropdownMenuItem>
              <DropdownMenuItem icon="activity">
                <Text>Activity</Text>
              </DropdownMenuItem>
              <DropdownMenuSeparator />
              <DropdownMenuItem icon="log-out">
                <Text>Log out</Text>
              </DropdownMenuItem>
            </DropdownMenu>
          ))}
        </Demo>
      </Section>

      <Section
        title="Sides"
        description="placement and align put the menu on a side of its trigger; it goes to the other side when only that one has room, and slides in from it."
      >
        <Demo row class="gap-6 py-16">
          {sides.map((side) => (
            <DropdownMenu
              placement={side}
              align="start"
              trigger={
                <Button variant="secondary">
                  <Text>{side[0].toUpperCase() + side.slice(1)}</Text>
                </Button>
              }
            >
              <DropdownMenuItem>
                <Text>Copy link</Text>
              </DropdownMenuItem>
              <DropdownMenuItem>
                <Text>Share</Text>
              </DropdownMenuItem>
            </DropdownMenu>
          ))}
        </Demo>
      </Section>

      <TokensSection
        prefix="menu"
        component="DropdownMenu"
        note="Select's list shares these tokens, so a change here reaches it too. The popup's fill, edge, corners and shadow are the popover tokens."
      />
    </Page>
  );
}

/**
 * An app's header: the workspace, its navigation and the account menu at the
 * end, held open as the page opens (it closes as any menu does) in a frame
 * tall enough for it.
 */
function AppHeader() {
  const last = signal("");
  const open = signal(true);
  return (
    <View class="w-full min-h-[400px] justify-between">
      <View class="flex-row items-center gap-4 rounded-xl border border-border bg-background px-4 py-3">
        <View class="flex-row items-center gap-2">
          <View class="size-7 items-center justify-center rounded-md bg-primary">
            <Text class="text-xs font-semibold text-primary-foreground">A</Text>
          </View>
          <Text class="text-sm font-semibold text-foreground">Acme Inc</Text>
        </View>
        <View class="flex-1 flex-row items-center gap-1">
          <Button variant="ghost" size="sm">
            <Text>Overview</Text>
          </Button>
          <Button variant="ghost" size="sm">
            <Text>Deployments</Text>
          </Button>
        </View>
        <AccountMenu open={open()} onOpenChange={(o) => open.set(o)} onSelect={(what) => last.set(what)} />
      </View>
      <Text class="text-xs text-muted-foreground">{last() === "" ? "Choose an item: the menu reports it and closes." : `Chose: ${last()}`}</Text>
    </View>
  );
}

function AccountMenu(props: { open: boolean; onOpenChange: (open: boolean) => void; onSelect: (what: string) => void }) {
  return (
    <DropdownMenu
      open={props.open}
      onOpenChange={(e) => props.onOpenChange(e.open)}
      align="end"
      contentClass="w-56"
      trigger={
        <Button variant="outline" size="sm" iconEnd="chevron-down">
          <Text>Sofia Davis</Text>
        </Button>
      }
    >
      <DropdownMenuLabel>
        <Text>My Account</Text>
      </DropdownMenuLabel>
      <DropdownMenuGroup>
        <DropdownMenuItem onSelect={() => props.onSelect("Profile")}>
          <Text>Profile</Text>
          <DropdownMenuShortcut>
            <Text>{keymap.label("shift+mod+p")}</Text>
          </DropdownMenuShortcut>
        </DropdownMenuItem>
        <DropdownMenuItem onSelect={() => props.onSelect("Billing")}>
          <Text>Billing</Text>
          <DropdownMenuShortcut>
            <Text>{keymap.label("mod+b")}</Text>
          </DropdownMenuShortcut>
        </DropdownMenuItem>
        <DropdownMenuItem icon="settings" onSelect={() => props.onSelect("Settings")}>
          <Text>Settings</Text>
          <DropdownMenuShortcut>
            <Text>{keymap.label("mod+,")}</Text>
          </DropdownMenuShortcut>
        </DropdownMenuItem>
      </DropdownMenuGroup>
      <DropdownMenuSeparator />
      <DropdownMenuGroup>
        <DropdownMenuItem onSelect={() => props.onSelect("Team")}>
          <Text>Team</Text>
        </DropdownMenuItem>
        <DropdownMenuSub
          trigger={
            <DropdownMenuSubTrigger>
              <Text>Invite users</Text>
            </DropdownMenuSubTrigger>
          }
        >
          <DropdownMenuItem onSelect={() => props.onSelect("Invite by email")}>
            <Text>Email</Text>
          </DropdownMenuItem>
          <DropdownMenuItem onSelect={() => props.onSelect("Invite by message")}>
            <Text>Message</Text>
          </DropdownMenuItem>
          <DropdownMenuSeparator />
          <DropdownMenuItem onSelect={() => props.onSelect("Invite link")}>
            <Text>Copy invite link</Text>
          </DropdownMenuItem>
        </DropdownMenuSub>
        <DropdownMenuItem onSelect={() => props.onSelect("New team")}>
          <Text>New Team</Text>
          <DropdownMenuShortcut>
            <Text>{keymap.label("mod+t")}</Text>
          </DropdownMenuShortcut>
        </DropdownMenuItem>
      </DropdownMenuGroup>
      <DropdownMenuSeparator />
      <DropdownMenuItem onSelect={() => props.onSelect("Support")}>
        <Text>Support</Text>
      </DropdownMenuItem>
      <DropdownMenuItem disabled>
        <Text>API</Text>
      </DropdownMenuItem>
      <DropdownMenuSeparator />
      <DropdownMenuItem icon="log-out" onSelect={() => props.onSelect("Log out")}>
        <Text>Log out</Text>
        <DropdownMenuShortcut>
          <Text>{keymap.label("shift+mod+q")}</Text>
        </DropdownMenuShortcut>
      </DropdownMenuItem>
    </DropdownMenu>
  );
}

/** An editor's View menu: panels shown (checkbox items) and the panel's side (radio items). */
function ViewMenuDemo() {
  const statusBar = signal(true);
  const activityBar = signal(false);
  const minimap = signal(true);
  const position = signal("bottom");
  const shown = () =>
    [statusBar() && "status bar", activityBar() && "activity bar", minimap() && "minimap"].filter((s) => s).join(", ") ||
    "nothing";
  return (
    <Demo row label={`Shown: ${shown()} · panel at the ${position()}`}>
      <DropdownMenu
        contentClass="w-56"
        trigger={
          <Button variant="outline" iconEnd="chevron-down">
            <Text>View</Text>
          </Button>
        }
      >
        <DropdownMenuLabel>
          <Text>Appearance</Text>
        </DropdownMenuLabel>
        <DropdownMenuCheckboxItem checked={statusBar()} onChange={(e) => statusBar.set(e.checked)}>
          <Text>Status Bar</Text>
        </DropdownMenuCheckboxItem>
        <DropdownMenuCheckboxItem checked={activityBar()} onChange={(e) => activityBar.set(e.checked)}>
          <Text>Activity Bar</Text>
        </DropdownMenuCheckboxItem>
        <DropdownMenuCheckboxItem checked={minimap()} onChange={(e) => minimap.set(e.checked)}>
          <Text>Minimap</Text>
        </DropdownMenuCheckboxItem>
        <DropdownMenuSeparator />
        <DropdownMenuLabel>
          <Text>Panel Position</Text>
        </DropdownMenuLabel>
        <DropdownMenuRadioGroup value={position()} onChange={(e) => position.set(e.value)}>
          <DropdownMenuRadioItem value="top">
            <Text>Top</Text>
          </DropdownMenuRadioItem>
          <DropdownMenuRadioItem value="bottom">
            <Text>Bottom</Text>
          </DropdownMenuRadioItem>
          <DropdownMenuRadioItem value="right">
            <Text>Right</Text>
          </DropdownMenuRadioItem>
        </DropdownMenuRadioGroup>
      </DropdownMenu>
    </Demo>
  );
}

/** A file in a list, its actions in an overflow menu. */
function FileRow(props: { name: string; note: string; last: boolean }) {
  const moved = signal<string | null>(null);
  return (
    <View class={`flex-row items-center gap-3 py-3 ${props.last ? "" : "border-b border-border"}`}>
      <View class="flex-1 gap-0.5">
        <Text class="truncate text-sm font-medium text-foreground">{props.name}</Text>
        <Text class="text-xs text-muted-foreground">{props.note}</Text>
      </View>
      <If when={moved()}>
        <Badge variant="secondary">
          <Text>{moved()!}</Text>
        </Badge>
      </If>
      <DropdownMenu
        align="end"
        trigger={<Button variant="ghost" size="sm" iconOnly icon="ellipsis" accessibilityLabel={`Actions for ${props.name}`} />}
      >
        <DropdownMenuItem>
          <Text>Rename</Text>
        </DropdownMenuItem>
        <DropdownMenuItem>
          <Text>Duplicate</Text>
          <DropdownMenuShortcut>
            <Text>{keymap.label("mod+d")}</Text>
          </DropdownMenuShortcut>
        </DropdownMenuItem>
        <DropdownMenuSub
          trigger={
            <DropdownMenuSubTrigger icon="arrow-right">
              <Text>Move to</Text>
            </DropdownMenuSubTrigger>
          }
        >
          {folders.map((folder) => (
            <DropdownMenuItem onSelect={() => moved.set(folder)}>
              <Text>{folder}</Text>
            </DropdownMenuItem>
          ))}
        </DropdownMenuSub>
        <DropdownMenuSeparator />
        <DropdownMenuItem variant="destructive" icon="trash-2">
          <Text>Delete</Text>
        </DropdownMenuItem>
      </DropdownMenu>
    </View>
  );
}
