use crate::vec3::Point3;
use crate::vec3::Vec3;

use rand::Rng;

pub struct Perlin {
    #[allow(dead_code)]
    point_count: u32,
    ranvec: [Vec3; 256],
    perm_x: [u32; 256],
    perm_y: [u32; 256],
    perm_z: [u32; 256],
      
}

impl Perlin {
    pub fn new() -> Self {
        let mut ranvec: [Vec3; 256] = [Vec3(0.0, 0.0, 0.0); 256];

        for i in 0..256 {
            ranvec[i] = Vec3::unit_vector(Vec3::random_range(-1.0, 1.0));
        }

        Perlin {
            point_count: 256,
            ranvec: ranvec,
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let mut u = p.x() - p.x().floor();
        let mut v = p.y() - p.y().floor();
        let mut w = p.z() - p.z().floor();
        u = u*u*(3.0-(2.0*u));
        v = v*v*(3.0-(2.0*v));
        w = w*w*(3.0-(2.0*w));

        let i =  p.x().floor() as i32;
        let j =  p.y().floor() as i32;
        let k =  p.z().floor() as i32;

        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3(0.0, 0.0, 0.0);2];2];2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[(self.perm_x[((i + di as i32) & 255) as usize] ^ self.perm_y[((j + dj as i32) & 255) as usize] ^ self.perm_z[((k + dk as i32) & 255) as usize]) as usize];
                }
            } 
        }


        return Perlin::perlin_interp(c, u, v, w);
    }

    pub fn turb(&self, p: &Point3) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;
        
        for _i in 0..7 {
            accum += weight*self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        
        accum.abs()
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

    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u*u*(3.0-(2.0*u));
        let vv = v*v*(3.0-(2.0*v));
        let ww = w*w*(3.0-(2.0*w));
        let mut accum: f64 = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 *uu + (1.0-i as f64 )*(1.0-uu))*
                            (j as f64 *vv + (1.0-j as f64 )*(1.0-vv))*
                            (k as f64 *ww + (1.0-k as f64 )*(1.0-ww))*Vec3::dot(c[i][j][k], weight_v);
                }
            } 
        }
        accum
    }
}

