// `Tooltip` from TSX: given as an element's `tooltip`, the library's
// component must be taken as the hint layer itself (its delay, placement
// and look), and a `Kbd` among its children must see it is in a tooltip.
import { mount, Text, View } from "nsh";
import { Button, Kbd, Tooltip } from "nsh-longbridge";

function App() {
  return (
    <View style={{ paddingTop: 100, paddingLeft: 40 }}>
      <View
        testID="trigger"
        style={{ width: 80, height: 30 }}
        tooltip={
          <Tooltip testID="bubble" delay={300}>
            <Text>Tip</Text>
            <Kbd testID="kbd">
              <Text>K</Text>
            </Kbd>
          </Tooltip>
        }
      >
        <Text>Trigger</Text>
      </View>
      <View testID="plain" style={{ width: 80, height: 30, marginTop: 40 }} tooltip={<Tooltip><Text>Plain</Text></Tooltip>}>
        <Text>Default delay</Text>
      </View>
      <View testID="away" style={{ height: 200 }}>
        <Text>Away</Text>
      </View>
      <Button testID="button" disabled tooltip={<Tooltip delay={0}><Text>On a button</Text></Tooltip>}>
        <Text>Off</Text>
      </Button>
      <Kbd testID="outside">
        <Text>Esc</Text>
      </Kbd>
    </View>
  );
}

mount(App);
