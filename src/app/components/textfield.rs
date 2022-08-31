use turbocharger::prelude::*;

#[frontend]
#[derive(Props)]
pub struct TextFieldProps<'a> {
 value: &'a UseState<String>,
}

#[frontend]
pub fn TextField<'a>(cx: Scope<'a, TextFieldProps<'a>>) -> Element<'a> {
 let value = cx.props.value;

 rsx! {cx,
  input { class: "font-mono text-gray-600",
   "type": "text",
   value: "{value}",
   oninput: move |evt| value.set(evt.value.clone()),
  }
 }
}
