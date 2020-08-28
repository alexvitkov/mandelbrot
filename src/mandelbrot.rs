use num::complex::Complex;
use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

#[derive(Copy, Clone)]
pub struct Options {
    tasks: usize,
    iter: usize,
    pub img_size_x: usize,
    pub img_size_y: usize,
    pub img_size_x_f32: f32,
    pub img_size_y_f32: f32,
    pub granularity: usize,

    min_x: f32,
    min_y: f32,
    width: f32,
    height: f32,
}

impl Options {
    pub fn new(
        img_size: (usize, usize),
        rect: (f32, f32, f32, f32),
        thread_count: usize,
        iter: usize,
        granularity: usize,
    ) -> Options {

        Options {
            img_size_x: img_size.0,
            img_size_y: img_size.1,
            img_size_x_f32: img_size.0 as f32,
            img_size_y_f32: img_size.1 as f32,
            min_x: rect.0,
            min_y: rect.2,
            width: rect.1 - rect.0,
            height: rect.3 - rect.2,
            tasks: thread_count,
            iter: iter,
            granularity: granularity,
        }
    }
}

pub static mut DATA: Vec<u8> = vec![];

pub static ALIVE_THREADS: AtomicUsize = ATOMIC_USIZE_INIT;

pub fn compute(o: Options) { 
    let vec_size = 3 * o.img_size_x * o.img_size_y;
    let start_time = std::time::Instant::now();

    unsafe {
        DATA = Vec::with_capacity(vec_size);
        DATA.set_len(vec_size);

        let chunks_count = o.tasks * o.granularity;
        let chunk_size = DATA.len() / chunks_count;


        for i in 0..(o.tasks) {
            thread::spawn(move || {
                ALIVE_THREADS.fetch_add(1, Ordering::SeqCst);
                worker(chunks_count, chunk_size, &o, i);
                ALIVE_THREADS.fetch_sub(1, Ordering::SeqCst);
            });
        };
    }

    thread::sleep(std::time::Duration::from_millis(200)); 
    while ALIVE_THREADS.load(Ordering::SeqCst) != 0 {
        thread::sleep(std::time::Duration::from_millis(1)); 
    }

    print!("All done in ");
    if start_time.elapsed().as_millis() < 2000 {
        println!(" {}ms", start_time.elapsed().as_millis())
    } else {
        println!(" {}s", start_time.elapsed().as_millis() as f32 / 1000.)
    }
}

unsafe fn worker(chunks_count: usize, chunk_size: usize, o: &Options, thread_id: usize) {
    let start_time = std::time::Instant::now();

    println!("Thread {} started.", thread_id);
        let mut c: i32 = 0;

    for n in (thread_id..chunks_count).step_by(o.tasks) {

        let chunk_start = n * chunk_size;
        let mut chunk_end = chunk_start + chunk_size;
        if chunk_end > (*DATA).len() - 2 {
            chunk_end = (*DATA).len() - 3;
        }

        // println!("{} {}" , chunk_start, chunk_end);

        for i in (chunk_start..chunk_end).step_by(3) {
            c+=1;
            let x = (i / 3) % o.img_size_x;
            let y = (i / 3) / o.img_size_x;
            let val = mandelbrot(o, x, y);
            if val < 0 {
                (*DATA)[i] = 255;
                (*DATA)[i + 1] = 255;
                (*DATA)[i + 2] = 255;
            } else {
                let v = val as f32 / o.iter as f32;
                let b = (v * 255.) as u8;
                (*DATA)[i] = b / 2;
                (*DATA)[i + 1] = 0;
                (*DATA)[i + 2] = b;
            }
        }
    }

    print!("Thread {} ({}) finished in ", thread_id, c);
    if start_time.elapsed().as_millis() < 2000 {
        println!(" {}ms", start_time.elapsed().as_millis())
    } else {
        println!(" {}s", start_time.elapsed().as_millis() as f32 / 1000.)
    }

}

fn mandelbrot(o: &Options, x: usize, y: usize) -> i32 {

    let x = ((x as f32) / o.img_size_x_f32) * o.width + o.min_x;
    let y = ((y as f32) / o.img_size_y_f32) * o.height + o.min_y;

    let c = Complex::new(x as f32, y as f32);

    let mut z = Complex::new(0., 0.);

    for i in 0..o.iter {
        // z = z * z + c;                                                       // Normalen mandelbrot
        z = c * z.cos();                                                        // proekt 18
        // z = c * Complex::new(std::f32::consts::E, 0.).powc(-z) + z * z;      // proekt 16

        let norm = z.norm();
        if norm > 2. || norm.is_nan() {
            return i as i32;
        }
    }
    -1
}