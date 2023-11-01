
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("jebbs01"),
        email: String::from("jebbs01@qq.com"),
        sign_in_count: 1
    };
    println!("user1's active is: {}", user1.active);
    println!("user1's username is: {}", user1.username.to_string());
    println!("user1's email is: {}", user1.email.to_string());
    println!("user1's sign_in_count is: {}", user1.sign_in_count);

    let mut user2 = User {
        active: true,
        username: String::from("jebbs02"),
        email: String::from("jebbs02@qq.com"),
        sign_in_count: 10
    };
    user2.sign_in_count = 20;
    println!("user2's active is: {}", user2.active);
    println!("user2's username is: {}", user2.username.to_string());
    println!("user2's email is: {}", user2.email.to_string());
    println!("user2's sign_in_count is: {}", user2.sign_in_count);

    let user3 = build_user(String::from("jebbs03"), String::from("jebbs03@qq.com"));
    println!("user3's active is: {}", user3.active);
    println!("user3's username is: {}", user3.username.to_string());
    println!("user3's email is: {}", user3.email.to_string());
    println!("user3's sign_in_count is: {}", user3.sign_in_count);

    let user4 = build_user2(String::from("jebbs04"), String::from("jebbs04@qq.com"));
    println!("user4's active is: {}", user4.active);
    println!("user4's username is: {}", user4.username.to_string());
    println!("user4's email is: {}", user4.email.to_string());
    println!("user4's sign_in_count is: {}", user4.sign_in_count);

    // 使用结构体更新语法
    let mut user5 = User {
        username: String::from("jebbs05"),
        ..user4
    };
    user5.email = String::from("jebbs05@qq.com");
    println!("user5's active is: {}", user5.active);
    println!("user5's username is: {}", user5.username.to_string());
    println!("user5's email is: {}", user5.email.to_string());
    println!("user5's sign_in_count is: {}", user5.sign_in_count);
}

fn build_user(username: String, email: String) -> User {
    User { 
        active: true, 
        username: username, 
        email: email, 
        sign_in_count: 1 
    }
}

// 参数名与字段名都完全相同，我们可以使用字段初始化简写语法，如：username和email字段
fn build_user2(username: String, email: String) -> User {
    User { 
        active: true, 
        username, 
        email, 
        sign_in_count: 1 
    }
}
