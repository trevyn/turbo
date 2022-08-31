use turbocharger::prelude::*;

pub trait Progress<R> {
 fn progress(&self) -> String;
 fn result(&self) -> Option<R>;
}

impl Progress<()> for String {
 fn progress(&self) -> String {
  self.clone()
 }
 fn result(&self) -> Option<()> {
  None
 }
}

impl<T, R> Progress<R> for (T, Option<R>)
where
 T: ToString,
 R: Clone,
{
 fn progress(&self) -> String {
  self.0.to_string()
 }
 fn result(&self) -> Option<R> {
  self.1.clone()
 }
}

#[frontend]
#[derive(Props)]
pub struct ActionButtonProps<
 'a,
 S: Stream<Item = Result<P, E>> + 'static,
 F: Fn() -> S + 'static,
 P: Progress<R>,
 E: ToString,
 R: 'static,
> {
 action: F,
 results: Option<&'a UseState<Option<R>>>,
 children: Element<'a>,
}

#[frontend]
pub fn ActionButton<'a, S, F, P, E, R>(
 cx: Scope<'a, ActionButtonProps<'a, S, F, P, E, R>>,
) -> Element<'a>
where
 S: Stream<Item = Result<P, E>> + 'static,
 F: Fn() -> S + 'static,
 P: Progress<R>,
 E: ToString,
{
 let state = use_state(&cx, String::new);

 rsx! {cx,
  p { super::button::Button { onclick: move |_| {
   let s = (cx.props.action)();
   let re = cx.props.results.cloned();
   to_owned![state];
   cx.spawn(async move {
    pin_mut!(s);
    while let Some(r) = s.next().await {
     state.set(match r {
      Ok(r) => {
       if let Some(ref re) = re { re.set(r.result()); };
       r.progress()
      },
      Err(e) => e.to_string(),
     });
    };
   });
  }, &cx.props.children }, " {state}" }
 }
}
