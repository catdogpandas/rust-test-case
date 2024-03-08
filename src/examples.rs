/// 循环中的堆空间申请
pub fn alloc_in_loop() {
    let mut v: Vec<Box<i32>> = Vec::new();
    for _ in 0..5 {
        let mut tmp_heap_alloc = Box::new(1);
        // v.push(tmp_heap_alloc); 接受所有权
    }
}

/// 释放后使用漏洞
pub fn use_after_free() {
    let v = {
        let mut s = vec![1, 2];
        let ptr = s.as_mut_ptr();
        unsafe { Vec::from_raw_parts(ptr, s.len(), s.len()) }
    };
    println!("v: {:?}", v);
}
/// 多重释放漏洞
pub fn double_free() {
    let mut a = vec![1, 2];
    let ptr = a.as_mut_ptr();
    unsafe {
        let mut _v = Vec::from_raw_parts(ptr, 2, 2);
    }
}

/// unsafe代码块
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

/// traits
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}", self.author,)
    }
}

pub struct Tweet {
    pub username: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}", self.username)
    }
}

/// 静态分发
pub fn print_summary<T: Summary>(obj: T) {
    print!("Summary = {}", obj.summarize());
}
pub fn print_summary_news_article(obj: NewsArticle) {
    print!("Summary = {}", obj.summarize());
}
pub fn print_summary_tweet(obj: Tweet) {
    print!("Summary = {}", obj.summarize());
}

/// 动态分发
pub fn print_summary_dyn(obj: &dyn Summary) {
    print!("Summary = {}", obj.summarize());
}

/// 堆上动态trait对象
pub fn heap_dyn_trait_obj() {
    let mut heap_obj: Box<dyn Summary> = Box::new(NewsArticle {
        author: String::from("aaa"),
    });
    println!("{}", heap_obj.summarize());
}

/// 栈上动态trait对象
pub fn stack_dyn_trait_obj() {
    let mut heap_obj: &dyn Summary = &NewsArticle {
        author: String::from("aaa"),
    };
    println!("{}", heap_obj.summarize());
}

/// Box包装动态trait对象
// struct Box {
//     pointer: *mut dyn Summary,
// }

/// ownership
pub fn ownership() {
    let s = String::from("hello"); // s 进入作用域
    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以到这里不再有效
    let x = 5; // x 进入作用域
    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，
                   // 所以在后面可继续使用 x
} // 这里，x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 没有特殊之处

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。
  // 占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处

pub fn reference() {
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);
}

/// 闭包
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        .into_iter()
        .filter(|s| {
            return s.size == shoe_size;
        })
        .collect()
}

pub fn closures() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let take_ownership_closure = || {
        let list_in_closure = list;
    };
    take_ownership_closure();
    // println!("After defining closure: {:?}", list);
}
