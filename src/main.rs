use num_complex::Complex;

// 引入 web.rs 模块
mod web;
mod option;
mod escapeTime;

fn main() {
    println!("Jess Demo!");
    //web::run();
    println!("{:?}", escapeTime::escape_time(Complex::new(3.00, 0.00), 5).unwrap());
}
