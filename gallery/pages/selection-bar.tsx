// The SelectionBar page: a task table whose checked rows raise the bar, the
// sizes, a dark bar with a filled action, and a narrow panel where the
// actions show their icons alone.

import { signal, Text, View } from "nsh";
import {
  Button,
  DataTable,
  type DataTableColumn,
  DropdownMenu,
  DropdownMenuItem,
  DropdownMenuSeparator,
  SelectionBar,
  SelectionBarAction,
  SelectionBarSeparator,
  type Size,
  toast,
} from "nsh-longbridge";
import { Caption, Demo, Page, Row, Section } from "../ui/layout";
import { TokensSection } from "../ui/tokens";

type Task = { id: string; title: string; status: string; assignee: string; due: string };

const allTasks: Task[] = [
  { id: "t1", title: "Draft the release notes", status: "In progress", assignee: "Ava", due: "Mon" },
  { id: "t2", title: "Review the onboarding copy", status: "To do", assignee: "Noah", due: "Tue" },
  { id: "t3", title: "Fix the export of empty reports", status: "In review", assignee: "Mia", due: "Tue" },
  { id: "t4", title: "Plan the next sprint", status: "To do", assignee: "Leo", due: "Wed" },
  { id: "t5", title: "Update the pricing page", status: "In progress", assignee: "Ava", due: "Thu" },
  { id: "t6", title: "Answer the support backlog", status: "To do", assignee: "Zoe", due: "Thu" },
  { id: "t7", title: "Profile the search index", status: "In review", assignee: "Noah", due: "Fri" },
  { id: "t8", title: "Archive last quarter's files", status: "Done", assignee: "Mia", due: "Fri" },
];

const columns: DataTableColumn[] = [
  { key: "title", title: "Task", grow: 1, minWidth: 200 },
  { key: "status", title: "Status", width: 112 },
  { key: "assignee", title: "Assignee", width: 96 },
  { key: "due", title: "Due", width: 64 },
];

const sizes: Size[] = ["sm", "md", "lg"];

export function SelectionBarPage() {
  return (
    <Page
      eyebrow="Components"
      title="Selection Bar"
      description="A bar that floats at the bottom of the window or a panel while items are selected: how many, a way to clear them, and what can be done to all of them at once."
    >
      <HeroDemo />

      <Section
        title="Sizes"
        description="The bar's size is its actions' and its count's: Button's heights, taller on touch screens."
      >
        <Demo class="items-stretch">
          {sizes.map((size) => (
            <Row label={size}>
              <View class="h-[84px] flex-1">
                <SelectionBar placement="container" size={size} count={3} onClear={() => {}}>
                  <SelectionBarAction icon="tag" label="Tags" />
                  <SelectionBarAction icon="archive" label="Archive" />
                  <SelectionBarSeparator />
                  <SelectionBarAction icon="trash-2" iconOnly variant="destructive" label="Delete" />
                </SelectionBar>
              </View>
            </Row>
          ))}
        </Demo>
      </Section>

      <Section
        title="A dark bar, its own words"
        description="appearance draws the bar, and the menus it opens, in the dark values on a light page. label says what is selected; a filled Button stands out as the main action."
      >
        <Demo class="items-stretch">
          <View class="h-[120px] w-full">
            <SelectionBar
              placement="container"
              appearance="dark"
              count={2}
              label="2 contacts selected"
              onClear={() => {}}
            >
              <DropdownMenu
                placement="top"
                align="start"
                trigger={<SelectionBarAction iconEnd="chevron-down" label="Add to group" />}
              >
                <DropdownMenuItem>
                  <Text>Customers</Text>
                </DropdownMenuItem>
                <DropdownMenuItem>
                  <Text>Partners</Text>
                </DropdownMenuItem>
              </DropdownMenu>
              <SelectionBarAction icon="archive" label="Archive" />
              <Button size="md" onPress={() => toast("2 contacts merged")}>
                <Text>Merge</Text>
              </Button>
            </SelectionBar>
          </View>
        </Demo>
      </Section>

      <Section
        title="Short of room"
        description="Step by step, each while the one before does not fit: the actions with an icon show it alone (the label in a tooltip and in what screen readers say), then the count shows its number alone (screen readers still hear the words), then the actions scroll sideways, fading out where more of them are."
      >
        <Demo class="items-center gap-6">
          <Panel width={440} caption="440 points: icons alone" />
          <Panel width={340} caption="340 points: the number alone" />
          <Panel width={260} caption="260 points: the actions scroll" />
        </Demo>
      </Section>

      <TokensSection prefix="selection-bar" component="SelectionBar" />
    </Page>
  );
}

/** A panel `width` points wide with a bar of six actions over it, and a caption under it. */
function Panel(props: { width: number; caption: string }) {
  return (
    <View class="items-center gap-2">
      <View class="h-[120px] rounded-lg border border-border bg-muted" style={{ width: props.width }}>
        <SelectionBar placement="container" count={12} onClear={() => {}}>
          <SelectionBarAction icon="tag" label="Tags" />
          <SelectionBarAction icon="user" label="Assign" />
          <SelectionBarAction icon="copy" label="Duplicate" />
          <SelectionBarAction icon="download" label="Export" />
          <SelectionBarSeparator />
          <SelectionBarAction icon="trash-2" variant="destructive" label="Delete" />
        </SelectionBar>
      </View>
      <Caption>{props.caption}</Caption>
    </View>
  );
}

function HeroDemo() {
  const tasks = signal<Task[]>(allTasks);
  const picked = signal<string[]>([]);
  const inset = signal(0);
  const count = () => picked().length;
  const noun = (n: number) => (n === 1 ? "task" : "tasks");
  const remove = (verb: string) => {
    const n = count();
    tasks.set(tasks().filter((t) => !picked().includes(t.id)));
    picked.set([]);
    toast(`${n} ${noun(n)} ${verb}`);
  };
  return (
    <Demo
      hero
      class="items-stretch p-6"
      label="Check rows to raise the bar. Escape clears the selection from the bar; in the table, the page listens for it."
    >
      <View class="h-[340px] w-full max-w-[720px] self-center">
        {/* Escape in the bar is the bar's; in the table, the page's own. */}
        <View
          class="flex-1"
          style={{ marginBottom: inset() }}
          onKeyDown={(e) => {
            if (e.key === "Escape" && picked().length > 0) picked.set([]);
          }}
        >
          <DataTable
            class="flex-1"
            accessibilityLabel="Tasks"
            columns={columns}
            data={tasks()}
            rowSelectable={false}
            checkboxColumn
            checkedRows={tasks().map((t) => picked().includes(t.id))}
            onCheckedChange={({ row, checked }) => {
              const id = tasks()[row].id;
              picked.set(checked ? [...picked(), id] : picked().filter((x) => x !== id));
            }}
            onCheckedAllChange={({ checked }) => picked.set(checked ? tasks().map((t) => t.id) : [])}
            empty={
              <View class="items-center gap-3">
                <Text class="text-sm text-muted-foreground">No tasks left.</Text>
                <Button variant="outline" size="sm" onPress={() => tasks.set(allTasks)}>
                  <Text>Bring them back</Text>
                </Button>
              </View>
            }
          />
        </View>
        <SelectionBar
          placement="container"
          count={count()}
          label={`${count()} ${noun(count())} selected`}
          onClear={() => picked.set([])}
          onInsetChange={(e) => inset.set(e.inset)}
        >
          <SelectionBarAction icon="tag" label="Tags" onPress={() => toast(`Tagging ${count()} ${noun(count())}`)} />
          <SelectionBarAction icon="user" label="Assign" onPress={() => toast(`Assigning ${count()} ${noun(count())}`)} />
          <SelectionBarAction icon="archive" label="Archive" onPress={() => remove("archived")} />
          <SelectionBarSeparator />
          <SelectionBarAction icon="trash-2" iconOnly variant="destructive" label="Delete" onPress={() => remove("deleted")} />
          <DropdownMenu
            placement="top"
            align="end"
            trigger={<SelectionBarAction icon="ellipsis" iconOnly label="More" />}
          >
            <DropdownMenuItem icon="copy" onSelect={() => toast(`${count()} ${noun(count())} duplicated`)}>
              <Text>Duplicate</Text>
            </DropdownMenuItem>
            <DropdownMenuItem icon="download" onSelect={() => toast("Exported as CSV")}>
              <Text>Export as CSV</Text>
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem variant="destructive" icon="trash-2" onSelect={() => remove("deleted")}>
              <Text>Delete</Text>
            </DropdownMenuItem>
          </DropdownMenu>
        </SelectionBar>
      </View>
    </Demo>
  );
}
