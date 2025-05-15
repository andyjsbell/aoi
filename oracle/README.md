# Oracle: Geographical Location Cryptographic Oracle

A Rust-based service that obtains geographical location data, converts it to a geohash representation, and cryptographically signs it using Ed25519. This creates verifiable proofs of geographical position that can be used in various applications such as supply chain tracking, geographic-based authentication, and location verification.

## Features

- 📍 **Location Detection**: Determines the current geographic location using IP geolocation
- 🧩 **Geohash Encoding**: Converts location data to geohash strings with configurable precision
- 🔐 **Cryptographic Signing**: Signs location data using Ed25519 digital signatures
- 🛠️ **Command-line Interface**: Easy-to-use CLI with key generation and signing operations
- ⚙️ **Flexible Implementation**: Modular design with traits for extensibility

## Installation

### Prerequisites

- Rust and Cargo (1.70.0 or later)
- Internet connection (for IP geolocation)

### Building from Source

```bash
git clone https://github.com/your-organization/oracle.git
cd oracle
cargo build --release
```

The compiled binary will be available at `target/release/oracle`.

## Usage

### Generating a Key Pair

Generate a new Ed25519 key pair for signing:

```bash
./oracle generate
```

This will output a private and public key pair in hexadecimal format:

```
Private=0x1a2b3c4d...
Public=0x5e6f7g8h...
```

The private key is used for signing location data, while the public key can be shared with others to verify your signed locations.

### Obtaining and Signing a Location

Run the oracle to get your current location, encode it as a geohash, and sign it:

```bash
./oracle run --key=0x1a2b3c4d... --accuracy=6
```

Alternatively, you can set the key as an environment variable:

```bash
export ORACLE_KEY=1a2b3c4d...
./oracle run --accuracy=8
```

#### Accuracy Parameter

The `accuracy` parameter controls the precision of the geohash:

- `1-3`: Country/region level (low precision, ~250-2500km)
- `4-5`: City level (medium precision, ~2.4-39km)
- `6-7`: Neighborhood level (high precision, ~0.15-2.4km)
- `8-9`: Street level (very high precision, ~0.02-0.3km)
- `10-12`: Building level (extremely high precision, ~0.001-0.019km)

Higher accuracy results in longer geohash strings and more precise location data.

### Output Format

The signed location is output as a JSON-encoded byte array representing the Ed25519 signature:

```json
[123,45,67,...]
```

## Technical Architecture

### Core Components

- **Location Service**: Retrieves geographical coordinates using IP geolocation
- **Geohash Encoder**: Converts coordinates to geohash strings
- **Blake2-256 Hasher**: Creates cryptographic hashes of location data
- **Ed25519 Signer**: Generates digital signatures for location hashes

### Key Files

- `lib.rs`: Core traits and types for the oracle functionality
- `geohash.rs`: Implementation of location detection and geohash encoding
- `blake2_256.rs`: Cryptographic hashing module
- `ed25519.rs`: Digital signature module
- `env.rs`: Environment and key management utilities
- `main.rs`: CLI application implementation

### Extensibility

The system is designed with traits that can be implemented to extend functionality:

- `Location`: For different location data sources or formats
- `Hasher`: For different cryptographic hash algorithms
- `Signer`: For different signature schemes

## Security Considerations

- **Private Key Management**: Keep your private key secure; anyone with access to it can generate signatures that appear to come from you
- **Environment Variables**: Using environment variables for keys is convenient but may be less secure in some environments
- **IP Geolocation Limitations**: IP-based geolocation has varying accuracy and can be affected by VPNs, proxies, etc.

## Development

### Building and Testing

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run the application in debug mode
cargo run -- generate
cargo run -- run --accuracy=6
```

### Adding New Location Sources

The `Location` trait can be implemented for different data sources:

```rust
use oracle::{Location, LocationError};

struct MyLocationProvider;

#[async_trait::async_trait]
impl Location for MyLocationProvider {
    type Output = String;
    
    async fn current_location(accuracy: u8) -> Result<Self::Output, LocationError> {
        // Your implementation here
    }
}
```

## License

[MIT License](LICENSE)

## Acknowledgments

- [ed25519-dalek](https://github.com/dalek-cryptography/ed25519-dalek) for Ed25519 signature implementation
- [geohash](https://github.com/georust/geohash) for geohash encoding/decoding
- [ipinfo.io](https://ipinfo.io/) for IP geolocation services

---

Built with ❤️ for secure, verifiable location proofs.