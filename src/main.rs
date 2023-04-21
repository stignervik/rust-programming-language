const NUMBER_OF_UNITS: usize = 10;
const GROUP_START_ADDRESS: usize = 10;
// const NUMBER_OF_GROUPS: usize = 5;
fn main() {
    let group_id = GROUP_START_ADDRESS;
    for id in 0..NUMBER_OF_UNITS {
        println!("Unit with id: {}", id);
    }
}
