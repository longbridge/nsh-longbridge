// The Avatar page: a team, the fallback, badges, groups, sizes.

import { Text, View } from "nsh";
import { Avatar, AvatarBadge, AvatarGroup, AvatarGroupCount, Badge, Icon, type Size } from "nsh-longbridge";
import { Demo, Page, Row, Section } from "../ui/layout";
import { Glyph } from "../ui/icons";
import { TokensSection } from "../ui/tokens";
import { User } from "nsh-icon-lucide";

const sizes: Size[] = ["xs", "sm", "md", "lg"];

/** A portrait drawn as an SVG: a backdrop, a head and shoulders. */
function portrait(backdrop: string, skin: string, shirt: string): string {
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64"><rect width="64" height="64" fill="${backdrop}"/><circle cx="32" cy="26" r="12" fill="${skin}"/><path d="M10 64c2-14 11-21 22-21s20 7 22 21z" fill="${shirt}"/></svg>`;
  return `data:image/svg+xml;utf8,${encodeURIComponent(svg)}`;
}

const team = [
  { name: "Sofia Davis", role: "Design lead", photo: portrait("#c7d2fe", "#f2c9a0", "#4f46e5"), online: true },
  { name: "Jackson Lee", role: "Engineer", photo: portrait("#bbf7d0", "#d9a27a", "#15803d"), online: true },
  { name: "Isabella Nguyen", role: "Product manager", photo: undefined, online: false },
  { name: "William Kim", role: "Engineer", photo: portrait("#fde68a", "#8d5a3b", "#b45309"), online: false },
  { name: "Olivia Martin", role: "Support", photo: "https://invalid.example/olivia.png", online: true },
];

export function AvatarPage() {
  return (
    <Page
      eyebrow="Components"
      title="Avatar"
      description="A person's picture in a circle, or their initials while it loads, when it fails and when there is none; a badge at its corner; a group that overlaps and counts the rest."
    >
      <HeroDemo />

      <Section
        title="Initials"
        description="Without a picture, or while it loads, or when it fails, the initials of the name: the first letters of the first two words."
      >
        <Demo row class="justify-center gap-4">
          <Avatar name="Ada Lovelace" size="lg" />
          <Avatar name="Grace Hopper" size="lg" />
          <Avatar name="王小明" size="lg" />
          <Avatar name="Olivia Martin" src="https://invalid.example/olivia.png" size="lg" />
          <Avatar name="Support" size="lg" fallback={<Glyph icon={User} size={20} />} />
        </Demo>
      </Section>

      <Section title="Badges" description="A dot at the bottom end, ringed in the page's color; an icon in it from md up.">
        <Demo row class="justify-center gap-6">
          <Avatar name="Sofia Davis" src={team[0].photo} size="lg">
            <AvatarBadge class="bg-green-600" />
          </Avatar>
          <Avatar name="Jackson Lee" src={team[1].photo} size="lg">
            <AvatarBadge>
              <Icon name="check" />
            </AvatarBadge>
          </Avatar>
          <Avatar name="William Kim" src={team[3].photo} size="md">
            <AvatarBadge class="bg-muted-foreground" />
          </Avatar>
        </Demo>
      </Section>

      <Section
        title="Groups"
        description="The first few, each over the one before with a ring in the page's color, then a count of the rest."
      >
        <Demo class="items-center gap-6">
          {sizes.map((size) => (
            <Row label={size}>
              <AvatarGroup
                size={size}
                limit={3}
                total={team.length}
                accessibilityLabel="Team"
                overflow={
                  <AvatarGroupCount>
                    <Text>{`+${team.length - 3}`}</Text>
                  </AvatarGroupCount>
                }
              >
                {team.map((m) => (
                  <Avatar name={m.name} src={m.photo} />
                ))}
              </AvatarGroup>
            </Row>
          ))}
        </Demo>
      </Section>

      <Section
        title="Sizes"
        description="20, 24, 32 and 40 points, the same on touch screens: an avatar is no press target. Any other side is a class."
      >
        <Demo row class="items-end justify-center gap-4">
          {sizes.map((size) => (
            <Avatar name="Sofia Davis" src={team[0].photo} size={size} />
          ))}
          <Avatar name="Sofia Davis" src={team[0].photo} class="size-14" />
          {sizes.map((size) => (
            <Avatar name="Isabella Nguyen" size={size} />
          ))}
        </Demo>
      </Section>

      <TokensSection prefix="avatar" component="Avatar" />
    </Page>
  );
}

function HeroDemo() {
  return (
    <Demo hero>
      <View class="w-full max-w-[420px] gap-5">
        <View class="flex-row items-center justify-between">
          <Text class="text-sm font-medium text-foreground">Team members</Text>
          <AvatarGroup
            limit={3}
            total={team.length}
            accessibilityLabel="Team"
            overflow={
              <AvatarGroupCount>
                <Text>{`+${team.length - 3}`}</Text>
              </AvatarGroupCount>
            }
          >
            {team.map((m) => (
              <Avatar name={m.name} src={m.photo} />
            ))}
          </AvatarGroup>
        </View>
        {team.slice(0, 4).map((m) => (
          <View class="flex-row items-center gap-3">
            <Avatar name={m.name} src={m.photo} size="lg" accessibilityLabel={`${m.name}, ${m.online ? "online" : "away"}`}>
              <AvatarBadge class={m.online ? "bg-green-600" : "bg-muted-foreground"} />
            </Avatar>
            <View class="flex-1 gap-0.5">
              <Text class="text-sm font-medium text-foreground">{m.name}</Text>
              <Text class="text-xs text-muted-foreground">{m.role}</Text>
            </View>
            <Badge variant={m.online ? "secondary" : "outline"}>
              <Text>{m.online ? "Online" : "Away"}</Text>
            </Badge>
          </View>
        ))}
      </View>
    </Demo>
  );
}
