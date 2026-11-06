use std::sync::Mutex;
use tauri::{State};

struct Reward {
    probablity: f64, // 確率
    refund: i32, // 返還額
}

struct Ticket {
    price: u32,
    prize_pool: Vec<Reward>,
}

struct TicketList {
    ticket: Vec<Ticket>,
}

#[tauri::command]
fn sample() {
    let ticket_list = TicketList {
        ticket: vec! [
            Ticket {
                price: 500 ,
                prize_pool: vec! [
                    Reward { probablity: 0.3, refund: 1000 }, 
                    Reward { probablity: 0.25, refund: 800 }, 
                    Reward { probablity: 0.2, refund: 750 }, 
                    Reward { probablity: 0.15, refund: 700 }, 
                    Reward { probablity: 0.08, refund: 600 }, 
                    Reward { probablity: 0.02, refund: 100 }, 
                ]
            },
        ]
    };
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![sample])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
