fn main()
{
    let a : u32 = 32;
    let b = String::from("Hello!");
    testu32(a);
    println!("{}", a);             //OK
    testString(b);
    //println!("{}", b);           //Error
    let c = String::from("Hello!");
    println!("{}", c.clone());     //OK
}

fn testu32(a : u32)
{
    println!("{}", a);
}

fn testString(a : String)
{
    println!("{}", a);
}