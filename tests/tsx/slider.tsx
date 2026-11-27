// `Slider` from TSX: the bubble's text from the app's `formatValue`, a JS
// function the library asks ahead of the value reaching it.
import { mount, signal, View } from "nsh";
import { Slider } from "nsh-longbridge";

function App() {
  const price = signal(250);
  return (
    <View style={{ padding: 60, width: 320 }}>
      <Slider
        min={0}
        max={1000}
        step={10}
        bubble="always"
        formatValue={(v) => `$${v}`}
        marks={[0, { value: 500, label: "500" }, 1000]}
        histogram={[1, 3, 2]}
        value={price()}
        onChange={({ value }) => price.set(value as number)}
        accessibilityLabel="Price"
      />
    </View>
  );
}

mount(App);
