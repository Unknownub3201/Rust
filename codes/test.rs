fn main(){
    let x:&str = "Test";
    cube();
    println!("{x}");
}

fn cube(){
    let x:i32 = 5;
    let y:f64 = 2.5;
    println!("Cube of Y: {}",y*y*y);
    println!("Cube of X: {}",x*x*x);
}
