//! Module to generate perlin noise

use crate::{ random_int, random_float };
use crate::vec3::Point3;

/// Perlin Permutation set of length N
#[derive(Debug, Clone, PartialEq)]
struct PerlinPerm<const N: usize> {
    pub perm: [i32; N],
}
impl<const N: usize> PerlinPerm<N> {
    fn new(perm: [i32; N]) -> Self {
        PerlinPerm { perm }
    }
    pub fn generate() -> Self {
        let mut arr: [i32; N] = std::array::from_fn(|i| i as i32);
        for i in (1..N).rev() {
            let target = random_int(0, i as i32) as usize;
            let tmp = arr[i];
            arr[i] = arr[target];
            arr[target] = tmp;
        }
        PerlinPerm::new(arr)
    }
}

/// Perlin Noise generator
#[derive(Debug, Clone)]
pub struct PerlinGenerator<const N: usize> {
    rand_floats: [f64; N],
    perm_x: PerlinPerm<N>,
    perm_y: PerlinPerm<N>,
    perm_z: PerlinPerm<N>,
}
impl<const N: usize> PerlinGenerator<N> {
    fn new(
        rand_floats: [f64; N],
        perm_x: PerlinPerm<N>,
        perm_y: PerlinPerm<N>,
        perm_z: PerlinPerm<N>
    ) -> Self {
        PerlinGenerator { rand_floats, perm_x, perm_y, perm_z }
    }
    pub fn init() -> Self {
        let rand_floats: [f64; N] = std::array::from_fn(|_| { random_float(0.0, 1.0) });
        let perm_x = PerlinPerm::<N>::generate();
        let perm_y = PerlinPerm::<N>::generate();
        let perm_z = PerlinPerm::<N>::generate();
        PerlinGenerator::new(rand_floats, perm_x, perm_y, perm_z)
    }
    pub fn noise(&self, p: Point3) -> f64 {
        let i = ((4.0 * p.x) as i32 & 255) as usize;
        let j = ((4.0 * p.y) as i32 & 255) as usize;
        let k = ((4.0 * p.z) as i32 & 255) as usize;

        self.rand_floats[(self.perm_x.perm[i] ^ self.perm_y.perm[j] ^ self.perm_z.perm[k]) as usize]
    }
}

#[test]
pub fn test_perm() {
    let gen_perm = PerlinPerm::<10>::generate();
    let default_perm = PerlinPerm::<10>::new(std::array::from_fn(|i| i as i32));
    assert_ne!(gen_perm, default_perm);
}

#[test]
pub fn test_noise() {
    let pg = PerlinGenerator::<256>::init();
    let noise = pg.noise(Point3::zero());
    assert!(noise < 1.0);
    assert!(noise >= 0.0);
}