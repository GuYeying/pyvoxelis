from pyvoxelis.world import VoxChunk
from pyvoxelis.interner import VoxInternerI32  # 你必须传 interner


class Mesh:
    def __init__(self, vertices: list, normals: list, indices: list):
        self.vertices = vertices
        self.normals = normals
        self.indices = indices

    @classmethod
    def generate_mesh(cls, chunk: VoxChunk, interner: VoxInternerI32, lod_level: int = 0):
        """
        从 VoxChunk 生成网格
        :param chunk: 区块
        :param interner: 必须传入 interner（Rust 要求）
        :param lod_level: LOD 等级，默认 0
        """
        # 空块直接返回 None
        if chunk.is_empty:
            return None

        # ✅ 直接调用 Rust 暴露的方法，一键获取顶点/法线/索引
        vertices, normals, indices = chunk.generate_mesh(
            interner,
            lod_level,
            0.0, 0.0, 0.0  # offset 偏移，默认 0
        )

        return cls(vertices, normals, indices)