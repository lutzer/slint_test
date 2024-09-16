// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod settings;
mod ui;

use settings::Settings;
use slint::{CloseRequestResponse, ComponentHandle, PhysicalPosition, WindowPosition};
use ui::{AppState, App, AppGlobals};

const SETTINGS_PATH: &str = "settings.json";

impl App {
    pub fn get_state(self: &App) -> AppState {
        return self.global::<AppGlobals>().get_state();
    }
    
    pub fn set_state(self: &App, state: AppState) {
        self.global::<AppGlobals>().set_state(state); 
    }

    pub fn save(self: &App, path: &str) {
        let state = self.get_state();
        Settings {
            window_x : self.window().position().x,
            window_y : self.window().position().y,
            state : state
        }.save(path).unwrap();
    }
}

fn main() {
    let app = App::new().unwrap();

    let settings = Settings::load(SETTINGS_PATH).unwrap_or(Settings::default());

    app.set_state(AppState {
        counter: settings.state.counter,
        counter_opacity: settings.state.counter_opacity
    });

    app.window().set_position(WindowPosition::Physical(PhysicalPosition{ x: settings.window_x, y: settings.window_y}));

    app.global::<AppGlobals>().on_increase_clicked({
        let app_weak = app.as_weak();
        move || {
            let mut state = app_weak.unwrap().get_state();
            state.counter += 1;
            app_weak.unwrap().set_state(state);
            app_weak.unwrap().save(SETTINGS_PATH);
        }
    });

    app.global::<AppGlobals>().on_decrease_clicked({
        let app_weak = app.as_weak();
        move || {
            let mut state = app_weak.unwrap().get_state();
            state.counter -= 1;
            app_weak.unwrap().set_state(state);
            app_weak.unwrap().save(SETTINGS_PATH);
        }
    });

    app.global::<AppGlobals>().on_fade_clicked({
        let app_weak = app.as_weak();
        move || {
            let mut state = app_weak.unwrap().get_state();
            state.counter_opacity = if state.counter_opacity > 0.0 {0.0} else {1.0};
            app_weak.unwrap().set_state(state);
            app_weak.unwrap().save(SETTINGS_PATH);
        }
    });

    app.window().on_close_requested({
        let app_weak = app.as_weak();
        move || {
            app_weak.unwrap().save(SETTINGS_PATH);
            return CloseRequestResponse::HideWindow;
        }
    });

    app.run().unwrap();
}



