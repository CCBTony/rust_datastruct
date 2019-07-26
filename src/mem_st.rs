/// 内存管理教学
pub mod mem_study {
    /// move 语义示例
    pub fn take_ownership_and_show(src: String) {
        println!("The string is {}, I take its ownership, and it will be released before I return", src)
    }

    /// borrow 语义示例
    pub fn borrow_and_show(src: &String) {
        println!("The string is {}, I just borrow it", src)
    }

    /// lifetime 示例
    pub fn lifetime_show<'a>(s1: &'a String, s2: &'a String) -> &'a String {
        if s1.len() > s2.len() {
            s1
        } else {
            s2
        }
    }

    pub fn test_lifetime() {
        let s1 = String::from("tony");
        let s2 = String::from("gzj");
        let s3;
        s3 = lifetime_show(&s1, &s2);

        println!("the longer is {}", s3)
    }

    pub fn test_ownership() {
        let stra = String::from("取得拥有权");
        let strb = String::from("借用一下");

        take_ownership_and_show(stra);
        // stra 的所有权被 take_ownership_and_show 内的作用域取得，
        // 随着调用结束，占用的内存被释放，所以外界不能再次访问 stra，否则编译器报错
         //println!("stra: {}", stra);

        borrow_and_show(&strb);
        println!("strb: {}", strb);
    }
}
