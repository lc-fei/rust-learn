use std::ops::Add;

pub fn gtf() {

    // 这里的PartialOrd和Copy是两个特征
    // 另外，关于这个+表示要同食符合多个特征约束
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
    
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
    
        return largest;
    }
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    fn add<T> (a: T, b: T) -> T 
    where T: Add<Output = T>
    {
        return a + b;
    }

    println!("{}", add(1, 5));


    fn logg<T: std::fmt::Debug, const N:usize>(arr: [T; N]) {
        println!("{:?}", arr)
    }

    logg([1,2,4,5,6,7,8,9,36,3,12,4,8,5]);

    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T
    }

    impl<T: Add<T, Output = T>> Add for Point<T>{
        type Output = Point<T>;

        // 下面这里要加上Self，表示Self中的Output类型别名，而不是其他的
        fn add(self, p: Point<T>) -> Self::Output {
            return Point {
                x: self.x + p.x,
                y: self.y + p.y
            }
        }
    }

    let p1 = Point{
        x: 1,
        y: 2
    };
    let p2 = Point{
        x: 3,
        y: 4
    };

    
    println!("{:?}",add(p1, p2));



    // 特征对象
    println!("特征对象");


    // 安全的特征才能拥有特征对象
    // 1. 方法的返回类型不能是self
    // 2. 方法没有任何泛型参数
    pub trait Draw {
        fn draw(&self) ;
    }
    #[derive(Debug)]
    struct Button {
        width: u32,
        height: u32,
        label: String
    }
    impl Draw for Button {
        fn draw(&self) {
            println!("{:?}", &self);
            println!("{}{}{}", &self.height, &self.label, &self.width)
        }
    }

    #[derive(Debug)]
    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }
    
    impl Draw for SelectBox {
        fn draw(&self) {
            // 绘制SelectBox的代码
            println!("{:?}", &self);
            println!("{}{:?}{}", &self.height, &self.options, &self.width)
        }
    }

    let mut components_list: Vec<Box<dyn Draw>> = Vec::new();
    let arr = [String::from("456"), String::from("789")];
    let selebox = SelectBox {
        width: 12,
        height: 13,
        // options: vec!arr // 不呢个这样用。vec！期待一个元素，而不是变量
        options: arr.to_vec()
    };
    let but = Button {
        width: 4,
        height: 5,
        label: String::from("68888")
    };
    components_list.push(Box::new(selebox)); // 特征对象
    components_list.push(Box::new(but)); // 特征对象

    for component in components_list {
        component.draw();
    }
    
}