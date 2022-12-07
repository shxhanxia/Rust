// fn main() {
//     {                      // s 在这里无效, 它尚未声明
//         let s = "hello";   // 从此处起，s 是有效的

//         // 使用 s
//     }                      // 此作用域已结束，s 不再有效
// }

// String类型
// fn main() {
//     let mut s = String::from("hello");

//     s.push_str(", world!"); // push_str() 在字符串后追加字面值

//     println!("{}", s); // 将打印 `hello, world!`
// }

// fn main() {
//     {
//         let s = String::from("hello"); // 从此处起，s 是有效的

//         // 使用 s
//     }                                  // 此作用域已结束，
//                                        // s 不再有效
// }

// fn main() {
//     let x = 5;
//     let y = x;

//     println!("{}, {}", x, y);

//     let a = "123";
//     let b = a;

//     println!("{}, {}", a, b);

//     // let s1 = String::from("hello");
//     // let s2 = s1;

//     // println!("{}, {}", s1, s2);

//     let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("s1 = {}, s2 = {}", s1, s2);
// }


// 所有权与函数
// fn main() {
//     let s = String::from("hello");  // s 进入作用域

//     takes_ownership(s);             // s 的值移动到函数里 ...
//                                     // ... 所以到这里不再有效

//     let x = 5;                      // x 进入作用域

//     makes_copy(x);                  // x 应该移动函数里，
//                                     // 但 i32 是 Copy 的，
//                                     // 所以在后面可继续使用 x

// } // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
//   // 没有特殊之处

// fn takes_ownership(some_string: String) { // some_string 进入作用域
//     println!("{}", some_string);
// } // 这里，some_string 移出作用域并调用 `drop` 方法。
//   // 占用的内存被释放

// fn makes_copy(some_integer: i32) { // some_integer 进入作用域
//     println!("{}", some_integer);
// } // 这里，some_integer 移出作用域。没有特殊之处


// 返回值与作用域
// fn main() {
//     let s1 = gives_ownership();         // gives_ownership 将返回值
//                                         // 转移给 s1

//     let s2 = String::from("hello");     // s2 进入作用域

//     let s3 = takes_and_gives_back(s2);  // s2 被移动到
//                                         // takes_and_gives_back 中,
//                                         // 它也将返回值移给 s3
// } // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
//   // 所以什么也不会发生。s1 离开作用域并被丢弃

// fn gives_ownership() -> String {             // gives_ownership 会将
//                                              // 返回值移动给
//                                              // 调用它的函数

//     let some_string = String::from("yours"); // some_string 进入作用域.

//     some_string                              // 返回 some_string 
//                                              // 并移出给调用的函数
//                                              // 
// }

// // takes_and_gives_back 将传入字符串并返回该值
// fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
//                                                       // 

//     a_string  // 返回 a_string 并移出给调用的函数
// }

// 用元组来返回多个值
// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{}' is {}.", s2, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() 返回字符串的长度

//     (s, length)
// }

// 引用
// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize { // s是String的引用
//     s.len()
// } // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
//   // 所以什么也不会发生


// 尝试修改借用的值
// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// 可变引用
fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s); // hello, world
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}



