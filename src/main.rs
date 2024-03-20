use num_complex::Complex;

// 引入 web.rs 模块
mod web;
mod option;
mod escape_time;
mod command_line_parameters;

fn main() {
    println!("Jess Demo!");
    web::run();
    println!("{:?}", escape_time::escape_time(Complex::new(3.00, 2.00), 5).unwrap());
}
