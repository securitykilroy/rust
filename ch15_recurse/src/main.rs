fn fac(n: u128) -> u128 {

    if n > 1 {
        n * fac(n-1)
    }
    else {
        n
    }
}

fn fib(n: u128) -> u128 {
    if n <= 1 {
        n
    }
    else {
        fib(n-1) + fib(n-2)
    }
}

fn towersolve(n: u16, from: char, to: char, other: char) {
    if n == 1 {
        println!("Moving disk 1 from rod {} to rod {}", from, to);
        return;
    }
    towersolve(n-1, from, other, to);
    println!("Moving disk {} from rod {} to rod {}", n, from, to);
    towersolve(n-1, other, to, from);
}

fn main() {
    println!("The result is {}", fac(5));
    println!("The 15th Fibonacci number is {}", fib(5));

    towersolve(4, 'A', 'B', 'C');

}
