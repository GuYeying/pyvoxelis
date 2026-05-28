# Voxelis Python Binding
**Fast, compact, high-performance voxel world library for Python**

Powered by the Rust-based [Voxelis](https://github.com/WildPixelGames/voxelis) engine.
Tiny voxels, huge worlds, low memory usage, and blazing fast speed.

---

## ✨ Features
- **Ultra-small voxel size** (4cm resolution) with extremely low memory usage
- **Blazing fast batch operations** (modify 100k+ voxels in milliseconds)
- **Thread-safe & no GC** (deterministic performance)
- **Pythonic API** (matches Rust behavior but feels natural in Python)
- **Read-only wrappers for safe usage**
- **Import / export model support**
- **Bytes encoding / decoding utilities**

---

## 🚀 Install
```bash
pip install pyvoxelis
```

---

## 📌 Quick Example

```python
import pyvoxelis

interner = pyvoxelis.interner.VoxInternerU8.with_memory_budget(1048576)
tree = pyvoxelis.spatial.VoxTreeU8(5)

batch = tree.create_batch()
batch.set(interner, 3, 0, 4, 1)

tree.apply_batch(interner, batch)
assert tree.get(interner, 3, 0, 4)==1
```

---

## 📦 What’s included?

- `VoxTree` – Voxel octree core

- `VoxInterner` – Memory pool \&amp; shared storage

- `Batch` – Fast batched editing

- `ModelExporter` / `ModelImporter` – Static utilities

- Helper functions for bytes, hashmaps, encoding

---

## 🔒 Safety

- All unsafe logic handled in Rust

- Read\-only wrappers where needed

- Fully thread\-safe for i32 types

---

## 📄 License

Same as original Voxelis:
MIT OR Apache\-2\.0

---

## ℹ️ About

This is an **official/independent Python binding** of [Voxelis](https://github.com/WildPixelGames/voxelis)\.
All core performance comes from Rust\.