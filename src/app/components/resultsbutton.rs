use turbocharger::prelude::*;

#[frontend]
#[derive(Props)]
pub struct ResultsButtonProps<
 'a,
 S: Stream<Item = Result<(T, Option<R>), E>> + 'static,
 F: Fn() -> S + 'static,
 T: ToString,
 E: ToString,
 R: 'static,
> {
 action: F,
 results: &'a UseState<Option<R>>,
 children: Element<'a>,
}

#[frontend]
pub fn ResultsButton<'a, S, F, T, E, R>(
 cx: Scope<'a, ResultsButtonProps<'a, S, F, T, E, R>>,
) -> Element<'a>
where
 S: Stream<Item = Result<(T, Option<R>), E>> + 'static,
 F: Fn() -> S + 'static,
 T: ToString,
 E: ToString,
{
 let state = use_state(&cx, String::new);

 rsx! {cx,
  p { super::button::Button { onclick: move |_| {
   let s = (cx.props.action)();
   let re = cx.props.results.clone();
   to_owned![state];
   cx.spawn(async move {
    pin_mut!(s);
    while let Some(r) = s.next().await {
     state.set(match r {
      Ok(r) => { re.set(r.1); r.0.to_string() },
      Err(e) => e.to_string(),
     });
    };
   });
  }, &cx.props.children }, " {state}" }
 }
}
