//! Module to generate perlin noise

use crate::random_int;
use crate::vec3::{ Point3, Vec3 };

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
    rand_vecs: [Vec3; N],
    perm_x: PerlinPerm<N>,
    perm_y: PerlinPerm<N>,
    perm_z: PerlinPerm<N>,
}
impl<const N: usize> PerlinGenerator<N> {
    fn new(
        rand_vecs: [Vec3; N],
        perm_x: PerlinPerm<N>,
        perm_y: PerlinPerm<N>,
        perm_z: PerlinPerm<N>
    ) -> Self {
        PerlinGenerator { rand_vecs, perm_x, perm_y, perm_z }
    }
    pub fn init() -> Self {
        let rand_vecs: [Vec3; N] = std::array::from_fn(|_| { Vec3::random(-1.0, 1.0) });
        let perm_x = PerlinPerm::<N>::generate();
        let perm_y = PerlinPerm::<N>::generate();
        let perm_z = PerlinPerm::<N>::generate();
        PerlinGenerator::new(rand_vecs, perm_x, perm_y, perm_z)
    }
    pub fn noise(&self, p: Point3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.rand_vecs[
                        (self.perm_x.perm[((i + (di as i32)) & 255) as usize] ^
                            self.perm_y.perm[((j + (dj as i32)) & 255) as usize] ^
                            self.perm_z.perm[((k + (dk as i32)) & 255) as usize]) as usize
                    ];
                }
            }
        }
        perlin_interp(c, u, v, w)
    }
    pub fn turb(&self, p: Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p = 2.0 * temp_p;
        };
        accum.abs()
    }
}

/// Trilinear interpolation extrapolated to vectors instead of floats
fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    // hermitian smoothing
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight_v = Vec3::new(u - (i as f64), v - (j as f64), w - (k as f64));
                accum +=
                    ((i as f64) * uu + (1.0 - (i as f64)) * (1.0 - uu)) *
                    ((j as f64) * vv + (1.0 - (j as f64)) * (1.0 - vv)) *
                    ((k as f64) * ww + (1.0 - (k as f64)) * (1.0 - ww)) *
                    c[i][j][k].dot(&weight_v);
            }
        }
    }

    accum
}

fn _trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut accum = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                accum +=
                    ((i as f64) * u + (1.0 - (i as f64)) * (1.0 - u)) *
                    ((j as f64) * v + (1.0 - (j as f64)) * (1.0 - v)) *
                    ((k as f64) * w + (1.0 - (k as f64)) * (1.0 - w)) *
                    c[i][j][k];
            }
        }
    }
    accum
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
