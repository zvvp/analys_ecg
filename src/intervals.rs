
pub struct IntervalsR {
    pub ind_r: Vec<usize>,
    pub intervals_r: Vec<usize>,
    pub div_intervals: Vec<Option<f32>>,
}

impl IntervalsR {
    pub fn get_ind_r(&mut self, sum_leads: &Vec<f32>) {
        let mut max_val: f32 = 0.0;
        let mut ind_max = 0;
        // let mut interval = 0;
        for (ind, val) in sum_leads.iter().enumerate() {
            if *val > max_val {
                max_val = *val;
                ind_max = ind;
            }
            if *val <= 0.0 && max_val > 0.0 {
                max_val = 0.0;
                // self.ind_r.push(ind_max);
                if self.intervals_r.len() == 0 {
                    self.intervals_r.push(ind_max);
                } else {
                    let interval = ind_max - self.ind_r[self.ind_r.len() - 1];
                    self.intervals_r.push(interval);
                }
                self.ind_r.push(ind_max);
            }
        }
        while self.ind_r[0] < 55 {
            self.ind_r.remove(0);
            self.intervals_r.remove(0);

        }
        while (sum_leads.len() - self.ind_r[self.ind_r.len() - 1]) < 55 {
            self.ind_r.remove(self.ind_r.len() - 1);
            self.intervals_r.remove(self.ind_r.len() - 1);
        }
        if (sum_leads.len() - self.ind_r[self.ind_r.len() - 1]) < 40 {
            self.ind_r.remove(self.ind_r.len() - 1);
            self.intervals_r.remove(self.ind_r.len() - 1);
        }
    }

    pub fn get_div_intervals(&mut self) {
        if self.intervals_r.len() > 0 {
            let mut intervals = self.intervals_r.to_owned();
            intervals.push(intervals[intervals.len() - 1]);
            intervals.remove(0);
            self.div_intervals = self.intervals_r
                .iter()
                .zip(intervals.iter())
                .map(|(&x, &y)| {
                    if y > 0 {
                        Some(x as f32 / y as f32)
                    } else {
                        None
                    }
                })
                .collect();
        }
    }
}