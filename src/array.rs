pub fn af() {
    // Option枚举就如其名，表示可选，在这里就表示u8 ｜ None，要么没值，要么就是u8类型
    // Option枚举
    // 思想：Option如果有值 =》 一切正常，如果没有值，可以通过模式匹配来处理
    let v: Option<u8> = Some(3);
    if let Some(3) = v {
        println!("123")
    }
}