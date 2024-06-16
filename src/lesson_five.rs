use std::fmt::{Display, Formatter};

mod lesson_five {
    /*
    Rust 并没有一个与 -> 等效的运算符；相反，Rust 有一个叫 自动引用和解引用（automatic referencing and dereferencing）的功能。
    方法调用是 Rust 中少数几个拥有这种行为的地方。

    它是这样工作的：当使用 object.something() 调用方法时，Rust 会自动为 object 添加 &、&mut 或 * 以便使
    object 与方法签名匹配。也就是说，这些代码是等价的：

    p1.distance(&p2);
    (&p1).distance(&p2);

    第一行看起来简洁的多。这种自动引用的行为之所以有效，是因为方法有一个明确的接收者———— self 的类型。
    在给出接收者和方法名的前提下，Rust 可以明确地计算出方法是仅仅读取（&self），做出修改（&mut self）
    或者是获取所有权（self）。事实上，Rust 对方法接收者的隐式借用让所有权在实践中更友好
    */
}

pub(crate) fn lesson1() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle1 is {} square pixels.", area(&rect1));
    println!("The area of the rectangle2 is {} square pixels.", rect1.area());

    let sq1 = Rectangle::square(3);
    let sq2 = Rectangle::square(5);
    println!("{}", sq2.can_hold(&sq1));

    println!("sq1-->{:?}", sq1);
    println!("sq2-->{:?}", sq2);
    println!("sq2-->{:?}", sq2);

    let s = String::from("hello");
    // print_str(s);
    println!("{}", s);
    println!("{}", s);
}

fn print_str(string: String){
    println!("{string}");
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}