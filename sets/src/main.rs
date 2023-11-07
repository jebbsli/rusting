fn main() {
    let v1:Vec<i32> = Vec::new();

    let v2 = vec![1,2,3];

    println!("v1:{:?}, v2:{:?}", v1, v2);

    // 更新
    let mut v3 = Vec::new();
    v3.push(6);
    v3.push(7);
    v3.push(8);

    println!("v3:{:?}", v3);

    // 通过索引读取
    let third: &i32 = &v3[2];
    println!("The third element is {}", third);

    // get方法
    let third:Option<&i32> = v3.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // 遍历
    for i in &v3 {
        println!("{}", i);
    }

    // 字符串
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    let hello = String::from("你好");
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");

}
