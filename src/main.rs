mod hw;
mod communication;

fn main(){
    hw::hello_world();
    let result = communication::my_udp();
    println!("udp com {:?}",result);
}
