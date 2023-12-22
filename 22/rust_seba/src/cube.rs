#[derive(Debug, Clone, Copy)]
pub struct Cube {
    pub index: u32,
    x: u32,
    y: u32,
    z: u32,
    xx: u32,
    yy: u32,
    zz: u32,
}

impl Cube {
    pub fn new(index: u32, x: u32, y: u32, z: u32, xx: u32, yy: u32, zz: u32) -> Self {
        Self {
            index,
            x,
            y,
            z,
            xx,
            yy,
            zz,
        }
    }

    pub fn has_settled(&self, cubes: &[Cube]) -> bool {
        self.z == 1
            || cubes
                .iter()
                .any(|other| other.zz == self.z - 1 && self.superposed(other))
    }

    fn superposed(&self, other: &Cube) -> bool {
        !(other.x > self.xx || other.xx < self.x || other.y > self.yy || other.yy < self.y)
    }

    fn settle_at_z(&mut self, z: u32) {
        let height = self.zz - self.z;
        self.z = z;
        self.zz = z + height;
    }

    fn next_support_level(&self, cubes: &[Cube]) -> u32 {
        cubes
            .iter()
            .filter(|other| other.zz < self.z && self.superposed(other))
            .map(|other| other.zz)
            .max()
            .unwrap_or(0)
            + 1
    }

    pub fn settle_at_next_support_level(&mut self, cubes: &[Cube]) {
        if self.has_settled(cubes) {
            return;
        }
        self.settle_at_z(self.next_support_level(cubes));
    }

    fn has_several_support(&self, cubes: &[Cube]) -> bool {
        cubes
            .iter()
            .filter(|other| other.zz == self.z - 1 && self.superposed(other))
            .count()
            > 1
    }

    pub fn can_destroy(&self, cubes: &[Cube]) -> bool {
        if !cubes
            .iter()
            .any(|other| other.z == self.zz + 1 && self.superposed(other))
        {
            return true;
        }
        let supported_cubes = cubes
            .iter()
            .filter(|other| other.z == self.zz + 1 && self.superposed(other))
            .collect::<Vec<&Cube>>();
        if supported_cubes
            .iter()
            .all(|cube| cube.has_several_support(cubes))
        {
            return true;
        }
        false
    }

    pub fn dependencies(&self, cubes: &[Cube]) -> Vec<u32> {
        cubes
            .iter()
            .filter(|other| other.zz == self.z - 1 && self.superposed(other))
            .map(|other| other.index)
            .collect::<Vec<u32>>()
    }
}

impl Ord for Cube {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.z.cmp(&other.z)
    }
}

impl PartialOrd for Cube {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        self.z == other.z
    }
}

impl Eq for Cube {}
