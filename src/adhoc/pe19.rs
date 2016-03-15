
// Between year1 and year2 (inclusive), given the first day of year1 (sunday = 0), count the number
// of times the first day of a month happens to be a sunday.
pub fn allfirstsundays(year1: usize, year2: usize, day: usize) -> usize{
    let mut firstday = day;
    let mut firstsundays = 0;
    for y in year1..year2 + 1 {
        let monthvec = monthvec(y);
        for m in &monthvec{
            firstday = (*m + firstday) % 7;
            firstday %= 7;
            if firstday == 0 {
                firstsundays += 1;
            }
        }
    }
    if firstday == 0 {
        firstsundays -= 1;
    }
    firstsundays
}
            
// Returns a Vec<usize> of length 12 representing the number of days in the months in a year.
fn monthvec(year: usize) -> Vec<usize> {
    let feb;
    if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
        feb = 29;
    } else {
        feb = 28;
    }
    vec![31, feb, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
}
