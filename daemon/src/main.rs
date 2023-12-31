use std::{
    process,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use tokio::select;
use tracing::info;

mod api;
mod controllers;
mod scripts;
mod sound;
mod state;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    info!("Starting daemon");

    let (controller_tx, mut controller_rx) = tokio::sync::mpsc::channel(32);
    let controllers: Arc<Mutex<Vec<Arc<Box<dyn controllers::Controller>>>>> =
        Arc::new(Mutex::new(Vec::new()));

    let state = Arc::new(state::AppState {
        controller_tx,
        controllers,
    });

    let state1 = state.clone();
    tokio::spawn(async move {
        while let Some(controller) = controller_rx.recv().await {
            info!("Received controller");
            controller.initialize().unwrap();

            state1.controllers.lock().unwrap().push(controller.clone());
        }
    });

    // let mut controllers: Vec<Arc<Box<dyn Alles>>> = Vec::new();

    // let controller: Arc<Box<dyn Alles>> = Arc::new(LaunchpadMiniMk1::guess().unwrap());
    // controllers.push(controller.clone());
    // let controller2: Arc<Box<dyn Alles>> = Arc::new(LaunchpadMiniMk3::guess().unwrap());
    // controllers.push(controller2.clone());

    // controller.initialize().unwrap();
    // controller2.initialize().unwrap();

    // let mut script = scripts::ping::PingScript::new();

    // let controller1 = controller.clone();
    // tokio::spawn(async move { controller1.run(&mut script).unwrap() });

    // let mut script2 = scripts::soundboard::SoundboardScript::new();

    // let controller21 = controller2.clone();
    // tokio::spawn(async move { controller21.run(&mut script2).unwrap() });

    select! {
        _ = api::serve(state) => {},
        _ = tokio::signal::ctrl_c() => {
            info!("Received SIGINT, shutting down");
        },
    }

    // controller.clear().unwrap();
    // controller2.clear().unwrap();

    thread::sleep(Duration::from_millis(100));

    process::exit(0);
}
