fn main() {
    ex_one_function();
    ex_two_function();
    ex_three_function();
}

fn ex_one_function() {
    for n in 1..4 {
        println!("Hello,World! {}",n);
    }
    println!("---------------------------");

}

fn ex_two_function() {
    for _n in (1..4).rev() {
        println!("This box_size is {}",_n);
    }
    println!("---------------------------");

}

fn ex_three_function() {
    let number = [1,2,3,4,5];
    for element in number.iter() {
        println!("This box_ex_size is {}",element);
    }
}