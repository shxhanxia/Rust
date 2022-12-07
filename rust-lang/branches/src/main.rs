// fn main() {
//     let number = 7;

//     if number {
//         println!("condition is true");
//     } else {
//         println!("condition is false");
//     }
// }

// fn main() {
//     let number = 3;

//     if number != 0 {
//         println!("number was something other than zero");
//     }
// }

// 使用else if处理多重条件
// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// 在let语句中使用if
fn main() {
    let condition = true;
    // let number = if condition { 5 } else { 6 };
    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}
