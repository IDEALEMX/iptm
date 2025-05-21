use crate::calendar::Calendar;

use rand::Rng;

#[derive(PartialEq, Eq)]
pub struct Id(u32);

impl Id {
    /*
     * Will generate a new and unique Id for a given task based on the ones already present at the
     * current calendar
     */
    pub fn new(calendar: Calendar) -> Id{
        let mut id: u32;
        loop {
            id = rand::rng().random();
            if let Some(_) = calendar.get_task_by_id(Id(id)) {}
            else {break};
        }
        Id(id)
    }
}
