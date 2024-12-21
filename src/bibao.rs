pub fn bibaof () {
    let x = 123;

    //error: 无法在 fn 项目中捕获动态环境 改用 `|| { ... }` 闭包形式
    // fn add(a:i64) -> i64 {
    //     a + x
    // }

    let add = |a: i32|  -> i32 {
        a + x
    };
    let res = add(123);
    println!("{}", res);


    fn fn_once<F>(func: F)
    where
        F: FnOnce(usize) -> bool,
    {
        println!("{}", func(3));
    }
    let x = vec![1, 2, 3];
    fn_once(|z|{z == x.len()});


    let mut s = String:: new();
    let mut update_string = |str| {
        s.push_str(str);
    };

    // 这里为什么是mut呢，因为这里的闭包是FnMut，可能会改变自己活着是捕获的环境（当然，这里主要的是捕获的环境）
    fn exec<'a,F>(f: &mut F)
        where F: FnMut(&'a str)
    {
        f("das46das465das465das");
    }

    exec(&mut update_string);
    exec(&mut update_string);
    println!("{}", s);



    fn factory(x:i32) -> Box<dyn Fn(i32) -> i32> {
        let num = 5;
        if x > 5 {
            return Box::new(move |x| x + num);
        } else  {
            return Box::new(move |x| x - num);
        }
    }
    
    let f = factory(1);
    
    let answer = f(1);
    assert_eq!(6, answer);
}