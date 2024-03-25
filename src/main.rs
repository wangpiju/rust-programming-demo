use num_complex::Complex;

// 引入 web.rs 模块
mod web;
mod option;
mod escape_time;
mod command_line_parameters;
mod float;
mod array;

fn main() {
    println!("Jess Demo!");
    //float::compare_float();
    //web::run();
    //println!("{:?}", escape_time::escape_time(Complex::new(3.00, 2.00), 5).unwrap());
    println!("{:?}", array::array_demo());
}
