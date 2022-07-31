use turbocharger::prelude::*;

#[frontend]
use super::button::Button;

#[frontend]
#[derive(Props)]
pub struct ActionButtonProps<
 'a,
 S: Stream<Item = Result<String, tracked::StringError>> + 'static,
 T: Fn() -> S + 'static,
> {
 stream: T,
 children: Element<'a>,
}

#[frontend]
pub fn ActionButton<'a, S, T>(cx: Scope<'a, ActionButtonProps<'a, S, T>>) -> Element<'a>
where
 S: Stream<Item = Result<String, tracked::StringError>> + 'static,
 T: Fn() -> S + 'static,
{
 let state = use_state(&cx, String::new);

 rsx! {cx,
  p { class: "p-4", Button { onclick: move |_| {
   let s = (cx.props.stream)();
   to_owned![state];
   cx.spawn(async move {
    pin_mut!(s);
    while let Some(r) = s.next().await {
     state.set(match r {
      Ok(r) => r,
      Err(e) => e.to_string(),
     });
    };
   });
  }, &cx.props.children }, " {state}" }
 }
}
