fn main() {
    let r = [(3, "Fizz"), (5, "Buzz"), (7, "Bizz")];
    for i in 1..=100 {
        let mut pr = false;
        for j in 0..r.len() {
            if i % r[j].0 == 0 {
                print!("{}", r[j].1);
                pr = true;
            }
        }
        match pr {
            true => println!(),
            _ => println!("{}", i),
        };
    }
}




    
