use crate::lesson_box::List::{Cons, Nil};

mod lesson_box {
    // string vect就是智能指针
    // 最简单直接的智能指针是 box，其类型是 Box<T>。
    // box 允许你将一个值放在堆上而不是栈上。留在栈上的则是指向堆数据的指针

    //Rust 在发现类型和 trait 实现满足三种情况时会进行 Deref 强制转换：
    //当 T: Deref<Target=U> 时从 &T 到 &U。
    //当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U。
    //当 T: Deref<Target=U> 时从 &mut T 到 &U。
}
// #[derive(Debug)]
// enum List {
//     Cons(i32, List),
//     Nil,
// }

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub(crate) fn lesson1() {
    // box 例子
    // 这里，b 是一个包含 5 的 Box 的指针。
    // 这里，b 是一个指向堆上 5 的指针。
    let mut b = Box::new(5);
    println!("b = {}", b);

    // box 修改b值
    *b = 6;
    println!("b = {}", b);

     // 这个类型 “有无限的大小”。其原因是 List 的一个成员被定义为是递归的：
    // 它直接存放了另一个相同类型的值。这意味着 Rust 无法计算为了存放 List
    // 值到底需要多少空间。让我们拆开来看为何会得到这个错误。首先了解一下
    // Rust 如何决定需要多少空间来存放一个非递归类型
    // let list = Cons(1, Cons(2, Cons(3, Nil)));

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("{:?}", list);

    let x = 5;
    let y = &x;
    println!("{} {}", x, *y);

    let x = 5;
    let y = Box::new(x);

    // *(y.deref())
    println!("{} {}", x, *y);
}