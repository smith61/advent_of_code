
use std::cmp;

#[derive(Clone, Copy)]
struct AvlTreeVecNode {
    parent_node: Option<usize>,
    left_node: Option<usize>,
    right_node: Option<usize>,

    height: isize,
    relative_position: usize
}

impl AvlTreeVecNode {

    fn new() -> Self {
        Self {
            parent_node: None,
            left_node: None,
            right_node: None,

            height: 0,
            relative_position: usize::MAX
        }
    }

}

struct AvlTreeVec {
    nodes: Vec<AvlTreeVecNode>,
    root_index: Option<usize>
}

impl AvlTreeVec {

    fn new(size: usize) -> Self {
        Self {
            nodes: vec![AvlTreeVecNode::new(); size],
            root_index: None
        }
    }

    fn get_node_at(&self, mut position: usize) -> usize {
        let mut current_node_index = self.root_index.unwrap();
        loop {
            if self.nodes[current_node_index].relative_position == position {
                return current_node_index;
            }

            current_node_index = if self.nodes[current_node_index].relative_position < position {
                position -= self.nodes[current_node_index].relative_position + 1;
                self.nodes[current_node_index].right_node.unwrap()

            } else {
                self.nodes[current_node_index].left_node.unwrap()
            };
        }
    }

    fn get_node_position(&self, node_index: usize) -> usize {
        let mut position = self.nodes[node_index].relative_position;

        let mut previous_node_index = node_index;
        let mut parent_node = self.nodes[node_index].parent_node;
        while let Some(parent_node_index) = parent_node {
            if self.nodes[parent_node_index].right_node == Some(previous_node_index) {
                position += self.nodes[parent_node_index].relative_position + 1;
            }

            previous_node_index = parent_node_index;
            parent_node = self.nodes[parent_node_index].parent_node;
        }

        position
    }

    fn insert_node(&mut self, node_index: usize, position: usize) {
        let old_root_index = self.root_index.take();
        let new_root_index =
            self.insert_node_recurse(
                None,
                old_root_index,
                node_index,
                position);
        
        self.root_index = Some(new_root_index);
    }

    fn remove_node_at(&mut self, position: usize) {
        let old_root_index = self.root_index.take();
        let (new_root_index, _) =
            self.remove_node_at_recurse(
                old_root_index,
                position);
        
        self.root_index = new_root_index;
    }

    fn insert_node_recurse(&mut self, parent_node_index: Option<usize>, current_node_link: Option<usize>, node_index: usize, mut position: usize) -> usize {
        if let Some(current_node_index) = current_node_link {
            if self.nodes[current_node_index].relative_position < position {
                position -= self.nodes[current_node_index].relative_position + 1;
                let old_right = self.nodes[current_node_index].right_node.take();
                let new_right =
                    self.insert_node_recurse(
                        Some(current_node_index),
                        old_right,
                        node_index,
                        position);
                
                self.nodes[current_node_index].right_node = Some(new_right);

            } else {
                let old_left = self.nodes[current_node_index].left_node.take();
                let new_left =
                    self.insert_node_recurse(
                        Some(current_node_index),
                        old_left,
                        node_index,
                        position);
                
                self.nodes[current_node_index].relative_position += 1;
                self.nodes[current_node_index].left_node = Some(new_left);
            }

            self.balance_node(current_node_index)

        } else {
            assert_eq!(position, 0);
            assert!(self.nodes[node_index].left_node.is_none());
            assert!(self.nodes[node_index].right_node.is_none());
            assert!(self.nodes[node_index].parent_node.is_none());

            self.nodes[node_index].parent_node = parent_node_index;
            self.nodes[node_index].relative_position = 0;
            node_index
        }
    }

    fn remove_node_at_recurse(&mut self, current_node_link: Option<usize>, mut position: usize) -> (Option<usize>, usize) {
        if let Some(current_node_index) = current_node_link {
            if self.nodes[current_node_index].relative_position == position {
                let replacement_node = match (self.nodes[current_node_index].left_node.take(), self.nodes[current_node_index].right_node.take()) {
                    (None, None) => {
                        None
                    },
                    (Some(left_node_index), None) => {
                        self.nodes[left_node_index].parent_node = self.nodes[current_node_index].parent_node;
                        Some(left_node_index)
                    },
                    (None, Some(right_node_index)) => {
                        self.nodes[right_node_index].parent_node = self.nodes[current_node_index].parent_node;
                        Some(right_node_index)
                    },
                    (Some(left_node_index), Some(right_node_index)) => {
                        let (new_right_node_index, successor_index) = 
                            self.remove_node_at_recurse(Some(right_node_index), 0);

                        self.nodes[successor_index].relative_position = self.nodes[current_node_index].relative_position;
                        self.nodes[successor_index].parent_node = self.nodes[current_node_index].parent_node;
                        self.nodes[successor_index].left_node = Some(left_node_index);
                        self.nodes[successor_index].right_node = new_right_node_index;

                        self.nodes[left_node_index].parent_node = Some(successor_index);
                        if let Some(new_right_node_index) = new_right_node_index {
                            self.nodes[new_right_node_index].parent_node = Some(successor_index);
                        }

                        Some(successor_index)
                    }
                };

                let replacement_node = replacement_node.map(|n| self.balance_node(n));

                self.nodes[current_node_index].parent_node = None;
                self.nodes[current_node_index].height = 0;
                self.nodes[current_node_index].relative_position = usize::MAX;
                return (replacement_node, current_node_index);

            }
            
            let removed_node_index = if self.nodes[current_node_index].relative_position < position {
                position -= self.nodes[current_node_index].relative_position + 1;
                let old_right = self.nodes[current_node_index].right_node.take();
                let (new_right, removed_node_index) =
                    self.remove_node_at_recurse(
                        old_right,
                        position);
                
                self.nodes[current_node_index].right_node = new_right;
                removed_node_index

            } else {
                let old_left = self.nodes[current_node_index].left_node.take();
                let (new_left, removed_node_index) =
                    self.remove_node_at_recurse(
                        old_left,
                        position);
                
                self.nodes[current_node_index].relative_position -= 1;
                self.nodes[current_node_index].left_node = new_left;
                removed_node_index
            };

            (Some(self.balance_node(current_node_index)), removed_node_index)

        } else {
            unreachable!();
        }
    }

    fn balance_node(&mut self, node_index: usize) -> usize {
        let node_balance = self.get_node_balance(node_index);
        if node_balance <= -2 {
            let left_node_index = self.nodes[node_index].left_node.take().unwrap();
            let left_node_balance = self.get_node_balance(left_node_index);
            let new_left_node_index = if left_node_balance > 0 {
                self.rotate_left(left_node_index)

            } else {
                left_node_index
            };

            self.nodes[node_index].left_node = Some(new_left_node_index);
            self.rotate_right(node_index)

        } else if node_balance >= 2 {
            let right_node_index = self.nodes[node_index].right_node.take().unwrap();
            let right_node_balance = self.get_node_balance(right_node_index);
            let new_right_node_index = if right_node_balance < 0 {
                self.rotate_right(right_node_index)

            } else {
                right_node_index
            };

            self.nodes[node_index].right_node = Some(new_right_node_index);
            self.rotate_left(node_index)

        } else {
            self.fixup_node_height(node_index);
            node_index
        }
    }

    fn fixup_node_height(&mut self, node_index: usize) {
        let left_node_height = self.nodes[node_index].left_node.map(|n| self.nodes[n].height).unwrap_or(0);
        let right_node_height = self.nodes[node_index].right_node.map(|n| self.nodes[n].height).unwrap_or(0);

        self.nodes[node_index].height = cmp::max(left_node_height, right_node_height) + 1;
    }

    fn get_node_balance(&self, node_index: usize) -> isize {
        let left_node_height = self.nodes[node_index].left_node.map(|n| self.nodes[n].height).unwrap_or(0);
        let right_node_height = self.nodes[node_index].right_node.map(|n| self.nodes[n].height).unwrap_or(0);

        right_node_height - left_node_height
    }

    fn rotate_left(&mut self, prev_root_index: usize) -> usize {
        let root_parent_index = self.nodes[prev_root_index].parent_node.take();

        let prev_right_index = self.nodes[prev_root_index].right_node.take().unwrap();
        let prev_right_left_index = self.nodes[prev_right_index].left_node.take();

        //
        // Update the old root to point to its new right child and parent.
        //

        {
            self.nodes[prev_root_index].parent_node = Some(prev_right_index);
            self.nodes[prev_root_index].right_node = prev_right_left_index;
            self.fixup_node_height(prev_root_index);
        }

        //
        // Update the new root to point to its new left child and parent.
        //

        {
            self.nodes[prev_right_index].parent_node = root_parent_index;
            self.nodes[prev_right_index].left_node = Some(prev_root_index);
            self.fixup_node_height(prev_right_index);

            //
            // The relative position of this node needs to be updated as new nodes
            // have been added to the left of this node.
            //

            self.nodes[prev_right_index].relative_position +=
                self.nodes[prev_root_index].relative_position + 1;
        }

        if let Some(prev_right_left_index) = prev_right_left_index {
            self.nodes[prev_right_left_index].parent_node = Some(prev_root_index);
        }

        prev_right_index
    }

    fn rotate_right(&mut self, prev_root_index: usize) -> usize {
        let root_parent_index = self.nodes[prev_root_index].parent_node.take();

        let prev_left_index = self.nodes[prev_root_index].left_node.take().unwrap();
        let prev_left_right_index = self.nodes[prev_left_index].right_node.take();

        //
        // Update the old root to point to its new left child and parent.
        //

        {
            self.nodes[prev_root_index].parent_node = Some(prev_left_index);
            self.nodes[prev_root_index].left_node = prev_left_right_index;
            self.fixup_node_height(prev_root_index);

            //
            // The relative position of this node needs to be updated as new nodes
            // have been removed from the left of this node.
            //

            self.nodes[prev_root_index].relative_position -=
                self.nodes[prev_left_index].relative_position + 1;
        }

        //
        // Update the new root to point to its new right child and parent.
        //

        {
            self.nodes[prev_left_index].parent_node = root_parent_index;
            self.nodes[prev_left_index].right_node = Some(prev_root_index);
            self.fixup_node_height(prev_left_index);
        }

        if let Some(prev_left_right_index) = prev_left_right_index {
            self.nodes[prev_left_right_index].parent_node = Some(prev_root_index);
        }

        prev_left_index
    }

}

fn run_simulation<const DECRYPTION_KEY: i64, const SHUFFLE_COUNT: u32>(input: &str) -> i64 {
    let nums =
        input
        .lines()
        .map(|line| i64::from_str_radix(line, 10).unwrap())
        .map(|n| n * DECRYPTION_KEY)
        .collect::<Vec<_>>();

    let mut tree = AvlTreeVec::new(nums.len());
    for index in 0..nums.len() {
        tree.insert_node(index, index);
    }

    for _ in 0..SHUFFLE_COUNT {
        for (sort_index, &num) in nums.iter().enumerate() {
            let num_pos = tree.get_node_position(sort_index);
            tree.remove_node_at(num_pos);
            let new_num_pos = ((num_pos as i64) + num).rem_euclid((nums.len() - 1) as i64) as usize;
            tree.insert_node(sort_index, new_num_pos);
        }
    }

    let zero_num_idx = nums.iter().position(|v| *v == 0).unwrap();
    let zero_position = tree.get_node_position(zero_num_idx);
    let mut val = 0;
    for index in [1000, 2000, 3000] {
        let r_index = (zero_position + index) % nums.len();
        val += nums[tree.get_node_at(r_index)];
    }

    val
}

pub fn part1(input: &str) -> i64 {
    run_simulation::<1, 1>(input)
}

pub fn part2(input: &str) -> i64 {
    run_simulation::<811589153, 10>(input)
}
