#![allow(clippy::type_complexity)]

use turbocharger::prelude::*;

#[frontend]
pub fn NavBar<'a>(cx: Scope<'a>, tabs: Vec<(&'static str, fn(Scope) -> Element)>) -> Element<'a> {
 let active_tab = use_state(&cx, || 0);
 let component = tabs[*active_tab.get()].1;

 // https://tailwindui.com/components/application-ui/navigation/navbars#component-aaed25b299f2015d2c4276b98d463cee
 rsx! {cx,
  nav { class: "bg-white shadow",
   div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
    div { class: "flex justify-between h-16",
     div { class: "flex",
      div { class: "hidden sm:ml-6 sm:flex sm:space-x-8",
       tabs.iter().enumerate().map(|(i, (title, _))| rsx! {
        Tab(title: title, active: i == *active_tab.get(), onclick: move |_| active_tab.set(i))
       })
      }
     }
    }
   }
  }
  component()
 }
}

#[frontend]
#[inline_props]
fn Tab<'a>(cx: Scope, title: &'static str, active: bool, onclick: EventHandler<'a>) -> Element {
 let classes = if *active {
  "border-indigo-500 text-gray-900"
 } else {
  "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700"
 };
 rsx! {cx,
  a { class: "{classes} inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium",
   href: "#",
   onclick: move |_| onclick.call(()),
   "{title}"
  }
 }
}
