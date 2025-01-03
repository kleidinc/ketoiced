# Mission : Implement a Form with Iced, Postgres and SQLX

The text_input (iced version 0.13), as the name implies, can only handle text. But, for many fields we need
to process numbers. All functionality (moving from one input field to the other, checks on input,...) we take
for granted in JS web-development, has to be coded in Rust and iced.

But the added benefits of robust code, full control, and the ability to create desktop apps, is more than worth it.

## For input fields (WIP)

1. Parse the String into a f32/i32 with a feedback loop.
2. Don't allow to save the form if any of the input_fields of the form contain erroneous types.
3. When there is an erroneous type in an input_field show a hint under the text_input.
4. Add tab-navigation. Activate the tab-key to go from one text_input to another. And shift-tab
   to go back-wards.
5. Add C-S short-cut to save a form.

**_ The `number_input` in the iced_aw crate is erroneous, and also has the limitation that you
need to use the mouse to change the value. Therefore it was discarded. _**
