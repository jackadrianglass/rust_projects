use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() != 2 {
        return;
    }

    let num : i32 = args[1].trim().parse()
        .expect("Invalid number arg");

    let result = cal_fib( num );
    println!("The {}th number in fibonacci is {}", num, result );
}

fn cal_fib( num : i32 ) -> i32 {
    if num > 0 {
        return num + cal_fib( num - 1 );
    }
    else {
        return 0;
    }
}