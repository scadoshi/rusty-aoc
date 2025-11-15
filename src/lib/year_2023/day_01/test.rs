use crate::year_2023::day_01::find_number::FindNumber;

#[allow(dead_code)]
#[ignore]
fn first_number_test() {
    assert_eq!("1twothree".first_number(), Some(1));
    assert_eq!("12three".first_number(), Some(1));
    assert_eq!("123".first_number(), Some(1));
    assert_eq!("one23".first_number(), Some(1));
    assert_eq!("onetwo3".first_number(), Some(1));
    assert_eq!("onetwothree".first_number(), Some(1));
    assert_eq!("".first_number(), None);
    assert_eq!("nospellhere".first_number(), None);
    assert_eq!("xyz7uvw".first_number(), Some(7));
    assert_eq!("sevenxyz".first_number(), Some(7));
    assert_eq!("sixseven8eight".first_number(), Some(6));
    assert_eq!("seven777".first_number(), Some(7));
    assert_eq!("987six".first_number(), Some(9));
    assert_eq!("sixsix".first_number(), Some(6));
    assert_eq!("two123".first_number(), Some(2));
    assert_eq!("threeight".first_number(), Some(3));
    assert_eq!("zerozero".first_number(), Some(0));
    assert_eq!("2zero".first_number(), Some(2));
}

#[allow(dead_code)]
#[ignore]
fn last_number_test() {
    assert_eq!("1twothree".last_number(), Some(3));
    assert_eq!("12three".last_number(), Some(3));
    assert_eq!("123".last_number(), Some(3));
    assert_eq!("one23".last_number(), Some(3));
    assert_eq!("onetwo3".last_number(), Some(3));
    assert_eq!("onetwothree".last_number(), Some(3));
    assert_eq!("".last_number(), None);
    assert_eq!("nospellhere".last_number(), None);
    assert_eq!("xyz7uvw".last_number(), Some(7));
    assert_eq!("sevenxyz".last_number(), Some(7));
    assert_eq!("6seven".last_number(), Some(7));
    assert_eq!("seven777".last_number(), Some(7));
    assert_eq!("987six".last_number(), Some(6));
    assert_eq!("sixsix".last_number(), Some(6));
    assert_eq!("two123".last_number(), Some(3));
    assert_eq!("threeight".last_number(), Some(8));
    assert_eq!("zerozero".last_number(), Some(0));
    assert_eq!("2zero".last_number(), Some(0));
}

#[allow(dead_code)]
#[ignore]
fn both_numbers_test() {
    assert_eq!("onetwo".both_numbers(), Some(12));
    assert_eq!("sevenfour".both_numbers(), Some(74));
    assert_eq!("1two".both_numbers(), Some(12));
    assert_eq!("one2".both_numbers(), Some(12));
    assert_eq!("threeight".both_numbers(), Some(38));
    assert_eq!("2zero".both_numbers(), Some(20));
    assert_eq!("12three".both_numbers(), Some(13));
    assert_eq!("123".both_numbers(), Some(13));
    assert_eq!("12345".both_numbers(), Some(15));
    assert_eq!("1thingand9".both_numbers(), Some(19));
    assert_eq!("only5here".both_numbers(), Some(55));
    assert_eq!("hereonlyzero".both_numbers(), Some(00));
    assert_eq!("nope".both_numbers(), None);
    assert_eq!("eightwo".both_numbers(), Some(82));
    assert_eq!("sevenine".both_numbers(), Some(79));
    assert_eq!("8seven9".both_numbers(), Some(89));
    assert_eq!("nineseven8one".both_numbers(), Some(91));
    assert_eq!("twotwo3one".both_numbers(), Some(21));
}
