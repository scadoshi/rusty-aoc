#[allow(dead_code)]
pub fn part_02(input: &[i32]) -> i32 {
    // println!("{:?}", input);
    let mut p = 50;
    let mut total = 0;
    for num in input {
        let range = if *num > 0 {
            0_isize..*num as isize
        } else {
            *num as isize..0_isize
        };
        for _ in range {
            // println!("num = {}", num);
            // std::thread::sleep(std::time::Duration::from_millis(50));
            // println!("{}", p);
            p = (p + num.signum()) % 100;
            if p < 0 {
                p += 100;
            }
            if p == 0 {
                // println!("hit zero!");
                total += 1;
            }
        }
    }
    total
}
