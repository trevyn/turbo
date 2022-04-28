use arc_swap::ArcSwapOption;
use eframe::egui;
use std::sync::Arc;
use turbocharger::futures_util::{self, StreamExt};
use turbocharger::prelude::*;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
#[derive(Default)]
pub struct TemplateApp {
 label: String,
 #[cfg_attr(feature = "persistence", serde(skip))]
 value: f32,
 encrypted_animal_time_stream: Arc<ArcSwapOption<String>>,
 mail_list: Arc<ArcSwapOption<Vec<String>>>,
 check_for_updates: Arc<ArcSwapOption<Result<String, tracked::StringError>>>,
 selected_anchor: String,
 num_frames: u32,
}

fn do_backend<T: 'static>(
 ctx: &egui::Context,
 store: &Arc<ArcSwapOption<T>>,
 fut: impl std::future::Future<Output = T> + 'static,
) {
 let ctx = ctx.clone();
 let store = store.clone();
 wasm_bindgen_futures::spawn_local(async move {
  store.store(Some(Arc::new(fut.await)));
  ctx.request_repaint();
 });
}

fn do_backend_stream<T: 'static, U>(
 ctx: &egui::Context,
 store: &Arc<ArcSwapOption<T>>,
 stream: impl futures_util::stream::Stream<Item = U> + 'static,
 transform: impl Fn(U) -> T + 'static,
) {
 let ctx = ctx.clone();
 let store = store.clone();
 let mut stream = Box::pin(stream);
 wasm_bindgen_futures::spawn_local(async move {
  while let Some(item) = stream.next().await {
   store.store(Some(Arc::new(transform(item))));
   ctx.request_repaint();
  }
 });
}

impl TemplateApp {
 pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
  let ctx = &cc.egui_ctx;
  let mut style = ctx.style().as_ref().clone();
  style.visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::WHITE;
  style.visuals.widgets.active.fg_stroke.color = egui::Color32::WHITE;
  style.visuals.widgets.inactive.fg_stroke.color = egui::Color32::WHITE;
  ctx.set_style(style);

  // Load previous app state (if any).
  // Note that you must enable the `persistence` feature for this to work.
  // #[cfg(feature = "persistence")]
  // if let Some(storage) = _storage {
  //  *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
  // }

  let s = Self::default();

  do_backend(ctx, &s.check_for_updates, crate::backend::check_for_updates());

  do_backend(ctx, &s.mail_list, async {
   crate::backend::mail_list()
    .await
    .unwrap()
    .into_iter()
    .map(|mail| {
     let decrypted = crate::wasm_decrypt(mail.data.unwrap()).unwrap();
     let parsed = mailparse::parse_mail(&decrypted.as_bytes()).unwrap();
     let body = parsed
      .subparts
      .iter()
      .find(|subpart| subpart.ctype.mimetype == "text/plain")
      .map(|subpart| subpart.get_body().unwrap());
     body.unwrap_or_default()
    })
    .collect()
  });

  do_backend_stream(
   ctx,
   &s.encrypted_animal_time_stream,
   crate::backend::encrypted_animal_time_stream(),
   |r| crate::wasm_decrypt(r.unwrap()).unwrap_or("decrypt error".into()),
  );

  s
 }
}

impl eframe::App for TemplateApp {
 // fn max_size_points(&self) -> Vec2 {
 //  Vec2::new(2048.0, 1024.0)
 // }

 /// Called by the frame work to save state before shutdown.
 #[cfg(feature = "persistence")]
 fn save(&mut self, storage: &mut dyn epi::Storage) {
  epi::set_value(storage, epi::APP_KEY, self);
 }

 /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
 fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
  let Self { label, value, .. } = self;

  egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
   ui.horizontal_wrapped(|ui| {
    ui.spacing_mut().button_padding = egui::vec2(10.0, 8.0);

    egui::widgets::global_dark_light_mode_switch(ui);

    for (name, anchor) in
     vec![("📪 Mail", "mail"), ("🐙 Animal Time", "animal-time"), ("⛭ Settings", "settings")]
      .into_iter()
    {
     let label = ui.selectable_label(
      self.selected_anchor == anchor,
      egui::RichText::new(name).font(egui::FontId::proportional(20.0)),
     );
     if label.hovered() {
      ui.output().cursor_icon = egui::CursorIcon::PointingHand;
     }
     if label.clicked() {
      self.selected_anchor = anchor.to_owned();
     }
    }
   })
  });

  egui::SidePanel::left("side_panel").show(ctx, |ui| {
   ui.heading("Side Panel");

   ui.horizontal(|ui| {
    ui.label("Write something: ");
    ui.text_edit_singleline(label);
   });

   ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
   if ui.button("Increment").clicked() {
    *value += 1.0;
   }

   self.num_frames += 1;
   ui.label(format!("num_frames: {}", self.num_frames));
   ui.label(format!("last frame: {:.0} ms", frame.info().cpu_usage.unwrap_or_default() * 1000.0));

   egui::warn_if_debug_build(ui);
  });

  egui::CentralPanel::default().show(ctx, |ui| {
   // The central panel the region left after adding TopPanel's and SidePanel's

   let mut size = ui.available_size();
   size.y = 0.0;

   egui::ScrollArea::vertical().show(ui, |ui| {
    ui.add_sized(
     size,
     egui::TextEdit::multiline(
      &mut format!("check_for_updates: {:?}", self.check_for_updates.load()).as_str(),
     )
     .font(egui::FontId::proportional(15.0))
     .desired_rows(2),
    );

    if let Some(encrypted_animal_time) = self.encrypted_animal_time_stream.load().as_deref() {
     ui.add_sized(
      size,
      egui::TextEdit::multiline(&mut encrypted_animal_time.as_str())
       .font(egui::FontId::proportional(15.0))
       .desired_rows(2),
     );
    }

    // for mail in self.mail_list.load().iter() {
    if let Some(mail_list) = self.mail_list.load().as_deref() {
     if let Some(mail) = mail_list.first() {
      ui.add_sized(
       size,
       egui::TextEdit::multiline(&mut mail.as_str()).font(egui::FontId::proportional(15.0)),
      );
     }
    }
   });
  });
 }
}
