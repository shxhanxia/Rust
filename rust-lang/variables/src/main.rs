// fn main() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }
// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// // 常量
// fn main() {
//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
// }

// 隐藏
// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
// }

// 复用
// fn main() {
//     let spaces = "   ";
//     let spaces = spaces.len();
// }

// 不能改变变量的类型
// fn main() {
//     let mut spaces = "   ";
//     spaces = spaces.len();
// }

// 使用parse将String转化为数字时, 必须增加类型注释
// fn main() {
//     let guess: u32 = "42".parse().expect("Not a number!");
// }

// 浮点型
// fn main() {
//     let x = 2.0; // f64

//     let y: f32 = 3.0; // f32
// }

// 数值运算
// fn main() {
//     // addition
//     let sum = 5 + 10;

//     // subtraction
//     let difference = 95.5 - 4.3;

//     // multiplication
//     let product = 4 * 30;

//     // division
//     let quotient = 56.7 / 32.2;
//     let floored = 2 / 3; // Results in 0

//     // remainder
//     let remainder = 43 % 5;
// }

// 布尔型
// fn main() {
//     let t = true;

//     let f: bool = false; // with explicit type annotation
// }

// 字符类型
// fn main() {
//     let c = 'z';
//     let z: char = 'ℤ'; // with explicit type annotation
//     let heart_eyed_cat = '😻';
// }

// 元组类型
// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
// }

// 模式匹配 结构
// fn main() {
//     let tup = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     println!("The value of y is: {y}");
// }

// 点索引
// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let five_hundred = x.0;

//     let six_point_four = x.1;

//     let one = x.2;
// }

// 数组类型
// fn main() {
//     let a = [1, 2, 3, 4, 5];
// }

// 简写
// fn main() {
//     let a = [3; 5];
// }

// 访问数组元素
// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     let first = a[0];
//     let second = a[1];
// }

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

