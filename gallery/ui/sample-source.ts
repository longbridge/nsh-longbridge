// The home page's sample as text (no NSH): `InviteCard` (`sample.tsx`) as the
// site shows its source, imports left out. The two must stay the same code.

export const INVITE_FILE = "invite-card.tsx";

export const INVITE_SOURCE = `
export function InviteCard() {
  const email = signal("");
  return (
    <Card>
      <CardHeader>
        <CardTitle><Text>Invite a teammate</Text></CardTitle>
        <CardDescription><Text>They join every project.</Text></CardDescription>
      </CardHeader>
      <CardContent class="gap-2">
        <Label><Text>Email</Text></Label>
        <Input placeholder="ada@northwind.dev" value={email()}
          onChange={(e) => email.set(e.value)} />
      </CardContent>
      <CardFooter class="justify-end gap-2">
        <Button variant="outline"><Text>Cancel</Text></Button>
        <Button disabled={!email().includes("@")}><Text>Send invite</Text></Button>
      </CardFooter>
    </Card>
  );
}
`;
