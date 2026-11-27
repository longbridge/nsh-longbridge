// `Table` from TSX: a `DropdownMenu` in a cell lights its row while open;
// a column's width classes and `items-end` win over the cell's own.
import { mount, Text, View } from "nsh";
import {
  Button,
  DropdownMenu,
  DropdownMenuItem,
  Table,
  TableBody,
  TableCell,
  TableRow,
} from "nsh-longbridge";

function App() {
  return (
    <View style={{ width: 500 }}>
      <Table accessibilityLabel="Invoices">
        <TableBody>
          <TableRow testID="row">
            <TableCell testID="fixed" class="w-[100px] flex-none">
              <Text>INV001</Text>
            </TableCell>
            <TableCell testID="amount" class="items-end">
              <Text>$250.00</Text>
            </TableCell>
            <TableCell>
              <DropdownMenu trigger={<Button variant="ghost"><Text>Actions</Text></Button>}>
                <DropdownMenuItem>
                  <Text>Copy</Text>
                </DropdownMenuItem>
              </DropdownMenu>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
      <View testID="away" style={{ width: 200, height: 40 }} />
    </View>
  );
}

mount(App);
