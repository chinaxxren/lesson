mod lesson_six {}

pub(crate) fn lesson1() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    // 使用 & 和 [] 会得到一个索引位置元素的引用,[] 方法，当引用一个不存在的元素时 Rust 会造成 panic
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // 使用索引作为参数调用 get 方法时，会得到一个可以用于 match 的 Option<&T>
    // 当 get 方法被传递了一个数组外的索引时，它不会 panic 而是返回 None
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // 一旦程序获取了一个有效的引用，借用检查器将会执行所有权和借用规则（第四章讲到）来确保 vector 内容的这个引用和任何其他引用保持有效
    // 在 vector 的结尾增加新元素时，在没有足够空间将所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。
    // 这时，第一个元素的引用就指向了被释放的内存。
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);

    // let mut v:Vec<i32> = Vec::new();
    // v.push(0);
    // v.push(1);
    // v.push(2);
    // v.push(3);
    //
    // let first = &v[0];
    // v.push(6);

    // println!("The first element is: {first}");

    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }

        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {announcement}");
            self.part
        }
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let im = ImportantExcerpt {
        part: first_sentence,
    };

    let str = im.announce_and_return_part("hello");
    println!("{str}");
    println!("{:?}", im);

    let s: &'static str = "I have a static lifetime.";
    println!("{s}");

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }

}