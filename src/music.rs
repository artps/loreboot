use serde::Serialize;
use ureq;
use uuid::Uuid;

#[derive(Serialize)]
struct MusicEvent<'a> {
    event: &'a str,
}

pub fn send_event(session_id: Uuid, event: &str) {
    let url = format!("http://srvn.sh:3000/session/{}", session_id);
    let payload = MusicEvent { event };

    let res = ureq::post(&url)
        .content_type("application/json")
        .send_json(serde_json::to_value(payload).unwrap());

    match res {
        Ok(response) => {
            println!("[AUDIO SYNC] Sent event: {}", event);
        }

        Err(e) => {
            eprintln!(
                "[AUDIO ERROR] Failed to send event: {} — {}",
                event,
                e.to_string(),
            );
        }
    }
}
