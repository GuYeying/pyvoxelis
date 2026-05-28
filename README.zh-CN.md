
# Voxelis Python 绑定库

**Python 高性能、轻量化、紧凑型体素世界开发库**

基于 Rust 开源引擎[Voxelis](https://github.com/WildPixelGames/voxelis) 深度绑定开发。

微型体素精度、超大世界支撑、极低内存占用、极致运行速度。

---

## ✨ 核心特性

- **超高精度体素**（4厘米物理精度），内存占用极致低廉

- **极速批量操作**（毫秒级完成十万级体素修改）

- **线程安全、无垃圾回收**（性能稳定可预测）

- **原生 Python 风格 API**（完全对齐 Rust 核心逻辑，适配 Python 开发习惯）

- **安全只读封装**，规避非法外部修改

- **支持模型导入导出**

- **内置字节编解码、哈希映射转换等工具方法**

---

## 🚀 安装

```bash
pip install pyvoxelis
```

---

## 🚀 开发
```bash
cargo add pyvoxelis
```

---

## 📌 快速上手示例

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

## 📦 库功能包含

- `VoxTree` — 体素八叉树核心引擎

- `VoxInterner` — 共享内存池与存储管理器

- `Batch` — 高性能批量体素编辑工具

- `ModelExporter` / `ModelImporter` — 模型导入导出静态工具类

- 内置字节处理、哈希映射转换、数据编解码通用工具函数

---

## 🔒 安全机制

- 所有底层不安全逻辑均由 Rust 托管处理

- 核心结构按需提供只读封装，保障数据安全

- i32 专用版本完全支持多线程安全操作

---

## 📄 开源许可证

与原 Voxelis 引擎保持一致：
MIT OR Apache\-2\.0

---

## ℹ️ 项目说明

本项目是[Voxelis](https://github.com/WildPixelGames/voxelis) 引擎的独立 Python 绑定库。

所有核心高性能逻辑均由原生 Rust 的 voxelis 引擎提供支撑。
