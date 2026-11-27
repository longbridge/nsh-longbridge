// The home page: the name, one sentence, and screens of a product made only
// of the library, which follow the theme controls in the header.

import { Text, View } from "nsh";
import { Badge, Button } from "nsh-longbridge";
import { componentPages } from "../catalog";
import { navigate } from "../router";
import { contentWidth } from "../ui/screen";
import { Showcase } from "../ui/showcase";

export const showcaseColumns = (width: number) => (width >= 980 ? 3 : width >= 620 ? 2 : 1);

export function HomePage() {
  return (
    <View class="gap-14">
      <View class="items-start gap-5 pt-4">
        <Badge variant="outline">
          <Text>{`${componentPages.length} components · light and dark`}</Text>
        </Badge>
        <Text class="text-4xl font-semibold tracking-tight text-foreground sm:text-5xl">nsh-longbridge</Text>
        <Text class="max-w-[600px] text-lg leading-7 text-muted-foreground">
          Components for NSH apps with the look of shadcn/ui, one Rust implementation for TSX and Rust, restyled from a
          few seed tokens down to a single part.
        </Text>
        <View class="flex-row flex-wrap gap-2 pt-1">
          <Button iconEnd="arrow-right" onPress={() => navigate(componentPages[0].id)}>
            <Text>Browse components</Text>
          </Button>
          <Button variant="outline" onPress={() => navigate("customize")}>
            <Text>Customize the theme</Text>
          </Button>
        </View>
      </View>
      <View class="gap-4">
        <Text class="text-sm text-muted-foreground">
          Every screen below is made of the library's components alone. Change the preset, the density or the appearance
          in the header, and all of it follows.
        </Text>
        <Showcase columns={showcaseColumns(contentWidth(true))} />
      </View>
    </View>
  );
}
