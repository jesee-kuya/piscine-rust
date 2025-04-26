use std::ops::Add;

pub struct StepIterator<T> {
    current: T,
    end: T,
    step: T,
    is_positive_step: bool,
}

impl<T> StepIterator<T>
where
    T: Add<Output = T> + PartialOrd + Copy,
{
    pub fn new(beg: T, end: T, step: T) -> Self {
        let is_positive_step = (beg + step) > beg;
        StepIterator {
            current: beg,
            end,
            step,
            is_positive_step,
        }
    }
}

impl<T> Iterator for StepIterator<T>
where
    T: Add<Output = T> + PartialOrd + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_positive_step {
            if self.current > self.end {
                return None;
            }
        } else {
            if self.current < self.end {
                return None;
            }
        }

        let current = self.current;
        self.current = current + self.step;
        Some(current)
    }
}

