// mod语法，用于声明一个模块或者将一个模块引入到当前作用域中
mod front_of_house;

// use语法，用于将一个模块路径引入当前作用域，一般用来简化模块的引用
pub use crate::front_of_house::hosting;

// mod和use一定要配合使用，或者只用mod
// mod的作用是将现有模块的内容引入到当前作用域，告诉编译器去寻找并加载这个文件夹内的内容
// use的作用是将一个已经定义和加载的模块中引入某些特定的路径或内容到当前作用域，方便再次使用的时候不用写完整路径

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}