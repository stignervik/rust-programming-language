use crate::model::unit::Unit;

const NUMBER_OF_UNITS: usize = 10;
const GROUP_START_ADDRESS: usize = 10;
// const NUMBER_OF_GROUPS: usize = 5;

mod model;

fn main() {
    let mut group_id = GROUP_START_ADDRESS;
    let mut number_of_units_in_group = 0;
    for id in 0..NUMBER_OF_UNITS {
        println!("Unit with id: {}", id);
        if number_of_units_in_group == 2 {
            number_of_units_in_group = 0;
            group_id += 1;
        }
        number_of_units_in_group += 1;

        println!("group id: {}", group_id);
    }

    let unit = Unit::new(1, String::from("Unit 1"), 0, 0);
    println!("Unit: {:?}", unit);
}
