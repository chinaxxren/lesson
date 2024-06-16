mod lesson_two {
    // 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
    // 引用必须总是有效的。
}

pub(crate) fn lesson1() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

pub(crate) fn lesson6() {
    let str = "hello";
    let reference_to_nothing = dangle();
}

//悬垂引用（Dangling References）
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
fn dangle() -> String {
    let s = String::from("hello");
    s
}

pub(crate) fn lesson5() {
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{}, {}", r1, r2);

    let r3 = &mut s; // 大问题
    println!("{}", r3);
    // println!("{}, {}, and {}", r1, r2, r3);
}

pub(crate) fn lesson4() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;
}

pub(crate) fn lesson3() {
    let mut s = String::from("hello");

    let r1 = &mut s;

    // error
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);
}

pub(crate) fn lesson2() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}