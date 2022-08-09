#[frontend]
use super::components::*;
use turbocharger::prelude::*;
use turbosql::{select, Turbosql};

#[derive(Clone, Debug, Turbosql)]
pub struct auth {
 pub rowid: Option<i64>,
 pub password: Option<String>,
}

#[backend]
pub fn auth_password(password: String) -> impl Stream<Item = Result<String, tracked::StringError>> {
 try_stream!({
  connection_local!(authed: &mut bool);

  let auth = select!(Option<auth> "WHERE rowid = 1")?;

  match auth {
   Some(auth) => {
    if auth.password == Some(password) {
     *authed = true;
     yield "logged in!".into();
    } else {
     Err("wrong password")?;
    }
   }
   None => {
    auth { rowid: None, password: Some(password) }.insert()?;
    *authed = true;
    yield "password set.".into();
   }
  }
 })
}

#[frontend]
pub fn Auth(cx: Scope) -> Element {
 let password = use_state(&cx, || "bob".to_string());
 let password_string = password.get().clone();

 rsx! {cx,
  input { class: "font-mono text-gray-600",
   "type": "text",
   value: "{password}",
   oninput: move |evt| password.set(evt.value.clone()),
  }

  ActionButton{action: move || auth_password(password_string.clone()), "Submit Password"}
 }
}
