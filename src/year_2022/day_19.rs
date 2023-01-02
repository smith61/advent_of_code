
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

    fn simulate_blueprint<const ITERATION_COUNT: u8>(&self) -> u64 {
        self.max_geodes(
            ITERATION_COUNT,
            Resources::new(0, 0, 0),
            Resources::new(1, 0, 0))
    }

    fn get_time_to_build(cur_goods: Resources, cur_rate: Resources, rb_cost: Resources) -> u16 {
        let ore_build_time = if cur_goods.ore < rb_cost.ore {
            div_ceil(rb_cost.ore - cur_goods.ore, cur_rate.ore)

        } else {
            0
        };
        
        let clay_build_time = if cur_goods.clay < rb_cost.clay {
            div_ceil(rb_cost.clay - cur_goods.clay, cur_rate.clay)

        } else {
            0
        };
        
        let obs_build_time = if cur_goods.obs < rb_cost.obs {
            div_ceil(rb_cost.obs - cur_goods.obs, cur_rate.obs)

        } else {
            0
        };

        ore_build_time.max(clay_build_time).max(obs_build_time) + 1
    }

    fn max_geodes(&self, rem_time: u8, cur_goods: Resources, cur_rate: Resources) -> u64 {
        if rem_time <= 1 {
            return 0;
        }

        let mut max_geodes = 0;
        if cur_rate.ore > 0 &&
           cur_rate.obs > 0 {

            let build_time = Self::get_time_to_build(cur_goods, cur_rate, self.geode_rb_cost);
            if rem_time as u16 > build_time {
                let new_rem_time = rem_time - (build_time as u8);
                let new_goods = cur_goods + (cur_rate * build_time) - self.geode_rb_cost;

                let geodes =
                    self.max_geodes(new_rem_time, new_goods, cur_rate);

                max_geodes = max_geodes.max(geodes + (new_rem_time as u64));
            }
        }

        let next_potential = {
            let rem_time = rem_time as u64;
            (rem_time - 1) / 2 * rem_time
        };

        if next_potential > max_geodes {
            let max_resources_produced = cur_goods + cur_rate * (rem_time as u16);
            let max_resources_used = self.max_rb_cost * (rem_time as u16);
            if max_resources_produced.ore < max_resources_used.ore {
                let build_time = Self::get_time_to_build(cur_goods, cur_rate, self.ore_rb_cost);
                if rem_time as u16 > (build_time + 1) {
                    let new_goods = cur_goods + (cur_rate * build_time) - self.ore_rb_cost;
                    let new_rate = cur_rate + Resources::new(1, 0, 0);

                    let geodes =
                        self.max_geodes(rem_time - (build_time as u8), new_goods, new_rate);

                    max_geodes = max_geodes.max(geodes);
                }
            }

            if max_resources_produced.clay < max_resources_used.clay {
                let build_time = Self::get_time_to_build(cur_goods, cur_rate, self.clay_rb_cost);
                if rem_time as u16 > (build_time + 1) {
                    let new_goods = cur_goods + (cur_rate * build_time) - self.clay_rb_cost;
                    let new_rate = cur_rate + Resources::new(0, 1, 0);

                    let geodes =
                        self.max_geodes(rem_time - (build_time as u8), new_goods, new_rate);

                    max_geodes = max_geodes.max(geodes);
                }
            }

            if max_resources_produced.obs < max_resources_used.clay &&
               cur_rate.ore > 0 &&
               cur_rate.clay > 0 {

                let build_time = Self::get_time_to_build(cur_goods, cur_rate, self.obs_rb_cost);
                if rem_time as u16 > (build_time + 1) {
                    let new_goods = cur_goods + (cur_rate * build_time) - self.obs_rb_cost;
                    let new_rate = cur_rate + Resources::new(0, 0, 1);

                    let geodes =
                        self.max_geodes(rem_time - (build_time as u8), new_goods, new_rate);

                    max_geodes = max_geodes.max(geodes);
                }
            }
        }

        max_geodes
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