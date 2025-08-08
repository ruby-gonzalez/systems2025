

fn is_even(n: i32) -> bool {
    n % == 0
}

fn main(){
    let nums = [17, 42, 8, 33, 56, 91, 24, 5, 67, 13];

    // for loop even/odd & FizzBuzz 
    for n in nums {
        if n % 3 == 0 && n % 5 == 0 {
            println!("{} -> FizzBuzz", n);
        } else if n % 3 == 0 {
            println!("{} -> Fizz", n);
        } else if n % 5 == 0 {
            println!("{} -> Buzz", n);
        } else {
            let even_or_odd = if is_even(n) { "Even" } else { "Odd" };
            println!("{} -> {}", n, even_or_odd);
        }
    }

    // while loop sum array
    let mut i = 0;
    let mut sum = 0;
    while i < nums.len() {
        sum += nums[i];
        i += 1;
    }
    println!("Sum of numbers: {}", sum);

    // loop largest number
    let mut max = nums[0];
    for n in nums {
        if n > max {
            max = n;
        }
    }
    println!("Largest number: {}", max);

}