from __future__ import annotations
from typing import Optional, List

from ..core import BlockId, TraversalDepth
from ..interner import VoxInternerU16, VoxInternerU32, VoxInternerU8

class BlockId:...
class TraversalDepth:...

class VoxInternerU16:...
class VoxInternerU32:...
class VoxInternerU8:...

# -----------------------------------------------------------------------------
# Common 工具类（静态方法）
# -----------------------------------------------------------------------------
class Common:
    @staticmethod
    def child_index(x: int, y: int, z: int, depth: TraversalDepth) -> int: ...

    @staticmethod
    def child_index2(x: int, y: int, z: int, current: int, max: int) -> int: ...

    @staticmethod
    def encode_child_index_path(x: int, y: int, z: int) -> int: ...

    # ------------------------------
    # U8 工具
    # ------------------------------
    @staticmethod
    def get_at_depth_u8(
        interner: VoxInternerU8,
        node_id: BlockId,
        x: int, y: int, z: int,
        depth: TraversalDepth
    ) -> Optional[int]: ...

    @staticmethod
    def to_vec_u8(
        interner: VoxInternerU8,
        root_id: BlockId,
        max_depth: int
    ) -> List[int]: ...

    @staticmethod
    def dump_structure_u8(
        interner: VoxInternerU8,
        root_id: BlockId,
        max_depth: int
    ) -> None: ...

    @staticmethod
    def dump_root_u8(
        interner: VoxInternerU8,
        root_id: BlockId
    ) -> None: ...

    @staticmethod
    def dump_statistics_u8(
        interner: VoxInternerU8,
        root_id: BlockId
    ) -> None: ...

    # ------------------------------
    # U16 工具
    # ------------------------------
    @staticmethod
    def get_at_depth_u16(
        interner: VoxInternerU16,
        node_id: BlockId,
        x: int, y: int, z: int,
        depth: TraversalDepth
    ) -> Optional[int]: ...

    @staticmethod
    def to_vec_u16(
        interner: VoxInternerU16,
        root_id: BlockId,
        max_depth: int
    ) -> List[int]: ...

    @staticmethod
    def dump_structure_u16(
        interner: VoxInternerU16,
        root_id: BlockId,
        max_depth: int
    ) -> None: ...

    @staticmethod
    def dump_root_u16(
        interner: VoxInternerU16,
        root_id: BlockId
    ) -> None: ...

    @staticmethod
    def dump_statistics_u16(
        interner: VoxInternerU16,
        root_id: BlockId
    ) -> None: ...

    # ------------------------------
    # U32 工具
    # ------------------------------
    @staticmethod
    def get_at_depth_u32(
        interner: VoxInternerU32,
        node_id: BlockId,
        x: int, y: int, z: int,
        depth: TraversalDepth
    ) -> Optional[int]: ...

    @staticmethod
    def to_vec_u32(
        interner: VoxInternerU32,
        root_id: BlockId,
        max_depth: int
    ) -> List[int]: ...

    @staticmethod
    def dump_structure_u32(
        interner: VoxInternerU32,
        root_id: BlockId,
        max_depth: int
    ) -> None: ...

    @staticmethod
    def dump_root_u32(
        interner: VoxInternerU32,
        root_id: BlockId
    ) -> None: ...

    @staticmethod
    def dump_statistics_u32(
        interner: VoxInternerU32,
        root_id: BlockId
    ) -> None: ...




__all__ = [
    "Common",
]