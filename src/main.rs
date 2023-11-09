fn main() {
    println!("Hello, world!");

    another_function();
    another_function2(7);
    print_labeled_measurement(5, 'h');
    //cant do bc statements dont return values
    //let x = (let y = 6);
    {
        let y = {
            let x = 3;
            x + 1
            //also works, last x is like a return 
            // let mut x = 3;
            // x = x + 1;
            // x
        };

    println!("The value of y is: {y}");
    }
    //Functions with Return Values
    {
        let x = five();

        println!("The value of x is: {x}");
    }

    {
        let x = plus_one(5);

        println!("The value of x is: {x}");
    }
}

fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}