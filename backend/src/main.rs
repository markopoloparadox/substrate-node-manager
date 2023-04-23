mod os;

use axum::{extract::State as AxumState, response::Html as AxumHtml, routing::get, Router};
use os::*;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

/* pub fn total_system_utilization() -> (u32, u32) {
    let arg =
    let out = Command::new("sh").args(["-c"])
        .arg("-c")
        .arg("pidof seed")
        .output()
        .unwrap();
}

pub fn get_pid_of(name: &str) -> (Option<u32>, Option<String>) {
    let out = Command::new("sh")
        .arg("-c")
        .arg("pidof seed")
        .output()
        .unwrap();
    let format = format_command_output(out);
    let mut pid = None;
    if let Some(p) = format.0 {
        pid = Some(p.parse().unwrap());
    }
    (pid, format.1)
}

pub fn get_node_health(pid: u32) -> (Option<String>, Option<String>) {
    let arg = format!("ps -p {pid:} -o %cpu,%mem -h");
    let out = Command::new("sh").arg("-c").arg(arg).output().unwrap();
    format_command_output(out)
} */

pub struct AxumSharedData {
    pub os: OperatingSystem,
}

pub type State = AxumState<Arc<Mutex<AxumSharedData>>>;

#[tokio::main]
async fn main() {
    println!("Hello World!");

    let axum_data = AxumSharedData {
        os: OperatingSystem::new(),
    };
    let state = Arc::new(Mutex::from(axum_data));

    let app = Router::new()
        .route("/", get(root))
        /*         .route("/action", get(action))
        .route("/refresh", get(refresh)) */
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root(state: State) -> AxumHtml<String> {
    let mut state = state.lock().unwrap();
    state.os.refresh();

    AxumHtml(state.os.serialize())
}

/* // basic handler that responds with a static string
async fn action(state: State<bool>) {
    dbg!("Hello World!");
    let mut state = state.lock().unwrap();
    state.refresh = true;
}

// basic handler that responds with a static string
async fn refresh(state: State<bool>) -> Json<bool> {
    let mut state = state.lock().unwrap();
    let refresh = state.refresh;
    dbg!(state.refresh);
    state.refresh = false;
    Json::from(refresh)
}
 */
