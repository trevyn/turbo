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

// https://geo-ant.github.io/blog/2021/rust-traits-and-variadic-functions/

pub trait ActionFn<A> {
 type Output;

 fn call(&self, q: String) -> Self::Output;
 fn is_textfield(&self) -> bool;
}

impl<F, S> ActionFn<()> for F
where
 F: Fn() -> S + 'static,
{
 type Output = S;

 fn call(&self, _q: String) -> S {
  self()
 }
 fn is_textfield(&self) -> bool {
  false
 }
}

impl<F, S> ActionFn<String> for F
where
 F: Fn(String) -> S + 'static,
{
 type Output = S;

 fn call(&self, q: String) -> S {
  self(q)
 }
 fn is_textfield(&self) -> bool {
  true
 }
}

#[frontend]
#[derive(Props)]
pub struct ActionButtonProps<
 'a,
 A,
 S: Stream<Item = Result<P, E>> + 'static,
 F: ActionFn<A, Output = S>,
 P: Progress<R>,
 E: ToString,
 R: 'static,
> {
 action: F,
 _phantomdata: Option<A>,
 results: Option<&'a UseState<Option<R>>>,
 children: Element<'a>,
}

#[frontend]
pub fn ActionButton<'a, A, S, F, P, E, R>(
 cx: Scope<'a, ActionButtonProps<'a, A, S, F, P, E, R>>,
) -> Element<'a>
where
 S: Stream<Item = Result<P, E>> + 'static,
 F: ActionFn<A, Output = S>,
 P: Progress<R>,
 E: ToString,
 R: 'static,
{
 let state = use_state(&cx, String::new);

 let text = use_state(&cx, String::new);
 let text_value = text.get().clone();

 let textfield = if cx.props.action.is_textfield() {
  Some(rsx! {cx, super::textfield::TextField{value: text}})
 } else {
  None
 };

 rsx! {cx,
  div {
   textfield
   super::button::Button { onclick: move |_| {
    let s = cx.props.action.call(text_value.clone());
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
   }, &cx.props.children }
   " {state}"
  }
 }
}
