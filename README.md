# Mission : Implement a Form with Iced, Postgres and SQLX

## Checking the type of input fields

The text_input (iced version 0.13), as the name implies, can only handle text. But, for many fields we need
to process numbers.

- Parse the String into a f32/i32 with a feedback loop.
- Don't allow to save the form if any of the input_fields of the form contain erroneous types.
- When there is an erroneous type in an input_field show a hint under the text_input.

**_ The `number_input` in the iced_aw crate is 'erroneous', and also has the limitation that you
need to use the mouse to change the value. Therefore it was 'temporarily' discarded. _**

## Converting String to f32 to BigDecimal and back

The text_input fields can only take String, so we need to parse them to f32. To write to Postgres we have to
convert them to Numeric, which in Rust (for SQLX types) is BigDecimal. There is no direct correct conversion
from f32 to BigDecimal, so we use an intermediate conversion to str and then to BigDecimal.

Created helper functions to parse from String to f32, and from f32 to BigDecimal.

## TODO: Connecting to Postgres

For now I start the connection in each write/read/... function. Although this works it is better to create a
pool once at the start and then share it amongst all the methods. Need to put it in the new function of Iced in a
Task::batch command.

## TODO: Navigating using tabs and keymappings for shortcuts

- Add tab-navigation. Activate the tab-key to go from one text_input to another. And shift-tab
  to go back-wards.
- Add C-S short-cut to save a form.
