use reqwest::blocking::Client;
use serde_json::Value;
use std::env;

// needed to add the token for api based bearer token output!
// export TOKEN=$(curl -u api-user:api-user-password -k -X POST "https://localhost:55000/security/user/authenticate?raw=true")

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("TOKEN").expect("TOKEN not set");
    
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;
    
    let wazuh_manager = "https://172.17.14.20:55000";
    let agent_id = "003"; 
    let endpoint = format!("{}/agents/{}/config/logcollector/localfile", wazuh_manager, agent_id);

    let response = client.get(&endpoint)
        .header("Authorization", format!("Bearer {}", token))
        .send()?;

    let config: Value = response.json()?;
    println!("Agent configuration: {:?}", config);

    Ok(())
}