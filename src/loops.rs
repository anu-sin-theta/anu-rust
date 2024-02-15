fn main(){
    for i in (1..=10).rev(){
        // 1..10 for exclusive range, 1..=10 for inclusive range
        println!("panther{}",i);

        // while loop
    while i > 5 {
        println!("{} is greater than 5", i);
        break;
    }

        // loop, simplest form of looping
    loop {
        if i == 5 {
            println!("{} is equal to 5", i);
            break;
        }
        println!("I'm a loop");
        break;
    }
    }
}