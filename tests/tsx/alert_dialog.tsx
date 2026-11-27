// `AlertDialog` from TSX with a library `Button` as its trigger: the press
// opens it, and its Cancel and action close it (neither is the trigger).
import { mount, Text, View } from "nsh";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  Button,
} from "nsh-longbridge";

function App() {
  return (
    <View style={{ paddingTop: 40, paddingLeft: 40, alignItems: "flex-start" }}>
      <AlertDialog trigger={<Button variant="destructive"><Text>Delete</Text></Button>}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>
              <Text>Delete 3 files?</Text>
            </AlertDialogTitle>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>
              <Text>Cancel</Text>
            </AlertDialogCancel>
            <AlertDialogAction>
              <Text>Remove</Text>
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </View>
  );
}

mount(App);
