# ListBox and ListModel example

This example demonstrates how to use `gtk::ListBox` in combination with
`gio::ListStore` as a model with a custom row type.

It sets up a `gtk::ListBox` containing, per row, a label, spinbutton and
an edit button. The edit button allows to edit the underlying data structure
and changes are taking place immediately in the listbox by making use of GObject
property bindings.

In addition, it is possible to add new rows and delete old ones.

Run it by executing:

```bash
cargo run --bin list_box_model --features="gtk/v3_16"
```

![screenshot](screenshot.png)