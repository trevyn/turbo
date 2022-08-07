use turbocharger::prelude::*;

#[frontend]
#[derive(Props)]
pub struct ActionButtonProps<
 'a,
 S: Stream<Item = Result<T, E>> + 'static,
 F: Fn() -> S + 'static,
 T: ToString,
 E: ToString,
> {
 action: F,
 children: Element<'a>,
}

#[frontend]
pub fn ActionButton<'a, S, F, T, E>(cx: Scope<'a, ActionButtonProps<'a, S, F, T, E>>) -> Element<'a>
where
 S: Stream<Item = Result<T, E>> + 'static,
 F: Fn() -> S + 'static,
 T: ToString,
 E: ToString,
{
 let state = use_state(&cx, String::new);

 rsx! {cx,
  p { class: "p-4", super::button::Button { onclick: move |_| {
   let s = (cx.props.action)();
   to_owned![state];
   cx.spawn(async move {
    pin_mut!(s);
    while let Some(r) = s.next().await {
     state.set(match r {
      Ok(r) => r.to_string(),
      Err(e) => e.to_string(),
     });
    };
   });
  }, &cx.props.children }, " {state}" }
 }
}
