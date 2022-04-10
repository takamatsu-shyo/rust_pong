mod communication;
mod hw;

fn main() {
    hw::hello_world();
    let result = communication::my_udp();
    println!("udp com {:?}", result);
}
