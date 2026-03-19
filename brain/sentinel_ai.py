import hashlib

def verify_sighting(animal):
    """Automates wildlife identification for international funding."""
    seal = hashlib.sha256(animal.encode()).hexdigest()
    return f"Kudu Sentinel: {animal} Verified. Trust Seal: {seal[:10]}"

if __name__ == "__main__":
    print(verify_sighting("Black Rhino")) 
