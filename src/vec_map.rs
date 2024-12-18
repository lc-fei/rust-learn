pub fn vmf () {
    use std::collections::HashMap;

    let teams_list = vec![
        ("中国队", 123),
        ("美国队", 456)
    ];

// 这里主要是要注意一下collect方法，在内部支持生成多种类型的目标集合，所以要提前给map设定类型，可以用<_, _>占位
    let map:HashMap<_, _> = teams_list.into_iter().collect();

    println!("{:?}", map);


    let name = String::from("Sunface");
    let age = 18;

    let mut handsome_boys = HashMap::new();
    handsome_boys.insert(name, age);


    // println!("因为过于无耻，{}已经被从帅气男孩名单中除名", name); // 注意这里的所有权问题，name的所有权已经交给了handsom_boys
    // 另外，如果要因为所有权问题去将引用交给map的时候，注意String的生命周期至少要比map长，不然会报错
    println!("还有，他的真实年龄远远不止{}岁", age);



}