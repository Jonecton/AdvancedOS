use rand::{random, thread_rng, Rng};

fn is_even(num: i32) -> bool {
    num % 2 == 0
}

fn fizz_buzz(num: i32) {
    if num % 3 == 0  && num % 5 == 0 {
        println!("FizzBuzz");
    }
    else if num % 5 == 0 {
        println!("Buzz");
    }
    else if num % 3 == 0 {
        println!("Fizz");
    }

    else {  }
}


fn main() {
    //Initialize array
    let mut nums_arr: [i8; 10] = [0; 10];
    for i in 1..10{
        let mut x = rand ::thread_rng();
        nums_arr[i] = x.gen_range(0..99);
    }
    println!("nums_arr = {:?}", nums_arr);

    //Sum values in array
    let mut sum:i32 = 0;
    let mut i = 0;
    while i < nums_arr.len(){
        sum = sum + nums_arr[i] as i32;
        i += 1;
    }
    println!("sum of nums_arr = {:?}", sum);

    //Find the largest value in the array
    let mut largest = 0;
    for i in nums_arr{
        if i > largest {
            largest = i;
        }
    }
    println!("largest num = {:?}", largest);
}
