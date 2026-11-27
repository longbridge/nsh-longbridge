// `Field` from TSX: a `FieldError` whose child is an expression that is
// `null` while the name is valid makes nothing, and shows the message once
// it is not; the field's label names its input without a ref.
import { mount, signal, Text, View } from "nsh";
import { Button, Field, FieldDescription, FieldError, FieldLabel, Input } from "nsh-longbridge";

const name = signal("Ada");
const error = () => (name().trim() === "" ? "Enter your name." : null);

function App() {
  return (
    <View style={{ padding: 20, width: 320 }}>
      <Field invalid={error() !== null}>
        <FieldLabel>
          <Text>Name</Text>
        </FieldLabel>
        <Input value={name()} onChange={({ value }) => name.set(value)} accessibilityLabel="Name box" />
        <FieldDescription>
          <Text>Your full name.</Text>
        </FieldDescription>
        <FieldError>{error() === null ? null : <Text>{error()}</Text>}</FieldError>
      </Field>
      <Button onPress={() => name.set("")}>
        <Text>Clear</Text>
      </Button>
    </View>
  );
}

mount(App);
