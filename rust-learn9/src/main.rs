fn main() {
    let s1 = String::new();
    println!("s1:{},s1-len:{}",s1,s1.len());
    let s2 = String::from("面向加薪学习");
    println!("s2:{},s2-len:{}",s2,s2.len());

    let mut s3 = String::new();
    s3.push_str("Go语言极简一本通");
    println!("{}",s3);

    s3.push('O');
    s3.push('K');

    println!("{}",s3);

    let s4 = String::from("面向加薪学习");
    let result = s4.replace("面向加薪学习","www.go-edu.cn");
    println!("{}",result);

    let s5 = String::from("面向加薪学习 www.go-edu.cn\n");
    println!("length is {}",s5.len());

    let s6 = "从0到Go语言微服务架构师".to_string();
    println!("{}",s6);

    let s7 = String::from("Go语言微服务架构核心22讲"); 
    show_name(s7.as_str());

}

fn show_name(name:&str) {
    println!("充电科目：{}",name);
}