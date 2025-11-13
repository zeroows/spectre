use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, Worker};
use serde::{Deserialize, Serialize};
use spectre::SpectreUserKey;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WorkerMessage {
    #[serde(rename = "generate_key")]
    GenerateKey { name: String, secret: String },
    #[serde(rename = "key_result")]
    KeyResult { 
        key_id: Vec<u8>,
        key_data: Vec<u8>,
        algorithm: u32,
    },
    #[serde(rename = "key_error")]
    KeyError { error: String },
}

impl WorkerMessage {
    pub fn to_spectre_key(&self) -> Option<SpectreUserKey> {
        match self {
            WorkerMessage::KeyResult { key_id, key_data, algorithm } => {
                if key_id.len() == 32 {
                    let mut key_id_array = [0u8; 32];
                    key_id_array.copy_from_slice(key_id);
                    Some(SpectreUserKey {
                        key_id: key_id_array,
                        key_data: key_data.clone(),
                        algorithm: *algorithm,
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    
    pub fn from_spectre_key(key: &SpectreUserKey) -> Self {
        WorkerMessage::KeyResult {
            key_id: key.key_id.to_vec(),
            key_data: key.key_data.clone(),
            algorithm: key.algorithm,
        }
    }
}

pub struct KeyWorker {
    worker: Option<Worker>,
}

impl KeyWorker {
    pub fn new() -> Result<Self, JsValue> {
        // Create an inline worker using a Blob URL
        let worker_js = include_str!("../assets/worker.js");
        
        let blob = web_sys::Blob::new_with_str_sequence(
            &js_sys::Array::of1(&JsValue::from_str(worker_js))
        )?;
        let url = web_sys::Url::create_object_url_with_blob(&blob)?;
        
        let worker = Worker::new(&url)?;
        
        Ok(Self {
            worker: Some(worker),
        })
    }
    
    pub async fn generate_key(
        &self,
        name: String,
        secret: String,
    ) -> Result<SpectreUserKey, String> {
        let worker = self.worker.as_ref().ok_or("Worker not initialized")?;
        
        // Create a channel for the result
        let (tx, rx) = futures::channel::oneshot::channel();
        let tx = std::sync::Arc::new(std::sync::Mutex::new(Some(tx)));
        
        // Set up message handler (one-time use)
        let tx_clone = tx.clone();
        let closure = Closure::wrap(Box::new(move |event: MessageEvent| {
            if let Some(data) = event.data().as_string() {
                if let Ok(msg) = serde_json::from_str::<WorkerMessage>(&data) {
                    match msg {
                        WorkerMessage::KeyResult { .. } => {
                            if let Some(key) = msg.to_spectre_key() {
                                if let Some(tx) = tx_clone.lock().unwrap().take() {
                                    let _ = tx.send(Ok(key));
                                }
                            } else {
                                if let Some(tx) = tx_clone.lock().unwrap().take() {
                                    let _ = tx.send(Err("Invalid key format".to_string()));
                                }
                            }
                        }
                        WorkerMessage::KeyError { error } => {
                            if let Some(tx) = tx_clone.lock().unwrap().take() {
                                let _ = tx.send(Err(error));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        
        worker.set_onmessage(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
        
        // Send message to worker
        let message = WorkerMessage::GenerateKey { name, secret };
        let message_json = serde_json::to_string(&message)
            .map_err(|e| format!("Serialization error: {}", e))?;
        
        worker.post_message(&JsValue::from_str(&message_json))
            .map_err(|_| "Failed to post message to worker".to_string())?;
        
        // Wait for response
        rx.await.map_err(|_| "Worker communication failed".to_string())?
    }
}

impl Drop for KeyWorker {
    fn drop(&mut self) {
        if let Some(worker) = self.worker.take() {
            worker.terminate();
        }
    }
}

