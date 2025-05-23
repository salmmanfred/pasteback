use std::{collections::HashMap, sync::Mutex, thread, time};

use axum::extract::{Json, Path};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref TOKENS: Mutex<HashMap<String, Paste>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Paste {
    save_time: i64,
    text: String,
}

impl Paste {
    pub fn new(s: String, i: i64) -> Self {
        Self {
            save_time: i,
            text: s,
        }
    }
}
pub async fn post_token(Json(item): Json<Paste>) -> String {
    //let item = Paste::new(item.0,item.1);
    let mut ttl = item.save_time;
    if ttl > 10 {
        ttl = 10;
    }
    if ttl < 1 {
        ttl = 1;
    }
    if item.text.len() > 10 * 1024 {
        return "Too big message".to_string();
    }
    let key = create_key(item);

    delete_after(key.clone(), ttl as u64);

    return key;
}

pub async fn ret_token(Path(params): Path<String>) -> Json<Paste> {
    let mut tl = TOKENS.lock().unwrap();
    let token = params;

    if tl.contains_key(&token) {
        let a = tl.remove(&token).unwrap();
        drop(tl);
        return Json(a);
    } else {
        drop(tl);

        return Json(Paste::new("Wrong token/token expiered".to_string(), 0));
    }
}

fn create_key(paste: Paste) -> String {
    let mut tl = TOKENS.lock().unwrap();
    let mut key = "".to_string();
    while key == "" {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(4)
            .map(char::from)
            .collect();
        if !tl.contains_key(&s) {
            key = s;
        }
    }
    tl.insert(key.clone(), paste);
    drop(tl);
    return key;
}

fn delete_after(key: String, ttl: u64) {
    thread::spawn(move || {
        let time = time::Duration::from_secs(ttl * 60);
        thread::sleep(time);
        let mut tl = TOKENS.lock().unwrap();

        tl.remove(&key);
        drop(tl)
    });
}
