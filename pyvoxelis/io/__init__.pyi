from __future__ import annotations
from typing import ClassVar, Optional, List, Tuple

# -----------------------------------------------------------------------------
# IO 常量（无法实例化）
# -----------------------------------------------------------------------------
class Constants:
    VTM_VERSION: ClassVar[int]
    VTM_MAGIC: ClassVar[bytes]
    VTC_MAGIC: ClassVar[bytes]
    RESERVED_1: ClassVar[int]
    RESERVED_2: ClassVar[int]

    def __new__(cls) -> None: ...

# -----------------------------------------------------------------------------
# 导出工具类（静态方法）
# -----------------------------------------------------------------------------
class Export:
    @staticmethod
    def export_to_obj(
        name: str,
        path: str,
        model: VoxModel,
        lod_level: int
    ) -> None: ...

    @staticmethod
    def export_to_vtm(
        name: str,
        path: str,
        model: VoxModel
    ) -> None: ...

# -----------------------------------------------------------------------------
# IO 标记位（无法实例化）
# -----------------------------------------------------------------------------
class Flags:
    NONE: ClassVar[int]
    COMPRESSED: ClassVar[int]
    DEFAULT: ClassVar[int]

    def __new__(cls) -> None: ...

# -----------------------------------------------------------------------------
# 导入工具类（静态方法）
# -----------------------------------------------------------------------------
class Import:
    @staticmethod
    def import_from_vtm(
        path: str,
        memory_budget: int,
        target_chunk_size: Optional[float]
    ) -> VoxModel: ...

# -----------------------------------------------------------------------------
# OBJ 模型读取
# -----------------------------------------------------------------------------
class Obj:
    @staticmethod
    def parse(path: str) -> Obj: ...

    @property
    def vertices(self) -> List[Tuple[float, float, float]]: ...

    @property
    def faces(self) -> List[Tuple[int, int, int]]: ...

    @property
    def aabb(self) -> Tuple[Tuple[float, float, float], Tuple[float, float, float]]: ...

    @property
    def size(self) -> Tuple[float, float, float]: ...

    def __repr__(self) -> str: ...

# -----------------------------------------------------------------------------
# Varint 编解码工具（静态方法）
# -----------------------------------------------------------------------------
class Varint:
    @staticmethod
    def encode_u32(value: int) -> bytes: ...

    @staticmethod
    def encode(value: int) -> bytes: ...

    @staticmethod
    def decode_u32(data: bytes) -> Optional[int]: ...

    @staticmethod
    def decode(data: bytes) -> Optional[int]: ...

# -----------------------------------------------------------------------------
# 依赖类型占位（保证不报错）
# -----------------------------------------------------------------------------
class VoxModel: ...

__all__ = [
    "Constants",
    "Export",
    "Flags",
    "Import",
    "Obj",
    "Varint",
]