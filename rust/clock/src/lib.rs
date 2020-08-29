use std::fmt::{self, Display};

const H : i32 = 24;
const M : i32 = 60;
#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut temp_hours: i32  = 0;
        let mut temp_minutes = 0;

        if 0 <= hours && 0 <= minutes { // hour >= 0 && minutes > 0
            if minutes % M == 0 {
                temp_minutes = 0;
            } else {
                temp_minutes = minutes % M;
            }
            
            let tmp_hour = minutes / M;
            
            let hours = hours + tmp_hour;
            if hours % H == 0 {
                temp_hours = 0;
            } else {
                temp_hours = hours % H;
            }
        } else if 0 <= hours && minutes < 0 { // hours >= 0 && mintues < 0
            let mut minutes = minutes;
            let mut tmp_hour = 0;
            loop {
                if 0 <= minutes {
                    break;
                }
                minutes += M;
                tmp_hour += 1;

            }
            temp_minutes = minutes;
            let mut hours = hours - tmp_hour;
            if 0 <= hours {
                if hours % H == 0 {
                    temp_hours = 0;
                }else {
                    temp_hours = hours % H;
                }
            }else {
                loop {
                    if 0 < hours {
                        break;
                    }
                    hours += H;
                }
                
                if hours % H == 0 {
                    temp_hours = 0;
                }else {
                    temp_hours = hours % H;
                }
            }
        } else if hours < 0 && 0 <= minutes { // hours < 0 && mintues >= 0
            if minutes % M == 0 {
                temp_minutes = 0;
            }else {
                temp_minutes = minutes % M;
            }
            let temp_hour = minutes / M;
            let mut hours = hours + temp_hour;
            if 0 < hours {
                if hours % H == 0 {
                    temp_hours = 0;
                }else {
                    temp_hours = hours % H;
                }
            }else {
                loop {
                    if 0 < hours {
                        break;
                    }
                    hours += H;
                }
                if hours % H == 0 {
                    temp_hours = 0;
                }else {
                    temp_hours = hours % H;
                }
            }
        } else if hours < 0 && minutes < 0 { // hours < 0 && mintues < 0
            let mut hours = hours;
            let mut minutes = minutes;

            let mut temp_hour = 0;
            loop {
                if 0 <= minutes {
                    break;
                }
                minutes += M;
                temp_hour += 1;
            }
            temp_minutes = minutes; // M
            hours = hours - temp_hour;
            loop {
                if 0 < hours {
                    break;
                }
                hours += H;
            }
            if hours % H == 0 {
                temp_hours = 0;
            }else {
                temp_hours = hours % H;
            }
        }

        Self { 
            hours: temp_hours,
            minutes: temp_minutes,
         }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
       Clock::new(self.hours,self.minutes+minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}


impl Eq for Clock{}
