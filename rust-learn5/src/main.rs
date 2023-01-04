fn main()
{
    let tup:(i32,f64,u8) = (500,6.4,1);

    let(x,y,z) = tup;

    println!("{x},{y},{z}");

    exmain();

    extwomain();

    exthreemain();
}

fn exmain()
{
    let tup:(i32,f64,u8) = (500,6.4,1);
    
    println!("{},{},{}",tup.0,tup.1,tup.2);
}

fn extwomain()
{
    let a = [1,2,3,4,5];
    println!("{:?}", a);
}

fn exthreemain()
{
    let month = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("{:?}",month);

    let first = month[0];
    
    println!("{first}");
}