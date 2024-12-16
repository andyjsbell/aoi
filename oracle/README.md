### Oracle

#### Notes
- Online service for GPS 
- Use Geohash to convert this
- Use Clap to create the CLI for Oracle
    - main.rs (is this in bin?)
    - lib.rs - core functionality with tests
- Sign against ed25519-dalek 
- Key required
    - Use commandline
    - Use env file


#### Steps
- Create library
    - gps.rs - request current GPS - this could be deep, best create a trait for this
    - geohash.rs - convert GPS to geohash
    - sign.rs - sign a payload using ed25519-dalek
    - tests
- Create binary
    - bin?
    - clap parsing or .env for PK
    - outfile path or stdout
    - call library
        - gps
        - geohash
        - sign
    - out and end
    

