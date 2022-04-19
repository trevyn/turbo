use eframe::{egui, epaint::Vec2};
use std::sync::{Arc, Mutex};
use turbocharger::futures_util::StreamExt;
use turbocharger::prelude::*;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
 // Example stuff:
 label: String,

 // this how you opt-out of serialization of a member
 #[cfg_attr(feature = "persistence", serde(skip))]
 value: f32,

 encrypted_animal_time_stream: Arc<Mutex<String>>,
}

impl TemplateApp {
 pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
  // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
  // Restore app state using cc.storage (requires the "persistence" feature).
  // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
  // for e.g. egui::PaintCallback.

  // Load previous app state (if any).
  // Note that you must enable the `persistence` feature for this to work.
  // #[cfg(feature = "persistence")]
  // if let Some(storage) = _storage {
  //  *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
  // }

  let s = Self {
   label: "Hello World!".to_owned(),
   value: 2.7,
   encrypted_animal_time_stream: Default::default(),
  };

  let mut stream = Box::pin(crate::backend::encrypted_animal_time_stream());
  let encrypted_animal_time_stream = s.encrypted_animal_time_stream.clone();
  let ctx = cc.egui_ctx.clone();
  wasm_bindgen_futures::spawn_local(async move {
   while let Some(item) = stream.next().await {
    *encrypted_animal_time_stream.lock().unwrap() = format!("{:?}", item);
    ctx.request_repaint();
    ::turbocharger::console_log!("{:?}", item);
   }
  });

  s
 }
}

impl eframe::App for TemplateApp {
 fn max_size_points(&self) -> Vec2 {
  Vec2::new(2048.0, 250.0)
 }

 /// Called by the frame work to save state before shutdown.
 /// Note that you must enable the `persistence` feature for this to work.
 #[cfg(feature = "persistence")]
 fn save(&mut self, storage: &mut dyn epi::Storage) {
  epi::set_value(storage, epi::APP_KEY, self);
 }

 /// Called each time the UI needs repainting, which may be many times per second.
 /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
 fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
  let Self { label, value, encrypted_animal_time_stream } = self;

  // Examples of how to create different panels and windows.
  // Pick whichever suits you.
  // Tip: a good default choice is to just keep the `CentralPanel`.
  // For inspiration and more examples, go to https://emilk.github.io/egui

  egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
   // The top panel is often a good place for a menu bar:
   egui::menu::bar(ui, |ui| {
    ui.menu_button("File", |ui| {
     if ui.button("Quit").clicked() {
      frame.quit();
     }
    });
   });
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
  });

  egui::CentralPanel::default().show(ctx, |ui| {
   // The central panel the region left after adding TopPanel's and SidePanel's

   ui.heading(encrypted_animal_time_stream.lock().unwrap().as_str());
   egui::warn_if_debug_build(ui);
  });
 }
}
