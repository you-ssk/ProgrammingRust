use std::borrow::BorrowMut;
use std::cell::Cell;
use std::cell::RefCell;
use std::fs::File;
use std::rc::Rc;

pub struct SpiderRobot {
    species: String,
    web_enabled: bool,
    // leg_devices: [fd::FileDesc; 8],
    hardware_eror_count: Cell<u32>,
    log_file: RefCell<File>,
}

pub struct SpiderSenses {
    robot: Rc<SpiderRobot>,
    //eyes: [Camera;32],
    // motion: Accelerometer,
}

impl SpiderRobot {
    pub fn add_hardware_error(&self) {
        let n = self.hardware_eror_count.get();
        self.hardware_eror_count.set(n + 1);
    }
    pub fn has_hardware_error(&self) -> bool {
        self.hardware_eror_count.get() > 0
    }
    pub fn log(&self, message: &str) {
        let mut file = self.log_file.borrow_mut();
        //writeln!(file, "{}", message).unwrap();
    }
}
