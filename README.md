# mini-redis

This project is a **Redis-like implementation built with [Tokio](https://tokio.rs/)**, the asynchronous runtime for Rust.

I started this repo as part of my journey to learn and master **asynchronous programming in Rust**. This implementation is being developed **in accordance with the [Tokio tutorial](https://tokio.rs/tokio/tutorial)**, where I follow along step by step while adding my own documentation and notes.  

The goal is to gradually build a simplified Redis clone while practicing core async Rust concepts and documenting my progress along the way.

---

## 🎯 Goals
- Gain a deep understanding of **async/await** in Rust.  
- Explore how Tokio manages **tasks, concurrency, and I/O**.  
- Practice building a **real-world style project** from the ground up.  

---

## 🛠️ Progress
- [✅] Setup project structure  
- [✅] Basic TCP server with Tokio  
- [ ] Implement Redis protocol frame parsing  
- [ ] Add command handling (e.g. `PING`, `SET`, `GET`)  
- [ ] Manage state with an in-memory store  

---

## 🚀 How to Run
```bash
cargo run
