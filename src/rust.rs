use std::io; 
fn main(){ 
    let mut lines = String::new(); 
    io::stdin().read_line(&mut lines).ok(); 
    println!("{}", lines); 
}

// 출처 : https://github.com/Shota-Kudo/rust_learning_by_paiza