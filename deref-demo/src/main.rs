fn display(s: &str) {
    println!("{}", s);
}

fn main() {
    // case 1: 自动Deref解引用
    let s = String::from("Hello");
    // 注意：这里传入的&s的数据类型是&String, 而display函数定义的参数类型是&str
    // 实现从&String -> &str类型之间的转换的背后，就是Rust编译通过Deref自动解引用帮我们做的
    // 也就是在标准库中实现String: Deref<&str>的Deref trait，标准库中的代码如下：
    // #[stable(feature = "rust1", since = "1.0.0")]
    // impl ops::Deref for String {
    // type Target = str;
    // #[inline]
    // fn deref(&self) -> &str {
    //     unsafe { str::from_utf8_unchecked(&self.vec) }
    // }
    // }
    display(&s);

    // case 2: 连续自动Deref解引用
    let b = Box::new(String::from("World"));
    // 注意：1. 首先&b是&Box<String>类型，通过它的Deref解引用，可以获取到&String类型引用
    //      2. 然后在连续自动的调用&String -> &str的Deref解引用
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     impl<T: ?Sized, A: Allocator> Deref for Box<T, A> {
    //     type Target = T;
    //     fn deref(&self) -> &T {
    //         &**self // 这里为什么要用两个*？因为第一个*是解&Box<String>, 第2个是解Box<String>得到String类型
    //     }
    // }
    display(&b);
}
