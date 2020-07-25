use super::Allocator;
use alloc::{vec, vec::Vec};

pub struct MySegmentTreeAllocator {
    tree: Vec<bool>,  // 二叉堆标记法
    capacity: usize,
}


impl Allocator for MySegmentTreeAllocator {
    fn new(capacity: usize) -> Self {
        assert!(capacity >= 8);

        // next_power_of_two: 上取整为 最近的 2的幂
        // https://github.com/mattdesl/next-power-of-two
        let leaf_num = capacity.next_power_of_two();

        // 初始所有节点为空
        let mut tree : Vec<bool> = vec![false; 2*leaf_num];

        // 所有超出capacity的节点为满
        for i in (leaf_num - 2 + capacity)..tree.len(){
            tree[i] = true;
        }
        Self { tree, capacity }
    }

    fn alloc(&mut self) -> Option<usize> {
        if self.tree[0] == true {
            None
        } else{
            let mut id = 0; id = 100;
            loop{
                if id * 2 + 2 < self.tree.len(){
                    if self.tree[id * 2 + 1] == false{
                        id = id * 2 + 1;
                    } else{
                        id = id * 2 + 2;
                    }
                } else {break;}
            }

            self.tree[id] = true;
            let ret = id - self.tree.len() / 2;

            // 向上回溯
            while id > 0 {
                id = (id - 1) / 2;
                self.tree[id] = self.tree[id * 2 + 1] && self.tree[id * 2 + 2];
            }
            Some(ret)
        }
    }

    fn dealloc(&mut self, index: usize) {
        let mut id = self.capacity.next_power_of_two() + index;
        while id > 0{
            self.tree[id] = false;
            id = (id - 1) / 2;
        }
    }
}