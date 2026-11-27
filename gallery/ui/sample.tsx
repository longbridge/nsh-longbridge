// The home page's sample: a component written with the library, and its
// source as the page shows it beside the running one (its imports left out:
// `signal` and `Text` from nsh, the rest from nsh-longbridge). The two below
// must stay the same code: `INVITE_SOURCE` is `InviteCard` as written here.

import { signal, Text } from "nsh";
import {
  Button,
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
  Input,
  Label,
} from "nsh-longbridge";

export { INVITE_FILE, INVITE_SOURCE } from "./sample-source";

export function InviteCard(props: { class?: string } = {}) {
  const email = signal("");
  return (
    <Card class={props.class}>
      <CardHeader>
        <CardTitle>
          <Text>Invite a teammate</Text>
        </CardTitle>
        <CardDescription>
          <Text>They join every project.</Text>
        </CardDescription>
      </CardHeader>
      <CardContent class="gap-2">
        <Label>
          <Text>Email</Text>
        </Label>
        <Input
          placeholder="ada@northwind.dev"
          accessibilityLabel="Email"
          value={email()}
          onChange={(e) => email.set(e.value)}
        />
      </CardContent>
      <CardFooter class="justify-end gap-2">
        <Button variant="outline">
          <Text>Cancel</Text>
        </Button>
        <Button disabled={!email().includes("@")}>
          <Text>Send invite</Text>
        </Button>
      </CardFooter>
    </Card>
  );
}
