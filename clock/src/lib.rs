use std::fmt;

pub struct Clock{
    hours: i32,
    minutes:i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // unimplemented!(
        //     "Construct a new Clock from {} hours and {} minutes",
        //     hours,
        //     minutes
        // );
        let mut cal = Clock { hours: hours, minutes: minutes };
        Clock::cal_min(&mut cal);  
        Clock::cal_hour(&mut cal);  
        cal
    }

    fn cal_min(cal: &mut Clock) {
        let mut stop = false;
        while !stop {
            if cal.minutes > 59 {
                cal.minutes = cal.minutes - 60;
                cal.hours = cal.hours + 1;
            } else if cal.minutes < 0 {
                cal.minutes = cal.minutes + 60;
                cal.hours = cal.hours - 1;
            } else {
                stop = true;
            }
            
        }           
    }

    fn cal_hour(cal: &mut Clock) {
        let mut stop = false;
        while !stop {
            if cal.hours > 23 {
                cal.hours = cal.hours - 24;
            } else if cal.hours < 0 {
                cal.hours = cal.hours + 24;
            } else {
                stop = true;
            }
            
        }
    }

    pub fn add_minutes(&self, aminutes: i32) -> Self {
        // unimplemented!("Add {} minutes to existing Clock time", minutes);
        let min = self.minutes + aminutes;
        let hr = self.hours;
        Clock::new(hr, min)
    }
}



impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool{ 
        self.hours == other.hours && self.minutes == other.minutes
        // true
    }
    // fn ne(&self, _other: &Clock) -> bool { false }
}