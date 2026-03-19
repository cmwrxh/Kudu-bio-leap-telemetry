// Kudu Biotech: KIES-Core 
// High-Integrity Tracking for Carbon, Conservation, and Tourism

#[derive(Debug)]
enum KuduFunction {
    BioLeap(String),   // Carbon Credits: Tree Species or Forest Block
    Sentinel(String),  // Conservation/Tourism: Animal ID (e.g., Rhino-001)
    KuduEnergy(f64),   // Social Economics: Conservancy Solar/Grid Kilowatts
}

struct Registry {
    id: String,
    function: KuduFunction,
    location: String,
    carbon_impact: f64, // Metric tonnes of CO2 tracked
}

fn main() {
    // 1. Tracking for Global Warming / Carbon Credits
    let forest_block = Registry {
        id: "BLOCK-MACHA-01".to_string(),
        function: KuduFunction::BioLeap("Indigenous Mix".to_string()),
        location: "Machakos Conservancy".to_string(),
        carbon_impact: 450.75, 
    };

    // 2. Tracking for Wildlife Conservation & Tourists
    let rhino_sighting = Registry {
        id: "SENTINEL-ID-99".to_string(),
        function: KuduFunction::Sentinel("Black Rhino".to_string()),
        location: "Ol Pejeta Sector 4".to_string(),
        carbon_impact: 0.0, // Wildlife has biodiversity value, not direct carbon
    };

    println!("--- KUDU BIOTECH CORE RUNNING ---");
    println!("TRACKING CARBON: {:?} at {} | Impact: {}t CO2", 
              forest_block.function, forest_block.location, forest_block.carbon_impact);
    println!("TRACKING WILDLIFE: {:?} | ID: {} | Loc: {}", 
              rhino_sighting.function, rhino_sighting.id, rhino_sighting.location);
}