extern crate chrono;
use chrono::*;

pub fn after(date_time: DateTime<UTC>) -> DateTime<UTC> {

let ten :i64 = 10;
date_time + Duration::seconds(ten.pow(9))

}
