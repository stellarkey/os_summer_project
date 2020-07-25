//! 提供队列结构实现的分配器 [`QueueAllcator`]

use super::Allocator;
use alloc::{vec, vec::Vec};

/// 使用队列实现分配器
pub struct QueueAllcator {
    list: Vec<(usize, usize)>,
}

impl Allocator for QueueAllcator {
    fn new(capacity: usize) -> Self {
        Self {
            list: vec![(0, capacity)],
        }
    }

    // 队列从 左边 出队，从 右边 进队
    fn alloc(&mut self) -> Option<usize> {
        if let Some((start, end)) = Some(self.list[0]) {
            self.list.remove(0);
            if end - start > 1 {
                self.list.push((start + 1, end));
            }
            Some(start)
        } else {
            None
        }
    }

    fn dealloc(&mut self, index: usize) {
        self.list.push((index, index + 1));
    }
}
