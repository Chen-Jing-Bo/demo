
mod part3;
mod part4;
use part4::traffic_light::Light;


fn main() {
    let mut arr = vec![-1, 3, 1, 2, 8, 3, 2, 9, 0];
    crate::part3::sort::array_sort1(&mut arr);
    println!("{:?}", arr);
    
   let red_light=crate::part4::traffic_light::Color::RED;
   let green_light=crate::part4::traffic_light::Color::GREEN;
   let yellow_light=crate::part4::traffic_light::Color::YELLOW;
   let red_time = red_light.show_time();
   let green_time =green_light.show_time();
   let yellow_time = yellow_light.show_time();
   println!("red light tiem is {}s",red_time);
   println!("green light tiem is {}s",green_time);
   println!("yellow light tiem is {}s",yellow_time);


//    let list:Vec<u32> = vec![1,2,3,4,5,6];
   let list:Vec<u32> = vec![999999999,999999999,999999999,999999999,999999999];
   let sum = crate::part4::sum::get_sum(list);
   match sum {
    None => print!("sum is none"),
    Some(sum) => print!("sum is {}",sum)
   }

}
