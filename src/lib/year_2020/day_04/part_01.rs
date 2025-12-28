//byr (Birth Year)
//iyr (Issue Year)
//eyr (Expiration Year)
//hgt (Height)
//hcl (Hair Color)
//ecl (Eye Color)
//pid (Passport ID)
//cid (Country ID)

fn is_valid(s: &str) -> bool {
    s.contains("byr:")
        && s.contains("iyr:")
        && s.contains("eyr:")
        && s.contains("hgt:")
        && s.contains("hcl:")
        && s.contains("ecl:")
        && s.contains("pid:")
    //&& s.contains("cid:")
}

pub fn part_01(input: &[&'static str]) -> usize {
    input.iter().filter(|x| is_valid(x)).count()
}
