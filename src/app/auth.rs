use super::*;

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
 let password_value = password.get().clone();

 rsx! {cx,
  TextField{value: password}
  ActionButton{action: move || auth_password(password_value.clone()), "Submit Password"}
 }
}
