fn main() {
    println!("Hello, world!");
}

struct Triangle{
    side: u32,
    altitude: u32,
} 

impl Triangle {
    fn area(&self) -> u32{
        1/2 * self.side * self.altitude
    }
}

//Compute area
fn compute_area<T>(len:<T>)->T{

}