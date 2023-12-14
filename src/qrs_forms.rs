use crate::file_ecg::Ecg;
use crate::intervals::IntervalsR;
use crate::qrs_lib::{get_coef_cor, max_vec};
use crate::ref_qrs::RefQrs;

pub struct QrsForm {
    pub form_indexes: Vec<usize>,
    pub mean_div_intervals: f32,
}

impl QrsForm {
    pub fn new() -> QrsForm {
        QrsForm {
            form_indexes: vec![],
            mean_div_intervals: 1.0,
        }
    }
    pub fn get_form_indexes(&mut self, leads: &Ecg, refs: &RefQrs, rem_indexes: &Vec<usize>, ind_r: &Vec<usize>) -> Vec<usize> {
        let mut rem_out = vec![];
        for i in 0..rem_indexes.len() {
            let mut coef_cor1 = vec![0.0; 41];
            let mut coef_cor2 = vec![0.0; 41];
            let mut coef_cor3 = vec![0.0; 41];

            let mut ind_qrs = ind_r[rem_indexes[i]];
            for j in 0..41 {
                let qrs1 = &leads.lead1[ind_qrs - 25 + j - 20..ind_qrs + 46 + j - 20].to_vec();
                coef_cor1[j] = get_coef_cor(&qrs1, &refs.ref_qrs1);
                let qrs2 = &leads.lead2[ind_qrs - 25 + j - 20..ind_qrs + 46 + j - 20].to_vec();
                coef_cor2[j] = get_coef_cor(&qrs2, &refs.ref_qrs2);
                let qrs3 = &leads.lead3[ind_qrs - 25 + j - 20..ind_qrs + 46 + j - 20].to_vec();
                coef_cor3[j] = get_coef_cor(&qrs3, &refs.ref_qrs3);
            }
            let max_cor1 = max_vec(&coef_cor1);
            let max_cor2 = max_vec(&coef_cor2);
            let max_cor3 = max_vec(&coef_cor3);
            if max_cor1 > 0.955 || max_cor2 > 0.955 || max_cor3 > 0.955 {
                &self.form_indexes.push(rem_indexes[i]);
            } else if max_cor1 > 0.84 && max_cor2 > 0.84 && max_cor3 > 0.84 {
                &self.form_indexes.push(rem_indexes[i]);
            } else {
                rem_out.push(rem_indexes[i]);
            }
        }
        rem_out
    }

    pub fn get_mean_div_intervals(&mut self, div_intervals: &Vec<f32>) {
        if !self.form_indexes.is_empty() {
            let mut sum_div: f32 = 0.0;
            for item in &self.form_indexes {
                sum_div += &div_intervals[*item];
            }
            self.mean_div_intervals = sum_div / self.form_indexes.len() as f32;
        }
    }
}

pub struct Forms {
    pub Form1: QrsForm,
    pub Form2: QrsForm,
    pub Form3: QrsForm,
    pub Form4: QrsForm,
    pub Form5: QrsForm,
    pub Form6: QrsForm,
    pub Form7: QrsForm,
    pub Form8: QrsForm,
    pub Form9: QrsForm,
    pub Form10: QrsForm,
}
