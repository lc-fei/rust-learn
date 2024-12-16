pub fn bf() {
    // let x = 5;
    // let y = &x;

    // fn a(i: i32) {
    //     println!("{}", i)
    // }
    // a(x);
    // a(*y);

    // fn first_word(s: &String) -> &str {
    //     &s[..1]
    // }
    // let mut s = String::from("hello world");

    // let word = first_word(&s);


    // println!("the first word is: {}", word);

    // s.clear();

    // let a = "hello world;";
    // let b = a;
    // println!("{}{}", a, b);
    // let mut str = String::from("abc");
    // if true {
    //     let c = &str[..];
    //     let d = c;
    //     println!("{}{}", c, d);
    //     str.clear();
    // }
    #[derive(Debug)]
 struct File {
   name: String,
   data: Vec<u8>,
 }

   let f1 = File {
     name: String::from("f1.txt"),
     data: Vec::new(),
   };

   let f1_name = &f1.name;
   let f1_length = &f1.data.len();

   println!("{:?}", f1);
   println!("{} is {} bytes long", f1_name, f1_length);
}