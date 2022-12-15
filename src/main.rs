mod part3;
mod part4;

use part4::traffic_light::Light;

use crate::part4::shape::get_area;
use crate::part4::shape::Circle;
use crate::part4::shape::Square;
use crate::part4::shape::Triangle;
use crate::part4::traffic_light::Color as Co;

fn main() {
    // part3
    let mut arr = vec![-1, 3, 1, 2, 8, 3, 2, 9, 0];
    crate::part3::sort::array_sort1(&mut arr);
    println!("{:?}", arr);

    // part4
    let red_light = Co::RED;
    let green_light = Co::GREEN;
    let yellow_light = Co::YELLOW;
    let red_time = red_light.show_time();
    let green_time = green_light.show_time();
    let yellow_time = yellow_light.show_time();
    println!("red light tiem is {}s", red_time);
    println!("green light tiem is {}s", green_time);
    println!("yellow light tiem is {}s", yellow_time);

    //    let list:Vec<u32> = vec![1,2,3,4,5,6];
    let list: Vec<u32> = vec![999999999, 999999999, 999999999, 999999999, 999999999];
    let sum = crate::part4::sum::get_sum(list);
    match sum {
        None => print!("sum is none"),
        Some(sum) => print!("sum is {}", sum),
    }

    let circle = Circle { r: 1.1 };
    let circle_area = get_area(&circle);
    println!("circle area is {}", circle_area);

    let triangle = Triangle { a: 1.1, h: 2.2 };
    let triangle_area = get_area(&triangle);
    println!("triangle_area is {}", triangle_area);

    let square = Square { b: 1.1 };
    let square_area = get_area(&square);
    println!("square_area is {}", square_area);
}
