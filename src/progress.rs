pub struct Progress {
    total: usize,
    current: usize,
}

impl Progress {
    pub fn new(total: usize) -> Progress {
        return Progress { total, current: 0 };
    }

    fn increment(&mut self) {
        self.current += 1;
    }

    pub fn print(&self) {
        let percentage = (self.current as f64 / self.total as f64) * 100.0;

        if self.current == self.total {
            println!("\r{} / {} = {:.3} %", self.current, self.total, percentage);
            return;
        }

        print!("\r{} / {} = {:.3} %", self.current, self.total, percentage);
    }

    pub fn print_and_increment(&mut self) {
        self.print();
        self.increment();

        if self.current == self.total {
            self.print();
        }
    }
}
