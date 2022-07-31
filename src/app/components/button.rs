use turbocharger::prelude::*;

#[frontend]
#[derive(Props)]
pub struct ButtonProps<'a> {
 onclick: EventHandler<'a, MouseEvent>,
 children: Element<'a>,
}

#[frontend]
pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element<'a> {
 // https://tailwindui.com/components/application-ui/elements/buttons#component-109c4104d58d9fedfa8650dbe24c1ae8
 rsx! {cx,
  button {
   "type": "button",
   class: "inline-flex items-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
   onclick: move |evt| cx.props.onclick.call(evt),
   &cx.props.children
  }
 }
}
