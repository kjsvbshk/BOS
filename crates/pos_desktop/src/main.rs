// Oculta la ventana de consola en las compilaciones release de Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() -> eframe::Result {
    pos_desktop::run()
}
