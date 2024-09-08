use chrono::NaiveDate;

pub fn is_date_valid(date: &String) -> bool {
    match NaiveDate::parse_from_str(&date, "%d/%m/%Y %H:%M:%S") {
        Ok(_) => true,
        Err(_) => false,
    }
}
