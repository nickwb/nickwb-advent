use super::MemoryCell;
use std::collections::VecDeque;

pub trait InputSource {
    fn next(&mut self) -> Option<MemoryCell>;
}

pub trait OutputSink {
    fn write(&mut self, value: MemoryCell);
}

pub struct BufferInput {
    buf: VecDeque<MemoryCell>,
}

pub struct NoInput;

pub struct RememberLastOutput {
    value: Option<MemoryCell>,
}

impl BufferInput {
    pub fn new(capacity: usize) -> BufferInput {
        BufferInput {
            buf: VecDeque::with_capacity(capacity),
        }
    }

    pub fn queue(&mut self, value: MemoryCell) {
        self.buf.push_back(value);
    }

    pub fn queue_many(&mut self, values: &[MemoryCell]) {
        for v in values {
            self.queue(*v);
        }
    }
}

impl InputSource for BufferInput {
    fn next(&mut self) -> Option<MemoryCell> {
        self.buf.pop_front()
    }
}

impl InputSource for NoInput {
    fn next(&mut self) -> Option<MemoryCell> {
        None
    }
}

impl RememberLastOutput {
    pub fn new() -> RememberLastOutput {
        RememberLastOutput { value: None }
    }

    pub fn last(&self) -> Option<MemoryCell> {
        self.value
    }
}

impl OutputSink for RememberLastOutput {
    fn write(&mut self, value: MemoryCell) {
        self.value = Some(value);
    }
}
