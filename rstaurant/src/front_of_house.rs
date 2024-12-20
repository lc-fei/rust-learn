// 餐厅前厅，用于吃饭
// 为他创建三个模块
// pub mod front_of_house {
    pub mod hosting {
        // 如果要对外暴露方法，不只是包要暴露，函数也要利用pub修饰暴露

        // 这个对枚举不适用，因为枚举一定要与其所有的内部字段配合使用，所以说，枚举加上pub之后，其内部的所有字段就都对外暴露了
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
// }