pub fn hr() {
    // let s1 = String::from("123");
    // let s2 = s1;
    // println!("{s1}", s1); // 会报错，value borrowed here after move

    let s3: &str = "12345";
    let s4: &str = s3;
    println!("{}", s4); // 可以正常输出
}