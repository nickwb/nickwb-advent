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

pub struct BufferOutput {
    values: VecDeque<MemoryCell>,
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

impl BufferOutput {
    pub fn new(capacity: usize) -> BufferOutput {
        BufferOutput {
            values: VecDeque::with_capacity(capacity),
        }
    }

    pub fn pop(&mut self) -> Option<MemoryCell> {
        self.values.pop_front()
    }

    pub fn last(&self) -> Option<MemoryCell> {
        Some(*(self.values.iter().last()?))
    }

    pub fn pop_all(&mut self) -> Vec<MemoryCell> {
        let mut result: Vec<MemoryCell> = Vec::with_capacity(self.values.len());
        while let Some(x) = self.pop() {
            result.push(x);
        }
        result
    }
}

impl OutputSink for BufferOutput {
    fn write(&mut self, value: MemoryCell) {
        self.values.push_back(value);
    }
}
