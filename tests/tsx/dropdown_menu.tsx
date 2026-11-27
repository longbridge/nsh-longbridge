// `DropdownMenu` from TSX with a library `Button` as its trigger: the
// press opens and closes it, and an item chosen closes it.
import { mount, Text, View } from "nsh";
import { Button, DropdownMenu, DropdownMenuItem } from "nsh-longbridge";

function App() {
  return (
    <View style={{ paddingTop: 40, paddingLeft: 40, alignItems: "flex-start" }}>
      <DropdownMenu trigger={<Button variant="outline"><Text>Account</Text></Button>}>
        <DropdownMenuItem>
          <Text>Profile</Text>
        </DropdownMenuItem>
        <DropdownMenuItem>
          <Text>Settings</Text>
        </DropdownMenuItem>
      </DropdownMenu>
    </View>
  );
}

mount(App);
