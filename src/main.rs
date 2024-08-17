fn main(){
    let mut x : u8 = 32;
    println!("The value of x is {}", x);

    x = 69;
    println!("The value of x is now {}", x);

    let arr : [u8; 4] = [12, 3, 42, 11];
    let mut arr_size : usize = arr.len();

    while arr_size != 0 {
        println!("Array element {} value is {}", arr_size - 1, arr[arr_size-1]);
        arr_size -= 1;
    }  
    some_function();

    let key : bool = true;

    if key {
        println!("Yes");
    }

}

fn some_function(){
    println!("Some Function is called");
}