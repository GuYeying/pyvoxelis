from __future__ import annotations
from typing import Optional, List, Tuple, Dict


class VoxInternerI32:...

class BatchI32:...

class MaxDepth:...

# -----------------------------------------------------------------------------
# VoxChunk
# -----------------------------------------------------------------------------
class VoxChunk:
    def __new__(cls) -> None:
        """Use model.get_or_create_chunk() instead"""
        ...

    @property
    def position(self) -> Tuple[int, int, int]: ...

    @property
    def world_position(self) -> Tuple[float, float, float]: ...

    @property
    def chunk_size(self) -> float: ...

    @property
    def is_empty(self) -> bool: ...

    @property
    def is_leaf(self) -> bool: ...

    @property
    def is_dirty(self) -> bool: ...

    @property
    def root_id(self) -> int: ...

    def voxel_size(self, lod_level: int) -> float: ...

    def max_depth(self, lod_level: int) -> MaxDepth: ...

    def voxels_per_axis(self, lod_level: int) -> int: ...

    def set_position(self, x: int, y: int, z: int) -> None: ...

    def mark_dirty(self) -> None: ...

    def clear_dirty(self) -> None: ...

    # -------------------------------------------------------------------------
    # 体素读写
    # -------------------------------------------------------------------------
    def get(self, interner: VoxInternerI32, x: int, y: int, z: int) -> Optional[int]: ...

    def set(self, interner: VoxInternerI32, x: int, y: int, z: int, value: int) -> bool: ...

    def fill(self, interner: VoxInternerI32, value: int) -> None: ...

    def clear(self, interner: VoxInternerI32) -> None: ...

    # -------------------------------------------------------------------------
    # Batch
    # -------------------------------------------------------------------------
    def create_batch(self) -> BatchI32: ...

    def apply_batch(self, interner: VoxInternerI32, batch: BatchI32) -> bool: ...

    # -------------------------------------------------------------------------
    # Test data
    # -------------------------------------------------------------------------
    def generate_test_data(self, interner: VoxInternerI32) -> None: ...

    def generate_test_sphere(
        self,
        interner: VoxInternerI32,
        cx: int,
        cy: int,
        cz: int,
        radius: int,
        value: int
    ) -> None: ...

    # -------------------------------------------------------------------------
    # Mesh
    # -------------------------------------------------------------------------
    def generate_mesh(
        self,
        interner: VoxInternerI32,
        lod_level: int,
        ox: float,
        oy: float,
        oz: float
    ) -> Tuple[
        List[List[float]],
        List[List[float]],
        List[int]
    ]: ...

    # -------------------------------------------------------------------------
    # Static constructors
    # -------------------------------------------------------------------------
    @staticmethod
    def with_position(
        chunk_size: float,
        max_depth: int,
        x: int,
        y: int,
        z: int
    ) -> VoxChunk: ...

    @staticmethod
    def deserialize(
        interner: VoxInternerI32,
        leaf_patterns: Dict[int, Tuple[int, int]],
        patterns: Dict[int, Tuple[int, List[int], int]],
        data: bytes,
        chunk_size: float,
        max_depth: int
    ) -> VoxChunk: ...

    # -------------------------------------------------------------------------
    # Serialize
    # -------------------------------------------------------------------------
    def serialize(self, id_map: Dict[int, int]) -> bytes: ...


# -----------------------------------------------------------------------------
# VoxModel
# -----------------------------------------------------------------------------
class VoxModel:
    @staticmethod
    def empty(max_depth: int, chunk_world_size: float, memory_budget: int) -> VoxModel: ...

    @staticmethod
    def new(max_depth: int, chunk_world_size: float, memory_budget: int) -> VoxModel: ...

    @staticmethod
    def with_dimensions(
        max_depth: int,
        chunk_world_size: float,
        world_bounds: Tuple[int, int, int],
        memory_budget: int
    ) -> VoxModel: ...

    def get_or_create_chunk(self, x: int, y: int, z: int) -> VoxChunk: ...

    def clear(self) -> None: ...

    def resize(self, bounds: Tuple[int, int, int]) -> None: ...

    def get_bounds_size(self) -> int: ...

    def is_position_in_bounds(self, pos: Tuple[int, int, int]) -> bool: ...

    @property
    def max_depth(self) -> int: ...

    @property
    def voxels_per_axis(self) -> int: ...

    @property
    def world_bounds(self) -> Tuple[int, int, int]: ...

    def get_interner(self) -> VoxInternerI32: ...

    def serialize(self) -> bytes: ...

    def deserialize(self, data: bytes) -> None: ...


# -----------------------------------------------------------------------------
# VoxWorld
# -----------------------------------------------------------------------------
class VoxWorld:
    @staticmethod
    def create() -> VoxWorld: ...

    @staticmethod
    def create_with_size(x: int, y: int, z: int) -> VoxWorld: ...

    @property
    def chunks_size(self) -> Tuple[int, int, int]: ...

    @property
    def chunks_len(self) -> int: ...

    def clear(self) -> None: ...

    def resize(self, x: int, y: int, z: int) -> None: ...

    def serialize(self) -> bytes: ...

    def __repr__(self) -> str: ...


__all__ = [
    "VoxChunk",
    "VoxModel",
    "VoxWorld",
]