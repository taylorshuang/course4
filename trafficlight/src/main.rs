fn main() {
    let time = printTrafficTime(Red);
    println!("Now is red light, please wait {} seconds.", time);

    printTrafficTime(Yellow);
    println!("Now is yellow light, please wait {} seconds.", time);

    printTrafficTime(Green);
    println!("Now is green light, please wait {} seconds.", time);
}

trait  TrafficTime{
   fn time(&self) -> u8; 
}


struct Red;

impl TrafficTime for Red{
    fn time(&self)->u8{
        100
    }
}

struct Yellow;
impl TrafficTime for Yellow{
    fn time(&self)->u8{
        10
    }
}

struct Green;
impl TrafficTime for Green{
    fn time(&self)->u8{
        60
    }
}

fn printTrafficTime<TT: TrafficTime>(tt: TT )->u8 {
    tt.time()
}