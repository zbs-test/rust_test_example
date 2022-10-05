//这是一个rust语法测试文件,参考代码来源于菜鸟教程(www.runnoob.com),基于个人理解,会有错误的地方,还请包涵和指正,谢谢！
//这个文件的编排方式不是很好(含有很多shadowing)，几乎全是通过代码理解内容，所以还请耐心一下，本文件旨在快速编写rust代码，至于具体的理解还得看官方文档(虽然我不看)
fn main() {
    let s = "mysql"; //这是一个str类型,即切片,切片是引用别的变量名实现(let s = &string_name[0..1])的或者是直接赋值,线性结构支持切片,如数组,与String类型区别
    let s = 10; //shadowing 必须使用let,上面的所有s 将不可用
    println!("{s}");
    const i: u8 = 20; //这是定义常量的用法
                      //   let i:u8 = 100; //更一般的用法,与上句效果一样，但不可再次对变量名i 进行shadowing
    println!("{i}");
    //元组()
    let tup: (u16, u8) = (90, 70);
    println!("{}", tup.1);
    println!("{:?}", tup); //复合类型的打印都是{:?}
    let (x, y) = tup;
    println!("{x} {y}");
    //数组
    let months = ["jan", "sep"];
    println!("{:?}", months);
    println!("{}", months[1]);
    //函数嵌套定义
    fn add(a: u8, b: u8) -> u8 {
        a + b //以表达式的方式返回
    }
    println!("{}", add(1, 2));
    //函数不支持匿名函数
    fn sum(a: u8, b: u8) -> u8 {
        return a + b; //以return 的方式返回
    }
    println!("{}", sum(10, 20));
    //条件语句
    let num = 5;
    if num < 10 {
        println!("num < 10");
    } else if num > 10 {
        println!("num > 10");
    } else {
        println!("num = 10");
    }
    //对c++三元运算符的变式
    let a = 3.14;
    //如果a>3.145,将1赋值给num
    let num = if a > 3.145 { 1 } else { -1 };
    //循环体
    let mut num = 1;
    while num != 4 {
        println!("{num}");
        num += 1; //在非for循环里不支持++或--操作
    }
    //没有do-while
    //没有c++式的for三段式
    //for 这样用
    let a = [1, 2, 3, 4, 5];
    //迭代器是从a里面取它的值给j的
    for j in a.iter() {
        //这里用j,避免上面的const i 干扰
        println!("{j}");
    }
    let b = ["wo", "zhen", "de", "bu", "hui", "rust-lang"];
    for j in b.iter() {
        //这两种打印有差别
        println!("{:?}", j);
        println!("{}", j);
    }
    //这个仅仅是让j从0..b.len()，即1..6，这个是遵循左闭右开的
    for j in 0..b.len() {
        println!("{}", b[j]);
    }
    //while(true)的替代品
    let s = ['R', 'U', 'S', 'T'];
    let mut j = 0;
    loop {
        //让ch shadowing 式地被s数组里的元素赋值
        let ch = s[j];
        if ch == 'T' {
            //当ch=='T',跳出循环跑路,如果这个比较改变，即让ch=='P',那么可能造成死循环
            break;
        }
        println!("{}", ch);
        j += 1;
        if j == 3 {
            //显示地loop 停止在j==4的条件上
            break;
        }
    }
    //loop 也可以返回值
    let s = ['R', 'U', 'S', 'T'];
    let mut j = 0;
    let location = loop {
        let ch = s[j];
        if ch == 'T' {
            //如果ch=='T'时，将此时的j的值返回给location
            break j;
        }
        j += 1;
    };
    println!("char 'T' 的location是{}", location);
    println!("下面开始新的概念--------");
    //所有权
    // rust 每个值都要有一个变量/不可变量去拥有它
    // 值一次只能被一个变量名拥有
    // 变量名退出其作用范围则该变量名将被删除
    {
        //这是一个域，与java类似
        let s = "hello world"; //变量名与值"hello world"绑定,此前此域s是不可见,此后s会在此域可见,但不保证会不会有shadowing等操作使其不可用
                               //...
    }
    //变量名与数据的交互方式为:移动move 和 克隆clone
    let x = 5;
    let y = x; //对基本数据类型就是移动,或称为直接复制(数据在Stack定义的,即数据藏在Stack中)
    println!("{x}"); //此时x依然可用
                     // “只因”本数据类型有
                     //  所有整数 u8,i32,i64...
                     //  所有bool
                     //  所有浮点 f32,f64...
                     //  所有char
                     //  仅包含以上数据类型的元组
                     // 对复合数据类型或高级数据结构
    let s1 = String::from("hello"); //数据在Heap中定义的,即数据藏在Heap中

    let s2 = s1; //这种方式也叫 move
                 //此时和此后s1是不可用的了
                 // println!("{:?}",s1);
                 // 克隆
    let s1 = String::from("hello");
    let s2 = s1.clone(); //克隆s1的值给s2
    println!("{s1} {s2}"); //此时两个变量s1,s2都能用

    // 涉及函数的所有权机制---样例1
    println!(" 涉及函数的所有权机制---样例1");
    //对Heap的数据的所有者,即变量名s
    let s = String::from("golang");
    // s 被声明有效
    takes_ownership(s);
    // s 的值被当作参数传入函数
    // println!("{}",s);
    // 所以可以当作 s 已经被移动，从这里开始已经无效
    //即当s使用非借用式传参时,s变量名被 move 到那个函数里了

    //对Stack的数据的所有者
    let x = 5;
    make_copy(x); //把x非借用式传参了
    println!("{x}"); //此时x依然可用

    // 函数返回值的所有权机制---样例2
    println!("函数返回值的所有权机制---样例2");
    let s1 = gives_ownership();
    // gives_ownership 移动它的返回值到 s1
    println!("{:?}", s1); //s1是获得函数gives_ownership()传出的some_string的所有权,some_string被move,不可用了
    let s2 = String::from("hello");
    // s2 被声明有效

    let s3 = takes_and_gives_back(s2);
    // s2 被当作参数移动, s3 获得返回值所有权
    // println!("{:?}",s2); s2不能用了，此时s2被移动到s3
    println!("{:?}", s3); //s3是可用的   //一般来说,出来main函数, s3 无效了,会被释放,s1 也无效了,会被释放.

    ///引用与租借---样例3
    println!("引用与租借---样例3");
    //  & 运算符可以取变量的"引用"，当一个变量的值被引用时，变量本身不会被认定无效。因为"引用"并没有在栈中复制变量的值：
    //  引用不会获得值的所有权。
    //  引用只能租借（Borrow）值的所有权。
    //  引用本身也是一个类型并具有一个值，这个值记录的是别的值所在的位置，但引用不具有所指值的所有权：
    let s1 = String::from("hello");

    let len = calculate_length(&s1); //一旦作为参数传递了，就会发生所有权的移动（Move）或借用（Borrow）

    println!("The length of '{}' is {}.", s1, len);

    let s1 = String::from("world");
    let mut s2 = &s1;
    println!("{:?}", s1); //s2借用s1,此时s1仍可用
    let s3 = s1;
    // println!("{:?}", s1); //s1移动到s3,此时s1不可用了,
    s2 = &s3; // 重新从 s3 租借所有权
    println!("{}", s2);

    let s1 = String::from("run");
    let s2 = &s1;
    println!("{}", s2);
    // s2.push_str("oob"); // 错误，禁止修改租借的值
    println!("{}", s2);

    let mut s1 = String::from("run");
    // s1 是可变的

    let s2 = &mut s1; // s2 是可变的引用,则可通过可变的引用s2对s1修改数值

    s2.push_str("oob");
    println!("{}", s2);
    println!("{}", s1); //s1依然可用，所以当有&的时候，效果是所有权没有被移动，该行为成为borrow
    println!("结构体--------------");
    struct Foo {
        name: String,   //这个name,domain叫做该结构体的字段,用,结尾
        domain: String, //结构体也能定义引用型数据类型,如str
    }
    let f = Foo {
        name: String::from("java"),
        domain: String::from("lua"),
    };
    println!("{:?}", f.name); //对Foo结构体实例f内的name字段打印
    let cp_f = Foo {
        domain: String::from("copy f.name to cp_f"),
        ..f //对f里的其它字段值所有权移动到这里
    };
    println!("{:?}", cp_f.name); //此时f.name不可用了

    //元组结构体
    #[derive(Debug)] //想要对Color元组结构体打印，必须在它的前面1行导入该调试库
    struct Color(u8, i32, f64);
    let c = Color(9, 70i32, 9.10f64);
    println!("{}  c:{:?}", c.0, c); //此时{:?}才可以用作对元组结构体c打印

    //结构体方法(含&self参数)和关联函数(不含&self参数)，暂时不讲，看几个例子就会了

    //枚举类，Book种类
    println!("枚举类----------");
    #[derive(Debug)]
    enum Book {
        Papery(u32), //纸质书,u32是属性类型，可去掉
        Electronic(String),
        Muti { index: u32 }, //甚至可以为属性类型命名index
    }
    let book = Book::Papery(1001); //1001是属性值
    let book = Book::Electronic(String::from("nothing"));
    let book = Book::Muti { index: 007 }; //以上我多次shadowing了，只保留最后的值
    println!("{:?}", book);
    println!("{:?}", Book::Papery(10));
    //match语句
    match book {
        //属性不具名的
        Book::Papery(u32) => {
            println!("papery book");
        }
        Book::Electronic(String) => {
            println!("ebook");
        }
        //对属性具名的
        Book::Muti { index } => {
            println!("muti book {}", index);
        }
        _ => {} //相当于default
    }
    //io操作和文件操作
    use std::io::stdin;
    let mut str_buf = String::new();
    stdin().read_line(&mut str_buf).expect("failed to type in"); //输入操作,链式调用expect,可理解为错误处理
    println!("{}", str_buf);
    //字符串不是这么比较的
    if str_buf == "hello" {
        println!("welcome");
    } else {
        println!("good day");
    }
    //文件
    use std::fs;
    let content = fs::read("T:/.vscode/tasks.json").unwrap();
    println!("{:?}", content);
    //以缓冲的形式读取，5个5个的来读取
    use std::io::prelude::*;
    let mut buffer = [0u8; 5];
    let mut file = fs::File::open("T:/.vscode/settings.json").unwrap();
    file.read(&mut buffer).unwrap();
    println!("{:?}", buffer);
    file.read(&mut buffer).unwrap();
    println!("{:?}", buffer);

    //创建文件
    use std::fs::File;
    fs::write(
        "./a.txt",
        "this is a string I want to type in the  file a.txt",
    )
    .unwrap(); //
    let mut file = File::create("./b.txt").unwrap();
    file.write(b"this is another method to create a file b.txt, Using File::create(\"filename\") to create a file, and Using write(\"some strings\") to write in file")
    .unwrap();
}

//涉及函数的所有权机制---样例1
fn takes_ownership(some_string: String) {
    // 一个 String 参数 some_string 传入，此时有效
    println!("{}", some_string);
} // 函数结束, 参数 some_string 在这里释放
fn make_copy(some_integer: i32) {
    // 一个 i32 参数 some_integer 传入，此时有效
    println!("{}", some_integer);
} // 函数结束, 参数 some_integer 是基本类型, 无需释放some_integer

// 函数返回值的所有权机制---样例2
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    // some_string 被声明有效

    return some_string;
    // some_string 被当作返回值移动出函数
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string 被声明有效

    a_string // a_string 被当作返回值移出函数
}

///引用与租借---样例3
fn calculate_length(s: &String) -> usize {
    s.len()
}
