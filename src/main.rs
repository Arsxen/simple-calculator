mod app;
mod evaluate;
mod expression;

use app::App;
use dioxus::desktop::WindowBuilder;

fn main() {
    let window = WindowBuilder::new()
        .with_title("Simple Calculator")
        .with_always_on_top(true)
        .with_resizable(true);

    dioxus::LaunchBuilder::new()
        .with_cfg(dioxus::desktop::Config::new().with_window(window))
        .launch(App);
}
