use crate::vec3::Point3;

use rand::Rng;

pub struct Perlin {
    point_count: u32,
    ranfloat: [f64; 256],
    perm_x: [u32; 256],
    perm_y: [u32; 256],
    perm_z: [u32; 256],
      
}

impl Perlin {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut ranfloat: [f64; 256] = [0.0; 256];

        for i in 0..256 {
            ranfloat[i] =  rng.gen_range(0.0..1.0);
        }

        Perlin {
            point_count: 256,
            ranfloat: ranfloat,
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let i =  ((4.0*p.x()) as i32) & 255;
        let j =  ((4.0*p.y()) as i32) & 255;
        let k =  ((4.0*p.z()) as i32) & 255;

        return self.ranfloat[(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize];
    }

    fn permute(p: &mut [u32; 256], n: u32) -> () {
        let mut rng = rand::thread_rng();

        for i in (1..n).rev() {
            let target: u32 = rng.gen_range(0..i);
            let tmp: u32 = p[i as usize];
            p[i as usize] = p[target as usize];
            p[target as usize] = tmp;
        }
    }

    fn perlin_generate_perm() -> [u32; 256] {
        let mut p : [u32; 256] = [0; 256];

        for i in 0..p.len() {
            p[i] =  i as u32;
        }

        Perlin::permute(&mut p, 256);
  
        return p;
    }
}

