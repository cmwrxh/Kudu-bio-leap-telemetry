import hashlib
from datetime import datetime, timezone

class KuduTelemetry:
    def __init__(self, sensor_id):
        self.sensor_id = sensor_id
        self.firmware_version = "1.0.2"

    def record_biomass_data(self, dbh, height):
        # Proprietary KIES formula
        biomass_index = (float(dbh) ** 2) * float(height) * 0.11
        
        # FIX: Using the new 2026-compliant UTC method
        timestamp = datetime.now(timezone.utc).isoformat()
        
        raw_data = f"{self.sensor_id}|{biomass_index}|{timestamp}"
        trust_seal = hashlib.sha256(raw_data.encode()).hexdigest()

        return {
            "timestamp": timestamp,
            "biomass_index": round(biomass_index, 4),
            "trust_seal": trust_seal
        }

if __name__ == "__main__":
    kies = KuduTelemetry(sensor_id="KUDU-NODE-001")
    print(kies.record_biomass_data(dbh=24.5, height=15.2))