const _STARTING_MISSILES: i32 = 8;
const _READY_AMOUNT: i32 = 5;
fn main() {
    let mut missiles = _STARTING_MISSILES;
    let ready = _READY_AMOUNT;

    missiles = missiles - ready;

    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left", missiles);
}

// parte 2
