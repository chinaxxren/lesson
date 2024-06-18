
mod lesson_rc {
    // 注意 Rc<T> 只能用于单线程场景
    // 因为 Rc<T> 内部使用了一个可变的引用计数器，变量拥有某个值。然而，有些情况单个值可能会有多个所有者
    // 在 “避免引用循环：将 Rc<T> 变为 Weak<T>” 部分会讲解 weak_count 的用途。
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub(crate) fn lesson1() {
    use crate::lesson_rc::List::{Cons, Nil};
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));

    let b = Cons(3, Box::new(a));
    // 编译错误，因为 a 已经移动到 b 了,展示不能用两个 Box<T> 的列表尝试共享第三个列表的所有权
    // let c = Cons(4, Box::new(a));
}

enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

use std::rc::Rc;

pub(crate) fn lesson2() {
    use crate::lesson_rc::RcList::{Cons, Nil};
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    // 调用 a.clone() 而不是 Rc::clone(&a)，不过在这里 Rust 的习惯是使用Rc::clone。
    // Rc::clone 的实现并不像大部分类型的 clone 实现那样对所有数据进行深拷贝。
    // Rc::clone 只会增加引用计数
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}

pub(crate) fn lesson3() {
    use crate::lesson_rc::RcList::{Cons, Nil};

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}