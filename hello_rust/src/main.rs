fn main() {
    let x = 3;
    println!("X의 값 : {x}"); // X의 값 : 3
    let x = x + 1;
    println!("X의 값 : {x}"); // X의 값 : 4
    {
        let x = x * 2;
        println!("X의 값 : {x}"); // X의 값 : 8
    }
    println!("X의 값 : {x}"); // X의 값 : 4
}
