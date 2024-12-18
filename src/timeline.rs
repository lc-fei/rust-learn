pub fn tlf() {
    // 悬垂引用
    // 长声明周期变量引用短声明周期变量
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    //     println!("{}{}", x, r);
    // }
    // println!("{}", r)

    // 函数中的声明周期
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // 和泛型一样，要使用声明周期函数，需要先声明'a
    // 这里的声明周期‘a是x、y两个变量声明周期的较小值
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }


    // 结构体中的声明周期
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    let i;
    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        i = ImportantExcerpt {
            part: first_sentence,
        };
        println!("{:?}",i);
    }

    use std::fmt::Display;

    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}