fn print_type_of<T>(_: &T){
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    println!("Hello, world!\n");

    // fizz buzz
    for i in 1..101{
        if i % 15 == 0 {
            print!("fizz,buzz,");
        }
        else if i % 3 == 0 {
            print!("fizz,");
        } else if i % 5 == 0 {
            print!("buzz,");
        } 
        else{
            print!("{},",i);
        }
    }

    println!("");

    // ceaser cipher
    let message = String::from("Hello world!!??\n");
    println!("Original: {}",message);

    let message_byte_vec = message.as_bytes();
    println!("UTF8 bytes: {:?}",message_byte_vec);
    print_type_of(&message_byte_vec);

    let shift = 60;
    let mut new_message_vec = Vec::new();

    for x in message_byte_vec.iter() {
        let y = x + shift;
        println!("{}", y);
        new_message_vec.push(y);
    }

    println!("{:?}", new_message_vec);

    
    for y in new_message_vec.iter() {
        let y_vec = vec![*y % 128];
        print!("{}", String::from_utf8(y_vec).unwrap());
    }

}
