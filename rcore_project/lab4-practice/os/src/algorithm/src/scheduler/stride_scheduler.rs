//! Stride Scheduling的调度器 [`StrideSchduler`]

use super::Scheduler;
use alloc::vec::Vec;

/// 将线程和调度信息打包
struct StrideThread<ThreadType: Clone + Eq> {
    stride: usize,
    BigStride: usize,
    priority: usize,  // pass = BigStride / priority
    /// 线程数据
    pub thread: ThreadType,
}

/// 采用 Stride Scheduling 算法的线程调度器
pub struct StrideScheduler<ThreadType: Clone + Eq> {
    /// 带有调度信息的线程池
    pool: Vec::<StrideThread<ThreadType>>,
}

/// `Default` 创建一个空的调度器
impl<ThreadType: Clone + Eq> Default for StrideScheduler<ThreadType> {
    fn default() -> Self {
        Self {
            pool:  Vec::<StrideThread<ThreadType>>::new(),
        }
    }
}

impl<ThreadType: Clone + Eq> Scheduler<ThreadType> for StrideScheduler<ThreadType> {
    type Priority = ();

    fn add_thread(&mut self, thread: ThreadType) {
        self.pool.push(StrideThread {
            stride: 0,
            BigStride: 23333,
            priority: 100,    // 暂时定为常数,因为ThreadType时抽象的，无法使用线程中保存的优先级
            thread,
        })
    }
    fn get_next(&mut self) -> Option<ThreadType> {
        // 遍历线程池，返回stride最小者
        if let Some(best) = self.pool.iter_mut().min_by(|x, y| {
            x.stride.cmp(& y.stride)
        }) {
            best.stride += best.BigStride / best.priority;
            Some(best.thread.clone())
        } else {
            None
        }
    }
    fn remove_thread(&mut self, thread: &ThreadType) {
        // 移除相应的线程并且确认恰移除一个线程
        let mut removed = self.pool.drain_filter(|t| t.thread == *thread);
        assert!(removed.next().is_some() && removed.next().is_none());
    }
    fn set_priority(&mut self, _thread: ThreadType, _priority: ()) {}
}
