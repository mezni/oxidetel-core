use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ProvisioningRequest {
    pub order_id: Uuid,
    pub sim_id: Uuid,
    pub action: String, // 'ACTIVATE' or 'BAR'
}

pub struct OssProvisioningAdapter {
    mock_network_url: String,
}

impl OssProvisioningAdapter {
    pub fn new(mock_network_url: String) -> Self {
        Self { mock_network_url }
    }

    /// Sends an activation command to the network element simulator
    pub async fn trigger_activation(&self, order_id: Uuid, sim_id: Uuid) -> Result<bool, String> {
        let payload = ProvisioningRequest {
            order_id,
            sim_id,
            action: "ACTIVATE".to_string(),
        };

        // For MVP architecture, simulate successful network handshake response
        println!("📡 [OSS ADAPTER] Sending activation command to core network for SIM ID: {}", sim_id);
        
        // In production, this would execute:
        // let client = reqwest::Client::new();
        // let res = client.post(&self.mock_network_url).json(&payload).send().await;

        Ok(true)
    }
}
