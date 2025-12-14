use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SeedFunctionRange {
    domain_start: i64,
    transform: i64,
    range_length: i64,
}

impl SeedFunctionRange {
    pub fn new(domain_start: i64, transform: i64, range_length: i64) -> Self {
        Self {
            domain_start,
            transform,
            range_length,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SeedFunctionMap {
    ranges: HashSet<SeedFunctionRange>,
}

impl SeedFunctionMap {
    pub fn new(ranges: HashSet<SeedFunctionRange>) -> Self {
        Self { ranges }
    }

    pub fn run(&self, value: i64) -> i64 {
        for range in &self.ranges {
            if range.domain_start <= value && value <= range.domain_start + range.range_length {
                return range.transform + value;
            }
        }

        value
    }

    pub fn rev_run(&self, value: i64) -> i64 {
        for range in &self.ranges {
            if range.domain_start + range.transform <= value
                && value <= range.domain_start + range.transform + range.range_length
            {
                return value - range.transform;
            }
        }

        value
    }
}

#[derive(Debug, Clone)]
pub struct SeedMap {
    pub seeds: Vec<i64>,
    pub seed_soil_map: SeedFunctionMap,
    pub soil_fertilizer_map: SeedFunctionMap,
    pub fertilizer_water_map: SeedFunctionMap,
    pub water_light_map: SeedFunctionMap,
    pub light_temp_map: SeedFunctionMap,
    pub temp_humidity_map: SeedFunctionMap,
    pub humidity_loc_map: SeedFunctionMap,
}

impl SeedMap {
    pub fn run(&self, value: i64) -> i64 {
        self.humidity_loc_map.run(
            self.temp_humidity_map.run(
                self.light_temp_map.run(
                    self.water_light_map.run(
                        self.fertilizer_water_map
                            .run(self.soil_fertilizer_map.run(self.seed_soil_map.run(value))),
                    ),
                ),
            ),
        )
    }

    // pub fn run_range(&self) -> Vec<i64> {
    //     self.seeds
    //         .iter()
    //         .map(|x| self.seed_soil_map.run(*x))
    //         .map(|x| self.soil_fertilizer_map.run(x))
    //         .map(|x| self.fertilizer_water_map.run(x))
    //         .map(|x| self.water_light_map.run(x))
    //         .map(|x| self.light_temp_map.run(x))
    //         .map(|x| self.temp_humidity_map.run(x))
    //         .map(|x| self.humidity_loc_map.run(x))
    //         .collect()
    // }

    pub fn is_in_seed_range(&self, value: i64) -> bool {
        self.seeds
            .chunks(2)
            .map(|x| x[0]..=(x[0] + x[1]))
            .any(|x| x.contains(&value))
    }

    pub fn rev_run(&self, value: i64) -> i64 {
        self.seed_soil_map.rev_run(
            self.soil_fertilizer_map.rev_run(
                self.fertilizer_water_map.rev_run(
                    self.water_light_map.rev_run(
                        self.light_temp_map.rev_run(
                            self.temp_humidity_map
                                .rev_run(self.humidity_loc_map.rev_run(value)),
                        ),
                    ),
                ),
            ),
        )
    }
}
