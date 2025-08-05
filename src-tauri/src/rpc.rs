use std::sync::Arc;
use tokio::sync::Mutex;

use fedimint_client_rpc::{RpcGlobalState, RpcRequest, RpcResponseHandler};
use fedimint_core::db::Database;
use fedimint_client_rpc::RpcResponse;
use tauri::{AppHandle, State, Manager};
use tauri::Emitter;

// Application state to hold the RPC handler
pub struct AppState {
    rpc_handler: Arc<Mutex<Option<RpcHandler>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            rpc_handler: Arc::new(Mutex::new(None)),
        }
    }
}

struct RpcHandler {
    state: Arc<RpcGlobalState>,
}
impl RpcHandler {
    // only duplicate error here u contiue
    pub fn new(app_handle: AppHandle) -> Result<Self, String> {
        use fedimint_rocksdb::RocksDb;
        use std::fs;

        let app_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;

        // Create the database directory if it doesn't exist
        let db_path = app_dir.join("fedimint_db");
        fs::create_dir_all(&db_path).map_err(|e| e.to_string())?;
        
        // Create the database
        let rocksdb = RocksDb::open_blocking_unlocked(&db_path).map_err(|e| e.to_string())?;
        let database = Database::new(rocksdb, Default::default());
        
        let state = Arc::new(RpcGlobalState::new(database));

        Ok(Self { state })
    }
}


// Initialize the RPC handler
#[tauri::command]
pub async fn initialize_rpc_handler(app_handle: AppHandle, state: State<'_, AppState>) -> Result<String, String> {
    let mut handler_guard = state.rpc_handler.lock().await;
    
    if handler_guard.is_some() {
        return Ok("RPC handler already initialized".to_string());
    }
    
    let handler = RpcHandler::new(app_handle)?;
    *handler_guard = Some(handler);
    
    Ok("RPC handler initialized successfully".to_string())
}



use tokio::sync::mpsc;

#[tauri::command]
pub async fn handle_rpc_request(
    request: String, 
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    let request: RpcRequest = serde_json::from_str(&request)
        .map_err(|e| format!("Failed to parse request: {}", e))?;
    
    let request_id = request.request_id;
    let (tx, mut rx) = mpsc::unbounded_channel::<RpcResponse>();
    
    struct TauriChannelHandler(mpsc::UnboundedSender<RpcResponse>);
    
    impl RpcResponseHandler for TauriChannelHandler {
        fn handle_response(&self, response: RpcResponse) {
            let _ = self.0.send(response);
        }
    }
    
    // Get the RPC handler from the state
    let handler_guard = state.rpc_handler.lock().await;
    let handler = handler_guard.as_ref()
        .ok_or_else(|| "RPC handler not initialized. Call initialize_rpc_handler first.".to_string())?;
    
    let state_clone = handler.state.clone();
    drop(handler_guard); // Release the lock early
    
    let handled = state_clone.handle_rpc(request, TauriChannelHandler(tx));
    
    if let Some(task) = handled.task {
        tokio::spawn(task);
    }
    
    // Spawn a separate task to handle the responses
    let app_clone = app.clone();
    tokio::spawn(async move {
        while let Some(response) = rx.recv().await {
            let is_end = matches!(response.kind, 
                fedimint_client_rpc::RpcResponseKind::End {} | 
                fedimint_client_rpc::RpcResponseKind::Aborted {}
            );
            
            let _ = app_clone.emit(
                &format!("fedimint-response-{}", request_id),
                &response
            );
            
            if is_end {
                break;
            }
        }
    });
    
    Ok(())
}

#[tauri::command]
pub async fn get_app_status(state: State<'_, AppState>) -> Result<String, String> {
    let handler_guard = state.rpc_handler.lock().await;
    
    if handler_guard.is_some() {
        Ok("RPC handler is initialized and ready".to_string())
    } else {
        Ok("RPC handler is not initialized".to_string())
    }
}