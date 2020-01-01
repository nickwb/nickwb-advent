use super::{MemoryCell, MemoryPointer};

pub trait Storage {
    fn size(&self) -> MemoryPointer;
    fn get(&self, at: MemoryPointer) -> MemoryCell;
    fn put(&mut self, at: MemoryPointer, value: MemoryCell) -> ();
}

pub type VecStorage = Vec<MemoryCell>;

impl Storage for VecStorage {
    fn size(&self) -> MemoryPointer {
        self.len()
    }
    fn get(&self, at: MemoryPointer) -> MemoryCell {
        self[at]
    }
    fn put(&mut self, at: MemoryPointer, value: MemoryCell) {
        self[at] = value;
    }
}

#[cfg(test)]
pub type MutSliceStorage<'a> = &'a mut [MemoryCell];

#[cfg(test)]
impl<'a> Storage for MutSliceStorage<'a> {
    fn size(&self) -> MemoryPointer {
        self.len()
    }
    fn get(&self, at: MemoryPointer) -> MemoryCell {
        self[at]
    }
    fn put(&mut self, at: MemoryPointer, value: MemoryCell) {
        self[at] = value;
    }
}

#[cfg(test)]
pub fn slice_storage<'a>(s: MutSliceStorage<'a>) -> MutSliceStorage<'a> {
    s
}
