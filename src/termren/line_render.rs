
pub struct LineRenderer {
    start: (i32, i32),
    end: (i32, i32),
    m: f64,
    b: f64
}

impl LineRenderer {
    pub fn new(start: (i32, i32), end: (i32, i32)) -> LineRenderer {
        let m;
        let b;
        if start.0 - end.0 == 0 {
            m = f64::INFINITY;
            b = 0.;
        } else {
            m = (start.1 - end.1) as f64 / (start.0 - end.0) as f64;
            b = start.1 as f64 - m*start.0 as f64;
        }

        LineRenderer {
            start, end, m, b
        }
    }

    pub fn start(&self,height: i32) -> i32 {
        if self.m==0.0 || self.m==f64::INFINITY{
            return self.start.0;
        }
        ((height as f64-self.b)/self.m) as i32
    }
    pub fn end(&self,height: i32) -> i32 {
        if self.m==0.0 || self.m==f64::INFINITY{
            return self.end.0;
        }
        ((height as f64-self.b)/self.m) as i32
    }
}