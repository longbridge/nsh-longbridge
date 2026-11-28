// `DropdownMenu` from TSX with a submenu: its trigger row is lit while
// focused and while the submenu is open.
import { mount, Text, View } from "nsh";
import {
  Button,
  DropdownMenu,
  DropdownMenuItem,
  DropdownMenuSub,
  DropdownMenuSubTrigger,
} from "nsh-longbridge";

function App() {
  return (
    <View style={{ paddingTop: 40, paddingLeft: 40, alignItems: "flex-start" }}>
      <DropdownMenu trigger={<Button variant="outline"><Text>Open</Text></Button>}>
        <DropdownMenuItem>
          <Text>Profile</Text>
        </DropdownMenuItem>
        <DropdownMenuSub trigger={<DropdownMenuSubTrigger><Text>Invite users</Text></DropdownMenuSubTrigger>}>
          <DropdownMenuItem>
            <Text>Email</Text>
          </DropdownMenuItem>
        </DropdownMenuSub>
      </DropdownMenu>
    </View>
  );
}

mount(App);
