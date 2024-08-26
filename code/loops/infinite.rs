fn main() {
    let mut n = 0;

    //create an infinite loop
    loop {
        n = n + 1; //increase n by 1

        //continue (skips this iteration)
        if n == 13 {
            continue;
        }

        //break the loop
        if n > 15 {
            break;
        }
        println!("The value of n is  {}", n);
    }
}
