#[no_mangle]
pub extern "C" fn simple_hello(){
    println!("Hello from rust");
}

fn main(){
    simple_hello();
}
