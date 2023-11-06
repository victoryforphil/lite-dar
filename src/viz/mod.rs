use self::app::LiteDarApp;

pub mod app;

pub struct SimViz {}

impl SimViz {
    pub fn new() -> Self {
        let native_options = eframe::NativeOptions {
            initial_window_size: Some([1000.0, 800.0].into()),
            min_window_size: Some([300.0, 220.0].into()),
            ..Default::default()
        };
        eframe::run_native(
            "eframe template",
            native_options,
            Box::new(|cc| Box::new(LiteDarApp::new(cc))),
        );

        Self {}
    }
}
