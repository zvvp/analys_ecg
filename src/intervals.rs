
pub struct IntervalsR {
    pub ind_r: Vec<usize>,
    pub intervals_r: Vec<usize>,
    pub div_intervals: Vec<f32>,
}

impl IntervalsR {
    pub fn new(sum_leads: &Vec<f32>) -> IntervalsR {
        let mut max_val: f32 = 0.0;
        let mut ind_max = 0;
        let mut intervals = IntervalsR {
            ind_r: vec![],
            intervals_r: vec![],
            div_intervals: vec![],
        };
        for (ind, val) in sum_leads.iter().enumerate() {
            if *val > max_val {
                max_val = *val;
                ind_max = ind;
            }
            if *val <= 0.0 && max_val > 0.0 {
                max_val = 0.0;
                if intervals.intervals_r.is_empty() {
                    intervals.intervals_r.push(ind_max);
                } else {
                    let interval = ind_max - intervals.ind_r[intervals.ind_r.len() - 1];
                    intervals.intervals_r.push(interval);
                }
                intervals.ind_r.push(ind_max);
            }
        }
        while intervals.ind_r[0] < 55 {
            intervals.ind_r.remove(0);
            intervals.intervals_r.remove(0);

        }
        while (sum_leads.len() - intervals.ind_r[intervals.ind_r.len() - 1]) < 55 {
            intervals.ind_r.remove(intervals.ind_r.len() - 1);
            intervals.intervals_r.remove(intervals.ind_r.len() - 1);
        }
        if (sum_leads.len() - intervals.ind_r[intervals.ind_r.len() - 1]) < 40 {
            intervals.ind_r.remove(intervals.ind_r.len() - 1);
            intervals.intervals_r.remove(intervals.ind_r.len() - 1);
        }
        if !intervals.intervals_r.is_empty() {
            let mut temp = intervals.intervals_r.to_owned();
            temp.push(temp[temp.len() - 1]);
            temp.remove(0);
            for i in 0..temp.len() {
                let t: f32 = temp[i] as f32;
                let intr: f32 = intervals.intervals_r[i] as f32;
                if t > 0.0 {
                    intervals.div_intervals.push(intr / t);
                }
            }
        }
        intervals
    }
}