# binancex

## Add to Cargo.toml

```
binancex = { git = "https://github.com/mbazarov/binancex", branch = "develop" }
```

## Optional Features

- **native-tls** (enabled by default): Enables TLS functionality provided by native-tls (on Linux usage openssl).
- **rustls-tls**: Enables TLS functionality provided by rustls (with webpki-roots).
- **serde_json** (enabled by default): Enables a serde_json for deserializing json.
- **simd_json**: Enables a simd-json instead of serde_json for deserializing json.
- **strict-enums**: Disable "Unknown" variant in enums.
- **schemes-strict-enums**: Disable "Unknown" variant in enums for schemes
- **types-strict-enums**: Disable "Unknown" variant in enums for types

## Tests

```
cargo test -p binance-types --features strict-enums
cargo test --features strict-enums
```

## Benchmarks

Example for a CPU with 4x physical cores and HT:
```
// List CPU cores with ids 
egrep "((id|processo).*:|^ *$)" /proc/cpuinfo

// Pin all system processes to specific cores
sed -i 's/\WCPUAffinity=/CPUAffinity=0,1,2,4,5,6/g' /etc/systemd/system.conf
reboot

// After reboot
CPU_AFFINITY=3 cargo bench --bench bench_name
```
