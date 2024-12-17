pub fn tf () {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", five);
    println!("{:?}", six);
    println!("{:?}", none);

    if let Some(3) = six {
        println!("这样也行？？？");
    }
    if let Some(i) = none {
        println!("{}", i);
    } 
    // Vec是动态数组
    let mut stack = Vec::new();

    // 向数组尾部插入元素
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // stack.pop从数组尾部弹出元素
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }


    // if let
    // while let
    // let-else

    fn catchIsI32(i: Option<i32>) {
        let Some(a) = i else {
            println!("匹配失败");
            return; // let else不发散，所以要在后面加上return或者其他语句跳出作用域，如果不加上return;会导致程序不终止。
        };
        println!("匹配成功")
    }

    catchIsI32(five);
    catchIsI32(none);

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

        // 绑定新变量 `p`，同时对 `Point` 进行解构
        let p @ Point {x: px, y: py } = Point {x: 10, y: 23};
        println!("x: {}, y: {}", px, py);
        println!("{:?}", p);


        let point = Point {x: 10, y: 5};
        if let p @ Point {x: 10, y} = point {
            println!("x is 10 and y is {} in {:?}", y, p);
        } else {
            println!("x was not 10 :(");
        }
}