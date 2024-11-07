#[test]
fn test1() {
    println!("Success!");
}

#[test]
fn test2() {
    let s: &str = "hello, world";
    greetings(s)
}
fn greetings(s: &str) {
    println!("{}",s)
}

#[test]
fn test3() {
    let mut s = String::from("");
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");
    println!("{}", s);
    println!("Success!");
}

#[test]
fn test4() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s);
}

#[test]
fn test5() {
    let mut s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}

#[test]
fn test6() {
    let s1: String = String::from("hello,");
    let s2: String = String::from("world!");
    let s3: String = s1 + s2.as_str(); //String -> &str
    assert_eq!(s3, "hello,world!");
    println!("{}", s3);
}

#[test]
fn test7() {
    let s: &str = "hello, world";
    greetings_owned(String::from(s));
}

fn greetings_owned(s: String) {
    println!("{}", s);
}

#[test]
fn test8() {
    let s: String = "hello, world".to_string();
    let s1: &str = s.as_str();

    println!("Success!");
}

#[test]
fn test9() {
    // You can use escapes to write bytes by their hexadecimal values
    // Fill the blank below to show "I'm writing Rust"
    let byte_escape = "I'm writing Ru\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // ...Or Unicode code points.
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
             unicode_codepoint, character_name );

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}

#[test]
fn test10() {
    let s1: String = String::from("hi,中国");
    let h: &str = &s1[0..1]; // Modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(h, "h");

    let h1 = &s1[3..6]; // Modify this line to fix the error, tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(h1, "中");

    println!("{}", h1);

    println!("Success!");
}

#[test]
fn test11() {
    // Fill the blank to print each char in "你好，世界"
    for c in "你好，世界".chars() {
        println!("{}", c)
    }
}

#[test]
fn test12() {
    // Fill the blank with proper array type
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Modify the code below to make it work
    assert_eq!(arr.len(), 5);
    println!("Success!");
}

#[test]
fn test13() {
    // We can ignore parts of the array type or even the whole type, let the compiler infer it for us
    let _arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];
    // Fill the blank
    // Arrays are stack allocated, std::mem::size_of_val returns the bytes which an array occupies
    // A char takes 4 bytes in Rust: Unicode char
    assert_eq!(size_of_val(&arr), 12);
    println!("Success!");
}

#[test]
fn test14() {
    // Fill the blank
    let list: [i32; 100] = [1; 100] ;
    assert_eq!(list[0], 1);
    assert_eq!(list.len(), 100);
    println!("Success!");
}

#[test]
fn test15() {
    // Fix the error
    let _arr: [i32; 3] = [1, 2, 3];
    println!("Success!");
}

#[test]
fn test16() {
    let arr = ['a', 'b', 'c'];
    let ele = arr[0]; // Only modify this line to make the code work!
    assert_eq!(ele, 'a');
    println!("Success!");
}

#[test]
fn test17() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    // Get returns an Option<T>, it's safe to use
    let _name0 = names.get(0).unwrap();
    // But indexing is not safe
    let _name1 = &names[1];
    println!("Success!");
}

#[test]
// Fix the errors, DON'T add new lines!
fn test18() {
    let arr = [1, 2, 3];
    let _s1: &[i32] = &arr[0..2];
    let _s2: &str = &("hello, world" as &str);
    println!("Success!");
}

#[test]
fn test19() {
    let arr: [char; 3] = ['中', '国', '人'];
    let slice = &arr[..2];
    // Modify '8' to make it work
    // TIPS: slice( reference ) IS NOT an array, if it is an array, then assert! will be passed:
    // Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
    assert_eq!(size_of_val(&slice), 16);
    println!("Success!");
}

#[test]
fn test20() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Fill the blanks to make the code work
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);
    println!("Success!");
}

#[test]
fn test21() {
    let s = String::from("hello");
    let slice1 = &s[0..2];
    // Fill the blank to make the code work, DON'T USE 0..2 again
    let slice2 = &s[..2];
    assert_eq!(slice1, slice2);
    println!("Success!");
}

#[test]
fn test22() {
    let s = "你好，世界";
    // Modify this line to make the code work
    let slice = &s[0..3];
    assert_eq!(slice, "你");
    println!("Success!");
}

#[test]
// Fix errors
fn test23() {
    let mut s = String::from("hello world");
    // Here, &s is &String type, but first_letter needs a &str type.
    // It works because &String can be implicitly converted to `&str. If you want to know more,
    // this is called `Deref coercion`.
    let letter = first_letter(&s);
    println!("the first letter is: {}", letter);
    s.clear(); // error!
}
fn first_letter(s: &str) -> &str {
    &s[..1]
}

#[test]
fn test24() {
    let _t0: (u8,i16) = (0, -1);
    // Tuples can be tuple's members
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    // Fill the blanks to make the code work
    let _t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
    println!("Success!");
}

#[test]
fn test25() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");
    println!("Success!");
}

#[test]
fn test26() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12); //13);
    println!("too long tuple: {:?}", too_long_tuple); //or println!("too long tuple: {:?}", 13);
}

#[test]
fn test27() {
    let tup = (1, 6.4, "hello");
    // Fill the blank to make the code work
    let (x, z, y) = tup;
    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);
    println!("Success!");
}

#[test]
fn test28() {
    //Fill the blank
    let (a, b, c) = (1, 2, 3);
    let (x, y, z) = (c, a, b);
    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);
    println!("Success!");
}

#[test]
fn test29() {
    // Fill the blank, need a few computations here.
    let (x, y) = sum_multiply((3, 2));
    assert_eq!(x, 5);
    assert_eq!(y, 6);
    println!("Success!");
}
fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}

#[test]
// Fix the error
fn test30() {
    let age = 30;
    let _p = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("reading"),
    };
    println!("Success!");
    struct Person {
        name: String,
        age: u8,
        hobby: String
    }
}

#[test]
fn test31() {
    struct Unit;
    trait SomeTrait {
        // ...Some behaviors defined here.
    }
    // We don't care about what fields  are  in the Unit, but we care about its behaviors.
    // So we use a struct with no fields and implement some behaviors for it
    impl SomeTrait for Unit {  }
    let u = Unit;
    do_something_with_unit(u);
    println!("Success!"); //на раст бай практіс не виводить
    fn do_something_with_unit(_u: Unit) {   }
}

#[test]
fn test32() {
    // Fix the error and fill the blanks
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let v = Point(0, 127, 255);
    check_color(v);
    println!("Success!");
    fn check_color(p: Point) {
        let Point(x, _, y) = p;
        assert_eq!(x, 0);
        assert_eq!(p.1, 127);
        assert_eq!(y, 255);
    }
}

#[test]
fn test33() {
    // Fill the blank and fix the error without adding/removing new line
    struct Person {
        name: String,
        age: u8,
    }
    let age = 18;
    let mut p = Person {
        name: String::from("sunface"),
        age,
    };
    // How can you believe sunface is only 18?
    p.age = 30;
    // Fill the blank
    p.name = String::from("sunfei");
    println!("Success!");
}

#[test]
fn test34() {
    // Fill the blank
    struct Person {
        name: String,
        age: u8,
    }
    println!("Success!");
    fn build_person(name: String, age: u8) -> Person {
        Person {
            age,
            name,
        }
    }
}

#[test]
fn test35() {
    // Fill the blank to make the code work
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };
    let _u2 = set_email(u1);
    println!("Success!");
    fn set_email(u: User) -> User {
        User {
            email: String::from("contact@im.dev"),
            ..u
        }
    }
}

#[test]
fn test36() {
    // Fill the blanks to make the code work
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // Print debug info to stderr and assign the value of  30 * scale to width
        height: 50,
    };
    dbg!(&rect1); // Print debug info to stderr
    println!("{:?}", rect1); // Print debug info to stdout
}

#[test]
fn test37() {
    // Fix errors to make it work
    #[derive(Debug)]
    struct File {
        name: String,
        data: String,
    }
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };
    let _name = f.name;
    // ONLY modify this line
    println!("{}, {}", _name, f.data);
}

#[test]
fn test38() {
    // Fix the errors
    enum Number {
        Zero,
        One,
        Two,
    }
    enum Number1 {
        Zero = 0,
        One,
        Two,
    }
    // C-like enum
    enum Number2 {
        Zero = 0,
        One = 1,
        Two = 2,
    }
    // An enum variant can be converted to a integer by as
    assert_eq!(Number::One as u8, Number1::One as u8);
    assert_eq!(Number1::One as u8, Number2::One as u8);
    println!("Success!");
}

#[test]
fn test39() {
    // Fill in the blank
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let _msg1 = Message::Move{x: 1, y: 2}; // Instantiating with x = 1, y = 2
    let _msg2 = Message::Write(String::from("hello, world!")); // Instantiating with "hello, world!"
    println!("Success!");
}

#[test]
fn test40() {
    // Fill in the blank and fix the error
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg = Message::Move{x: 2, y: 2};
    if let Message::Move{x: a, y: b} = msg {
        assert_eq!(a, b);
    } else {
        panic!("NEVER LET THIS RUN！");
    }
    println!("Success!");
}

#[test]
fn test41() {
    // Fill in the blank and fix the errors
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];
    for msg in msgs {
        show_message(msg)
    }
    fn show_message(msg: Message) {
        println!("{:?}", msg);
    }
}

#[test]
fn test42() {
    // Fill in the blank to make the println work.
    // Also add some code to prevent the panic from running.
    let five = Some(5);
    let six = plus_one(five);
    let _none = plus_one(None);
    if let Some(n) = six {
        println!("{}", n);

        println!("Success!");
    } else {
        panic!("NEVER LET THIS RUN！");
    }
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
}