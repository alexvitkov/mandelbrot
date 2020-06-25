extern crate clap;
extern crate crossbeam;
extern crate png;

mod mandelbrot;
mod savepng;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Mandelbrot4o")
        .arg(
            Arg::with_name("size")
                .short("s")
                .long("size")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("rect")
                .short("r")
                .long("rect")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("tasks")
                .short("t")
                .long("tasks")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("iter")
                .short("i")
                .long("iter")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("granularity")
                .short("g")
                .long("granularity")
                .takes_value(true),
        )
        .arg(Arg::with_name("nooutput").long("nooutput"))
        .get_matches();

    let size_input = matches.value_of("size").unwrap_or("640x480");
    let size = parse_size(&size_input).expect(&format!("Invalid size {}", &size_input));

    let rect_input = matches.value_of("rect").unwrap_or("-2.0:2.0:-1.0:1.0");
    let rect = parse_rect(&rect_input).expect(&format!("Invalid rect {}", &rect_input));

    let outputpath = matches.value_of("output").unwrap_or("output.png");

    let tasks = matches
        .value_of("tasks")
        .unwrap_or("1")
        .parse::<usize>()
        .expect("Invalid tasks, expected integer");

    let iter = matches
        .value_of("iter")
        .unwrap_or("50")
        .parse::<usize>()
        .expect("Invalid iter, expected integer");

    let chunksize: usize = size.0 * size.1 * tasks
        / matches
            .value_of("granularity")
            .unwrap_or("1")
            .parse::<usize>()
            .expect("Inalid granularity, expected integer");

    let options = mandelbrot::Options::new(size, rect, tasks, iter, chunksize);
    let data = mandelbrot::compute(options);

    /*
    for y in 0..options.img_size.y {
        for x in 0..options.img_size.x {
            print!("{}", data[(x + y * options.img_size.x) * 3]);
        }
        println!("");
    }
    */

    if !matches.is_present("nooutput") {
        savepng::save(
            options.img_size.x,
            options.img_size.y,
            &data[..],
            outputpath,
        );
    }
}

fn parse_size(s: &str) -> Option<(usize, usize)> {
    let index = s.find('x')?;
    let width = &s[..index].parse().ok()?;
    let height = &s[index + 1..].parse().ok()?;
    Some((*width, *height))
}

fn parse_rect(s: &str) -> Option<(f32, f32, f32, f32)> {
    let split = s
        .split(':')
        .map(|x| x.parse::<f32>())
        .collect::<Result<Vec<_>, _>>()
        .ok()?;

    if split.len() != 4 {
        return None;
    }

    Some((split[0], split[1], split[2], split[3]))
}
