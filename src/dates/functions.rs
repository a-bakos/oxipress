use crate::dates::date::GB_DateTime;

pub fn get_current_date() -> String {
    let date: GB_DateTime = GB_DateTime::new();
    date.formatted
}
