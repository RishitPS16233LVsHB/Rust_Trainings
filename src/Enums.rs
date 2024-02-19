
#[derive(Debug)]
enum WeekDays{
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday
}


fn main() {
    let day = WeekDays::Friday;
    println!("day is {:?} and it is the {}th day of the week",day,get_weekday_number(&day));
}


fn get_weekday_number(weekday:&WeekDays) -> i32{
    match weekday {
        WeekDays::Sunday => 1,
        WeekDays::Monday => 2,
        WeekDays::Tuesday => 3,
        WeekDays::Wednesday => 4,
        WeekDays::Thursday => 5,
        WeekDays::Friday => 6,
        WeekDays::Saturday => 7
    }
}