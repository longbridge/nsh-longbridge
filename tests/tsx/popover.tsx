// `Popover` from TSX with a library `Button` as its trigger (which takes
// the press itself, before the trigger slot's view): the press opens and
// closes it, a button in it is not its trigger, and a plain view still is.
import { mount, Text, View } from "nsh";
import { Button, Popover } from "nsh-longbridge";

function App() {
  return (
    <View style={{ paddingTop: 40, paddingLeft: 40, gap: 20, alignItems: "flex-start" }}>
      <Popover trigger={<Button variant="outline"><Text>Share</Text></Button>}>
        <Text>Anyone with the link</Text>
        <Button testID="copy"><Text>Copy</Text></Button>
      </Popover>
      <Popover defaultOpen placement="right" trigger={<Button variant="outline"><Text>Held</Text></Button>}>
        <Text>Open at first</Text>
      </Popover>
      <Popover placement="right" trigger={<View testID="plain" style={{ width: 40, height: 20 }} />}>
        <Text>From a view</Text>
      </Popover>
      <View testID="away" style={{ width: 40, height: 40 }} />
    </View>
  );
}

mount(App);
