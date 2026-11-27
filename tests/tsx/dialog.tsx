// `Dialog` from TSX with a library `Button` as its trigger: the press
// opens it, and its `DialogClose` closes it (it is not the trigger).
import { mount, Text, View } from "nsh";
import { Button, Dialog, DialogClose, DialogContent, DialogDescription, DialogTitle } from "nsh-longbridge";

function App() {
  return (
    <View style={{ paddingTop: 40, paddingLeft: 40, alignItems: "flex-start" }}>
      <Dialog trigger={<Button variant="outline"><Text>Edit profile</Text></Button>}>
        <DialogContent>
          <DialogTitle>
            <Text>Profile</Text>
          </DialogTitle>
          <DialogDescription>
            <Text>Make changes to your profile here.</Text>
          </DialogDescription>
          <DialogClose>
            <Text>Cancel</Text>
          </DialogClose>
        </DialogContent>
      </Dialog>
    </View>
  );
}

mount(App);
