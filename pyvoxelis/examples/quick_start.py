import pyvoxelis


interner = pyvoxelis.interner.VoxInternerU8.with_memory_budget(1048576)
tree = pyvoxelis.spatial.VoxTreeU8(5)

batch = tree.create_batch()
batch.set(interner, 3, 0, 4, 1)

tree.apply_batch(interner, batch)
assert tree.get(interner, 3, 0, 4)==1
