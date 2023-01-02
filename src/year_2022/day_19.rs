
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

fn div_ceil(num: u16, div: u16) -> u16 {
    let mut val = num / div;
    if (num % div) != 0 {
        val += 1;
    }

    val
}

fn parse_int(bytes: &[u8], index: &mut usize) -> u16 {
    let mut val = 0;
    while bytes[*index] >= b'0' && bytes[*index] <= b'9' {
        val = (val * 10) + ((bytes[*index] - b'0') as u16);
        *index += 1;
    }

    val
}

#[derive(Clone, Copy, Debug, Default)]
struct Resources {
    ore: u16,
    clay: u16,
    obs: u16
}

impl Resources {

    fn new(ore: u16, clay: u16, obs: u16) -> Self {
        Resources {
            ore,
            clay,
            obs
        }
    }

}

impl Add<Resources> for Resources {

    type Output = Resources;

    fn add(self, rhs: Resources) -> Self::Output {
        let mut new = self;
        new += rhs;
        new
    }

}

impl AddAssign<Resources> for Resources {

    fn add_assign(&mut self, rhs: Resources) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obs += rhs.obs;
    }

}

impl Mul<u16> for Resources {

    type Output = Resources;

    fn mul(self, rhs: u16) -> Self::Output {
        let mut new = self;
        new *= rhs;
        new
    }

}

impl MulAssign<u16> for Resources {

    fn mul_assign(&mut self, rhs: u16) {
        self.ore *= rhs;
        self.clay *= rhs;
        self.obs *= rhs;
    }

}

impl Sub<Resources> for Resources {

    type Output = Resources;

    fn sub(self, rhs: Resources) -> Self::Output {
        let mut new = self;
        new -= rhs;
        new
    }

}

impl SubAssign<Resources> for Resources {

    fn sub_assign(&mut self, rhs: Resources) {
        self.ore -= rhs.ore;
        self.clay -= rhs.clay;
        self.obs -= rhs.obs;
    }

}

struct State {
    remaining_time: u16,
    current_resources: Resources,
    current_rate: Resources,
    geodes: u64
}

impl State {

    fn initial_state(iteration_count: u16) -> Self {
        Self {
            remaining_time: iteration_count,
            current_resources: Resources::new(0, 0, 0),
            current_rate: Resources::new(1, 0, 0),
            geodes: 0
        }
    }

    fn get_potential(&self) -> u64 {
        let rem_time = self.remaining_time as u64;
        (rem_time * (rem_time - 1) / 2) + self.geodes
    }

    fn get_time_to_build(&self, rb_cost: Resources) -> u16 {
        let ore_build_time = if self.current_resources.ore < rb_cost.ore {
            div_ceil(rb_cost.ore - self.current_resources.ore, self.current_rate.ore)

        } else {
            0
        };
        
        let clay_build_time = if self.current_resources.clay < rb_cost.clay {
            div_ceil(rb_cost.clay - self.current_resources.clay, self.current_rate.clay)

        } else {
            0
        };
        
        let obs_build_time = if self.current_resources.obs < rb_cost.obs {
            div_ceil(rb_cost.obs - self.current_resources.obs, self.current_rate.obs)

        } else {
            0
        };

        ore_build_time.max(clay_build_time).max(obs_build_time) + 1
    }

    fn branch(&self, blueprint: &Blueprint) -> impl Iterator<Item = State> {
        let max_resources_produced = self.current_resources + (self.current_rate * self.remaining_time);
        let max_resources_consumed = blueprint.max_rb_cost * self.remaining_time;
        let geode_state = if self.current_rate.obs > 0 {
            let build_time = self.get_time_to_build(blueprint.geode_rb_cost);
            let new_remaining_time = self.remaining_time.checked_sub(build_time).unwrap_or(0);
            if new_remaining_time >= 1 {
                let new_resources = self.current_resources + (self.current_rate * build_time) - blueprint.geode_rb_cost;
                let new_state = State {
                    remaining_time: new_remaining_time,
                    current_resources: new_resources,
                    current_rate: self.current_rate,
                    geodes: self.geodes + (new_remaining_time as u64)
                };

                Some(new_state)

            } else {
                None
            }

        } else {
            None
        };

        let ore_state = if max_resources_produced.ore < max_resources_consumed.ore {
            let build_time = self.get_time_to_build(blueprint.ore_rb_cost);
            let new_remaining_time = self.remaining_time.checked_sub(build_time).unwrap_or(0);
            if new_remaining_time >= 2 {
                let new_resources = self.current_resources + (self.current_rate * build_time) - blueprint.ore_rb_cost;
                let new_state = State {
                    remaining_time: new_remaining_time,
                    current_resources: new_resources,
                    current_rate: self.current_rate + Resources::new(1, 0, 0),
                    geodes: self.geodes
                };

                Some(new_state)

            } else {
                None
            }

        } else {
            None
        };

        let clay_state = if max_resources_produced.clay < max_resources_consumed.clay {
            let build_time = self.get_time_to_build(blueprint.clay_rb_cost);
            let new_remaining_time = self.remaining_time.checked_sub(build_time).unwrap_or(0);
            if new_remaining_time >= 3 {
                let new_resources = self.current_resources + (self.current_rate * build_time) - blueprint.clay_rb_cost;
                let new_state = State {
                    remaining_time: new_remaining_time,
                    current_resources: new_resources,
                    current_rate: self.current_rate + Resources::new(0, 1, 0),
                    geodes: self.geodes
                };

                Some(new_state)

            } else {
                None
            }

        } else {
            None
        };

        let obs_state = if self.current_rate.clay > 0 && max_resources_produced.obs < max_resources_consumed.obs {
            let build_time = self.get_time_to_build(blueprint.obs_rb_cost);
            let new_remaining_time = self.remaining_time.checked_sub(build_time).unwrap_or(0);
            if new_remaining_time >= 2 {
                let new_resources = self.current_resources + (self.current_rate * build_time) - blueprint.obs_rb_cost;
                let new_state = State {
                    remaining_time: new_remaining_time,
                    current_resources: new_resources,
                    current_rate: self.current_rate+ Resources::new(0, 0, 1),
                    geodes: self.geodes
                };

                Some(new_state)

            } else {
                None
            }

        } else {
            None
        };

        geode_state.into_iter()
        .chain(ore_state)
        .chain(clay_state)
        .chain(obs_state)
    }

}

struct Blueprint {
    ore_rb_cost: Resources,
    clay_rb_cost: Resources,
    obs_rb_cost: Resources,
    geode_rb_cost: Resources,

    max_rb_cost: Resources
}

impl Blueprint {

    fn parse_blueprint(desc: &str) -> Blueprint {
        let bytes = desc.as_bytes();
        let mut index = "Blueprint ".len();

        let _id = parse_int(bytes, &mut index);
        
        index += ": Each ore robot costs ".len();
        let ore_rb_cost = parse_int(bytes, &mut index);
        
        index += " ore. Each clay robot costs ".len();
        let clay_rb_cost = parse_int(bytes, &mut index);

        index += " ore. Each obsidian robot costs ".len();
        let obs_rb_ore_cost = parse_int(bytes, &mut index);
        index += " ore and ".len();
        let obs_rb_clay_cost = parse_int(bytes, &mut index);

        index += " clay. Each geode robot costs ".len();
        let geode_rb_ore_cost = parse_int(bytes, &mut index);
        index += " ore and ".len();
        let geode_rb_obs_cost = parse_int(bytes, &mut index);

        let max_rb_cost =
            Resources::new(
                ore_rb_cost.max(clay_rb_cost).max(obs_rb_ore_cost).max(geode_rb_ore_cost),
                clay_rb_cost.max(obs_rb_clay_cost),
                geode_rb_obs_cost
                );

        Blueprint {
            ore_rb_cost: Resources::new(ore_rb_cost, 0, 0),
            clay_rb_cost: Resources::new(clay_rb_cost, 0, 0),
            obs_rb_cost: Resources::new(obs_rb_ore_cost, obs_rb_clay_cost, 0),
            geode_rb_cost: Resources::new(geode_rb_ore_cost, 0, geode_rb_obs_cost),

            max_rb_cost
        }
    }

    fn simulate_blueprint<const ITERATION_COUNT: u16>(&self) -> u64 {
        let mut best_count = 0;
        self.run_simulation(
            State::initial_state(ITERATION_COUNT as u16),
            &mut best_count);

        best_count
    }

    fn run_simulation(&self, current_state: State, current_best: &mut u64) {
        *current_best = (*current_best).max(current_state.geodes);
        for next_state in current_state.branch(self) {
            if next_state.get_potential() > *current_best {
                self.run_simulation(next_state, current_best);
            }
        }
    }

}

pub fn part1(input: &str) -> u64 {
    input
    .lines()
    .map(|line| Blueprint::parse_blueprint(line))
    .map(|bp| bp.simulate_blueprint::<24>())
    .enumerate()
    .map(|(index, geodes)| ((index as u64) + 1) * geodes)
    .sum()
}

pub fn part2(input: &str) -> u64 {
    input
    .lines()
    .take(3)
    .map(|line| Blueprint::parse_blueprint(line))
    .map(|bp| bp.simulate_blueprint::<32>())
    .fold(1, |p, geodes| p * geodes)
}