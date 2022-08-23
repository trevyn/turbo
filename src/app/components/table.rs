// https://tailwindui.com/components/application-ui/lists/tables#component-e56f750c63d4e53a24f5f0bf9fd7b52a

use turbocharger::prelude::*;

#[frontend]
#[derive(Props)]
pub struct TableProps<'a, R: Serialize + 'static> {
 results: &'a UseState<Option<R>>,
}

#[frontend]
pub fn Table<'a, R>(cx: Scope<'a, TableProps<'a, R>>) -> Element<'a>
where
 R: Serialize + 'static,
{
 match cx.props.results.get() {
  None => rsx! {cx,
   p { "no results" }
  },
  Some(r) => {
   let v: serde_json::Value = serde_json::from_str(&serde_json::to_string(r).unwrap()).unwrap();
   value(cx, &v)
  }
 }
}

#[frontend]
fn value<'a, R>(cx: Scope<'a, TableProps<'a, R>>, v: &serde_json::Value) -> Element<'a>
where
 R: Serialize + 'static,
{
 match v {
  serde_json::Value::Null => rsx! {cx, "null"},
  serde_json::Value::Bool(b) => rsx! {cx, "{b}"},
  serde_json::Value::Number(n) => rsx! {cx, "{n}"},
  serde_json::Value::String(s) => rsx! {cx, "{s}"},
  serde_json::Value::Array(a) => {
   let rendered = a.iter().enumerate().map(|(i, v)| {
    let v = value(cx, v);
    rsx!(cx, p { "{i}: " v })
   });
   rsx! {cx,
    p { class:"p-4", rendered }
   }
  }
  serde_json::Value::Object(o) => {
   let rendered = o.iter().map(|(k, v)| {
    let v = value(cx, v);
    rsx!(cx, p { "{k}: " v })
   });
   rsx! {cx,
    p { class:"p-4", rendered }
   }
  }
 }
}
