use rand::Rng;

pub struct Perlin {
    point_count: u32
    ranfloat: [f64: 256],
    perm_x: [u32: 256],
    perm_y: [u32: 256],
    perm_z: [u32: 256],
      
}

impl Perlin {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut ranfloat = [f64: 256];

        for i in 0..ranfloat.len() {
            randfloat[i] =  rng.gen();
        }

        Perlin {
            point_count: 256,
            randfloat: randfloat,
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    pub noise(p: &Point3) -> f64 {
        let i =  (4.0*p.x() as u32) & 255;
        let j =  (4.0*p.y() as u32) & 255;
        let k =  (4.0*p.z() as u32) & 255;

        return self.ranfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]];
    }

    fn perlin_generate_perm() -> [u32: 256] {
        let p = [u32: 256];

        for i in 0..p.len() {
            p[i] =  i;
        }

        permute(&p, 256);
  
        return p;
    }

    fn permute(p: &[u32: 256], n: u32) -> [u32: 256] {
        let mut rng = rand::thread_rng();

        for i in (1..n).rev() {
            let target: u32 = rng.gen_range(0..i);
            let tmp: u32 = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
    }
}

