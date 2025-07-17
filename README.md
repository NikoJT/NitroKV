# NitroKV

**NitroKV DB** is an ultra-performant, in-memory key-value database inspired by Redis, ripgrep, and embedded systems like Sled — but with a modern Rust-native twist.

> ⚡ Fast. 🔍 Searchable. 🧠 Smart. 📦 Embeddable.

---

## 📜 Project Goals

NitroKV-DB aims to be:
- **Blazing fast**: Zero-copy, memory-mapped reads and append-only writes
- **Minimal & practical**: Single binary, no external dependencies
- **Searchable**: Full-text or regex-based search support over values
- **Embeddable**: Usable as both a CLI tool and a Rust crate
- **Durable**: Crash-safe with journaling and snapshot recovery
- **Extendable**: Easy to add TTLs, pub/sub, possible versioning in the future.

---

## 🚀 Getting Started

### 🔧 Installation

```bash
git clone git@github.com:NikoJT/NitroKV.git
cd NitroKV
cargo build --release
```
### 🧪 Running NitroKV
```bash
cargo run -- set mykey "hello world"
cargo run -- get mykey
```

| Feature                                        | Status     |
| ---------------------------------------------- | ---------- |
| In-memory KV store                             | ✅ Done     |
| Basic commands (`set`, `get`, `del`, `exists`) | ✅ Done     |
| Append-only log persistence                    | ⬜ Planned  |
| Snapshot + restore                             | ⬜ Planned  |
| CLI interface                                  | ⬜ Planned  |
| Library crate support                          | ✅ Done     |
| Crash-safe write handling                      | ⬜ Planned  |
| Full-text search index                         | ⬜ Planned  |
| Regex search                                   | ⬜ Planned  |
| Taggable keys / metadata                       | ⬜ Planned  |
| Key TTL support                                | ⬜ Planned  |
| Pub/Sub channels                               | ⬜ Optional |
| Namespaces / Buckets                           | ⬜ Planned  |
| Versioned keys (history)                       | ⬜ Optional |
| mmap-backed read performance                   | ⬜ Optional |
| REST or TCP server                             | ⬜ Optional |
| Web UI dashboard                               | ⬜ Optional |

## 🧱 Architecture Notes

- Core data structure: HashMap<String, Value>

- Future storage: Append-only log + optional snapshots

- Search: Inverted index w/ tokenization

- Memory model: Single-threaded to start, RwLock/async planned later

- Format: Custom binary, zero-dependency for persistence


## 🔬 Example CLI Usage
```bash
# Set a value
nitro set user:42 '{"name": "Alice", "age": 30}'

# Get it
nitro get user:42

# Delete a key
nitro del user:42

# Search values
nitro search "Alice"
```

## 🤝 Contributing
Pull requests welcome. Please file issues or ideas if you want to help design new features or storage formats.

## 📄 License
creator github: @NikoJT
MIT