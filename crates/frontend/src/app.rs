use shared::TextPayload;
use std::sync::mpsc::{Receiver, Sender, channel};

enum AppEvent {
    TextReceived(String),
    StatusUpdate(String),
}

struct ChannelPair {
    tx: Sender<AppEvent>,
    rx: Receiver<AppEvent>,
}

impl Default for ChannelPair {
    fn default() -> Self {
        let (tx, rx) = channel();
        Self { tx, rx }
    }
}

fn get_api_url(path: &str) -> String {
    #[cfg(not(target_arch = "wasm32"))]
    {
        format!("http://127.0.0.1:3000{}", path)
    }
    #[cfg(target_arch = "wasm32")]
    {
        path.to_string()
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    label: String,

    #[serde(skip)]
    value: f32,

    #[serde(skip)]
    status_message: String,

    #[serde(skip)]
    channel: ChannelPair,
}

impl Default for App {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            value: 0.0,
            status_message: String::new(),
            channel: ChannelPair::default(),
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }

    fn send_to_server(&self, ctx: egui::Context) {
        let payload = TextPayload {
            text: self.label.clone(),
        };

        match serde_json::to_vec(&payload) {
            Ok(json_bytes) => {
                let mut request = ehttp::Request::post(get_api_url("/api/text"), json_bytes);
                request.headers.insert("Content-Type", "application/json");

                let tx = self.channel.tx.clone();
                ehttp::fetch(request, move |result| {
                    match result {
                        Ok(response) => {
                            if response.ok {
                                let _ = tx.send(AppEvent::StatusUpdate(
                                    "Text successfully sent to server!".to_string(),
                                ));
                            } else {
                                let _ = tx.send(AppEvent::StatusUpdate(format!(
                                    "Server error: HTTP {}",
                                    response.status
                                )));
                            }
                        }
                        Err(err) => {
                            let _ =
                                tx.send(AppEvent::StatusUpdate(format!("Network error: {err}")));
                        }
                    }
                    ctx.request_repaint();
                });
            }
            Err(err) => {
                let _ = self.channel.tx.send(AppEvent::StatusUpdate(format!(
                    "Serialization error: {err}"
                )));
            }
        }
    }

    fn get_from_server(&self, ctx: egui::Context) {
        let request = ehttp::Request::get(get_api_url("/api/text"));
        let tx = self.channel.tx.clone();
        ehttp::fetch(request, move |result| {
            match result {
                Ok(response) => {
                    if response.ok {
                        if let Ok(payload) = serde_json::from_slice::<TextPayload>(&response.bytes)
                        {
                            let _ = tx.send(AppEvent::TextReceived(payload.text));
                            let _ = tx.send(AppEvent::StatusUpdate(
                                "Text received from server!".to_string(),
                            ));
                        } else {
                            let _ = tx.send(AppEvent::StatusUpdate(
                                "Failed to parse JSON response".to_string(),
                            ));
                        }
                    } else {
                        let _ = tx.send(AppEvent::StatusUpdate(format!(
                            "Server error: HTTP {}",
                            response.status
                        )));
                    }
                }
                Err(err) => {
                    let _ = tx.send(AppEvent::StatusUpdate(format!("Network error: {err}")));
                }
            }
            ctx.request_repaint();
        });
    }
}

impl eframe::App for App {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Process any messages received asynchronously from the server
        while let Ok(event) = self.channel.rx.try_recv() {
            match event {
                AppEvent::TextReceived(text) => {
                    self.value = text.chars().count() as f32;
                    self.label = text;
                }
                AppEvent::StatusUpdate(status) => {
                    self.status_message = status;
                }
            }
        }

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Mátábor Foglaló 2026");

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                let resp = ui.text_edit_singleline(&mut self.label);
                if resp.changed() {
                    self.value = self.label.chars().count() as f32;
                }
            });

            ui.separator();
            ui.label(format!("Text length: {}", self.value));

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                if ui.button("Send to server").clicked() {
                    self.send_to_server(ui.ctx().clone());
                }
                if ui.button("Get from server").clicked() {
                    self.get_from_server(ui.ctx().clone());
                }
            });

            if !self.status_message.is_empty() {
                ui.add_space(8.0);
                ui.label(egui::RichText::new(&self.status_message).italics());
            }
        });
    }
}
