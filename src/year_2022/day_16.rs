
use std::cmp::{self, Reverse};
use std::collections::BinaryHeap;

use fxhash::FxHashMap;

use ndarray::{Array2, Array3};

struct SimulationState<const PART_COUNT: u8, const STEP_COUNT: u16> {
    node_count: u8,
    node_distances: Array2<u8>,
    flow_rates: Vec<u16>,
    max_flows: Vec<u16>
}

impl<const PART_COUNT: u8, const STEP_COUNT: u16> SimulationState<PART_COUNT, STEP_COUNT> {

    fn new(input: &str) -> Self {
        let (node_distances, flow_rates, node_count) = Self::parse_input(input);
        assert!(node_count <= 16);

        Self {
            node_count,
            node_distances,
            flow_rates,
            max_flows: vec![0; 1 << node_count]
        }
    }

    fn build_max_flows_table(&mut self) {
        const SEEN_BIT: u16 = 1 << (u16::BITS - 1);
        
        let mut dp_cache =
            Array3::<u16>::zeros((1 << self.node_count, self.node_count as usize, STEP_COUNT as usize));

        let mut queue = BinaryHeap::<Reverse<(u16, usize, usize)>>::new();
        dp_cache[[0, 0, 0]] = SEEN_BIT;
        queue.push(Reverse((0, 0, 0)));
        while let Some(Reverse((current_time, current_node, toggled_valves))) = queue.pop() {
            let rem_time = STEP_COUNT - current_time;
            let mut new_flow = dp_cache[[toggled_valves, current_node, current_time as usize]];
            new_flow += self.flow_rates[current_node] * rem_time;
            new_flow |= SEEN_BIT;
            for next_node in 1..self.node_count as usize {
                if toggled_valves & (1 << next_node) != 0 {
                    continue;
                }

                let distance = (self.node_distances[(current_node, next_node)] + 1) as u16;
                if distance >= rem_time {
                    continue;
                }

                let new_toggled_valves = toggled_valves | (1 << next_node);
                let entry =
                    &mut dp_cache[[new_toggled_valves, next_node, (current_time + distance) as usize]];

                if (*entry & SEEN_BIT) == 0 {
                    queue.push(Reverse((current_time + distance, next_node, new_toggled_valves)));
                }

                *entry = cmp::max(*entry, new_flow);
            }

            self.max_flows[toggled_valves] = cmp::max(self.max_flows[toggled_valves], new_flow);
        }

        for index in 0..self.max_flows.len() {
            self.max_flows[index] &= !SEEN_BIT;
        }
    }

    fn run_simulation(&mut self) -> u64 {
        self.build_max_flows_table();
        if PART_COUNT == 1 {
            *self.max_flows.iter().max().unwrap() as u64

        } else {
            assert_eq!(PART_COUNT, 2);
            
            let mut dp_cache = vec![0; 1 << self.node_count];
            for toggled_valves in 0..(1 << self.node_count) {
                let mut max_flow = self.max_flows[toggled_valves];
                for valve in 0..self.node_count {
                    if toggled_valves & (1 << valve) == 0 {
                        continue;
                    }

                    max_flow = max_flow.max(dp_cache[toggled_valves & !(1 << valve)]);
                }

                dp_cache[toggled_valves] = max_flow;
            }

            let mut max_flow = 0;
            let mask = (1 << self.node_count) - 1;
            for toggled_valves in 0..(1 << self.node_count) {
                let non_toggled_valves = !toggled_valves & mask;
                let non_toggled_potential = dp_cache[non_toggled_valves];
                max_flow = max_flow.max(self.max_flows[toggled_valves] + non_toggled_potential);
            }

            max_flow as u64
        }
    }

    fn parse_node_id(bytes: &[u8], index: &mut usize) -> u16 {
        let hb = bytes[*index + 0] as u16;
        let lb = bytes[*index + 1] as u16;
        *index += 2;
        (hb << 8) | lb
    }

    fn parse_input(input: &str) -> (Array2<u8>, Vec<u16>, u8) {
        let start_node_id = Self::parse_node_id(&[b'A', b'A'], &mut 0);
    
        let node_count = input.lines().count();
        let mut next_flow_node_idx = 1;
        let mut next_blocked_node_idx = node_count - 1;
    
        let mut node_ids = FxHashMap::default();
        let mut node_edges = FxHashMap::default();
        let mut flow_rates = vec![0; node_count];
    
        for line in input.lines() {
            let bytes = line.as_bytes();
            let mut index = "Valve ".len();
            let node_id = Self::parse_node_id(bytes, &mut index);
    
            index += " has flow rate=".len();
            let flow_rate = {
                let mut val = 0;
                while bytes[index] != b';' {
                    val = (val * 10) + ((bytes[index] - b'0') as u16);
                    index += 1;
                }
    
                val
            };
    
            index += "; tunnels lead to valve".len();
            if bytes[index] == b's' {
                index += 2;
    
            } else {
                index += 1;
            }
    
            let mut edges = Vec::new();
            while index < bytes.len() {
                edges.push(Self::parse_node_id(bytes, &mut index));
                index += 2;
            }
    
            let node_index = if node_id == start_node_id {
                0
    
            } else if flow_rate == 0 {
                let index = next_blocked_node_idx;
                next_blocked_node_idx -= 1;
                index
    
            } else {
                let index = next_flow_node_idx;
                next_flow_node_idx += 1;
                index
            };
    
            node_ids.insert(node_id, node_index);
            node_edges.insert(node_index, edges);
            flow_rates[node_index] = flow_rate;
        }
    
        let mut node_distances = Array2::from_elem((node_count, node_count), u8::MAX / 2);
        for node_idx in 0..node_count {
            node_distances[(node_idx, node_idx)] = 0;
            for &edge in &node_edges[&node_idx] {
                let dest_idx = node_ids[&edge];
                node_distances[(node_idx, dest_idx)] = 1;
            }
        }
    
        for k in 0..node_count {
            for i in 0..node_count {
                for j in 0..node_count {
                    node_distances[(i, j)] =
                        cmp::min(node_distances[(i, j)],
                                 node_distances[(i, k)] + node_distances[(k, j)]);
                }
            }
        }
    
        (node_distances, flow_rates, next_flow_node_idx as u8)
    }

}

pub fn part1(input: &str) -> u64 {
    SimulationState::<1, 30>::new(input).run_simulation()
}

pub fn part2(input: &str) -> u64 {
    SimulationState::<2, 26>::new(input).run_simulation()
}