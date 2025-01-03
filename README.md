# Mission : Implement a Form with Iced and Postgres

The text_input can only handle text, as the name implies. But for many fields we need
to get a number. We need build a feature-set, which :

1. Parse the String into a f32/i32 with a feedback loop.
2. Don't allow to save the form if the input_fields contain erroneous types.
3. When there is an erroneous type in an input_field show a hint under the text_input.
4. Add tab-navigation. Activate the tab-key to go from one text_input to another. And shift-tab
   to go back-wards.
5. On Enter inside a field, move to the next text_input.
