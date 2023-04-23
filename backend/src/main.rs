mod node;
mod os;

use axum::{extract::State as AxumState, response::Html as AxumHtml, routing::get, Router};
use node::{Node, NodeManager};
use os::*;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{task, time};

pub struct AxumSharedData {
    pub os: OperatingSystem,
    pub node: Node,
    pub node_manager: NodeManager,
}

impl AxumSharedData {
    pub fn new() -> Self {
        Self {
            os: OperatingSystem::new(),
            node: Node::new(),
            node_manager: NodeManager::default(),
        }
    }

    pub fn refresh(&mut self) {
        self.os.refresh();
        self.node.refresh(&self.os);
    }

    pub fn stop_node(&mut self) -> Result<String, String> {
        self.node_manager.stop_node(&self.node)
    }

    pub fn start_node(&mut self) -> Result<String, String> {
        self.node_manager.start_node(&self.node)
    }
}

pub type State = AxumState<Arc<Mutex<AxumSharedData>>>;

#[tokio::main]
async fn main() {
    println!("Hello World!");

    let state = Arc::new(Mutex::from(AxumSharedData::new()));
    let a = state.clone();

    let forever = task::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(250));
        println!("Hello");

        loop {
            interval.tick().await;
            if let Ok(mut state) = a.lock() {
                state.refresh();
            }
        }
    });

    let app = Router::new()
        .route("/", get(root))
        .route("/start", get(start_node))
        .route("/stop", get(stop_node))
        .route("/log", get(get_log))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    _ = forever.await;
}

// basic handler that responds with a static string
async fn root(state: State) -> AxumHtml<String> {
    let state = state.lock().unwrap();
    AxumHtml(state.node.serialize())
}

async fn start_node(state: State) -> AxumHtml<String> {
    println!("/start Endpoint");
    let mut state = state.lock().unwrap();
    AxumHtml(match state.start_node() {
        Ok(x) => x,
        Err(x) => x,
    })
}

async fn stop_node(state: State) -> AxumHtml<String> {
    println!("/stop Endpoint");
    let mut state = state.lock().unwrap();
    AxumHtml(match state.stop_node() {
        Ok(x) => x,
        Err(x) => x,
    })
}

// basic handler that responds with a static string
async fn get_log(state: State) -> AxumHtml<String> {
    println!("/log Endpoint");
    let res = std::fs::read_to_string("/home/marko/Projects/centrality/seed/stderr_log.txt");
    let Ok(res) = res else {
        return AxumHtml(String::from("Error"));
    };

    AxumHtml(res)
}
