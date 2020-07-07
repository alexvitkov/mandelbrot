use num::complex::Complex;

use crossbeam;
use std::sync::{Arc, Mutex};

mod v2;
use v2::{V2f, V2u, V2};

#[derive(Copy, Clone)]
pub struct Options {
    tasks: usize,
    iter: usize,
    pub img_size: V2u,
    min: V2f,
    multiplier: V2f,
    chunksize: usize,
}

impl Options {
    pub fn new(
        img_size: (usize, usize),
        rect: (f32, f32, f32, f32),
        thread_count: usize,
        iter: usize,
        chunksize: usize,
    ) -> Options {
        let (xmin, xmax, ymin, ymax) = rect;

        let img_size_v: V2u = V2u::from(img_size);
        let img_size_f = V2f::from(&img_size_v);
        let min = V2f { x: xmin, y: ymin };
        let max = V2f { x: xmax, y: ymax };
        let rect_size = &max - &min;
        let multiplier = &rect_size / &img_size_f;

        Options {
            img_size: img_size_v,
            min: min,
            multiplier: multiplier,
            tasks: thread_count,
            iter: iter,
            chunksize: chunksize,
        }
    }

    pub fn p2c(&self, p: &V2u) -> V2f {
        V2 {
            x: p.x as f32 * self.multiplier.x + self.min.x,
            y: p.y as f32 * self.multiplier.y + self.min.y,
        }
        //  &(&V2<T>::from(p) * &self.multiplier) + &self.min
        /*
        V2 {
            x: F::from_usize(p.x).unwrap() * F::from_f32(self.multiplier.x).unwrap(),
            y: F::from_usize(p.x).unwrap() * F::from_f32(self.multiplier.x).unwrap(),
        }
        */
    }
}

struct SharedData<'a> {
    chunks: &'a mut std::slice::ChunksMut<'a, u8>,
    startpx: usize,
}

pub fn compute(o: Options) -> Vec<u8> {
    let size = 3 * o.img_size.x * o.img_size.y;
    let mut vtor: Vec<u8> = Vec::with_capacity(size);
    unsafe {
        vtor.set_len(size);
    }

    let mut ch = vtor.chunks_mut(o.chunksize * 3);

    let _ = crossbeam::scope(|scope| {
        let shared = SharedData {
            chunks: &mut ch,
            startpx: 0,
        };
        let mutex = Arc::new(Mutex::new(shared));

        for i in 0..o.tasks {
            let arc2 = Arc::clone(&mutex);
            scope.spawn(move |_| {
                worker(arc2, &o, i);
            });
        }
    });

    vtor
}

fn worker(shared: Arc<Mutex<SharedData>>, o: &Options, n: usize) {
    let now = std::time::Instant::now();
    loop {
        let data: &mut [u8];
        let startpx: usize;
        {
            let mut shared = shared.lock().unwrap();

            match (*shared).chunks.next() {
                Some(x) => {
                    data = x;
                    startpx = (*shared).startpx;
                    (*shared).startpx += data.len() / 3;
                }
                None => {
                    print!("Thread {} finished in ", n);
                    if now.elapsed().as_millis() < 2000 {
                        println!(" {}ms", now.elapsed().as_millis())
                    } else {
                        println!(" {}s", now.elapsed().as_millis() as f32 / 1000.)
                    }
                    return;
                }
            }
        }

        let w = o.img_size.x;
        let endpx = startpx + data.len() / 3;

        for i in startpx..endpx {
            let x = i % w;
            let y = i / w;
            let ds = i * 3 - startpx * 3;
            let val = mandelbrot(o, &V2u { x, y });
            if val < 0 {
                data[ds] = 255;
                data[ds + 1] = 255;
                data[ds + 2] = 255;
            } else {
                let v = val as f32 / o.iter as f32;
                let b = (v * 255.) as u8;
                data[ds] = b / 2;
                data[ds + 1] = 0;
                data[ds + 2] = b;
            }

            //println!("{} {} {}", x, y, data[ds]);
            //panic!();
        }
    }
}

fn mandelbrot(o: &Options, px: &V2u) -> i32
where
{
    let pxf = o.p2c(px);

    let c = Complex::new(pxf.x, pxf.y);
    let mut z = Complex::new(0., 0.);

    for i in 0..o.iter {
        // Normalen mandelbrot
        // z = z * z + c;
        // proekt 18
        z = c * z.cos();
        // proekt 16
        // z = c * Complex::new(std::f32::consts::E, 0.).powc(-z) + z * z;

        let norm = z.norm();
        if norm > 2. || norm.is_nan() {
            return i as i32;
        }
    }
    -1
}
