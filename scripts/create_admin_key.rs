use redis::Commands;
use rand::RngCore;
use std::env;

const ADMIN_KEY_PREFIX: &str = "admin_key:";
const ADMIN_KEY_FORMAT_PREFIX: &str = "sk-admin-";

fn generate_admin_key() -> String {
    let mut rng = rand::thread_rng();
    let mut random_bytes = [0u8; 48]; // 384 bits of entropy
    rng.fill_bytes(&mut random_bytes);
    
    // Encode as base64 and remove padding characters
    let random_part = base64::encode_config(&random_bytes, base64::URL_SAFE_NO_PAD);
    
    // Combine prefix and random data
    format!("{}{}", ADMIN_KEY_FORMAT_PREFIX, random_part)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get Redis URL from command line or use default
    let redis_url = env::args().nth(1).unwrap_or_else(|| "redis://localhost:6379".to_string());
    
    // Generate admin key
    let admin_key = generate_admin_key();
    
    // Connect to Redis
    let client = redis::Client::open(redis_url)?;
    let mut conn = client.get_connection()?;
    
    // Store admin key in Redis
    let redis_key = format!("{}{}", ADMIN_KEY_PREFIX, admin_key);
    conn.set::<_, _, ()>(&redis_key, "admin")?;
    
    println!("Created admin key: {}", admin_key);
    println!("Use this key with the X-Admin-Key header to access API key management endpoints.");
    
    Ok(())
}
