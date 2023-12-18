use crate::file_ecg::Ecg;
use crate::qrs_lib::{get_coef_cor, max_vec};

pub struct RefQrs {
    pub ref_qrs1: Vec<f32>,
    pub ref_qrs2: Vec<f32>,
    pub ref_qrs3: Vec<f32>,
}

impl RefQrs {
    pub fn get_ref_forms(&mut self, leads: &Ecg, rem_indexes: &Vec<usize>, ind_r: &Vec<usize>) {
        if rem_indexes.len() > 1 {
            for i in 0..rem_indexes.len() - 1 {
                let ind_qrs = ind_r[rem_indexes[i]];
                let ind_qrs1 = ind_r[rem_indexes[i + 1]];

                if ind_qrs < 45 {
                    continue;
                }
                if ind_qrs > ind_r.len() - 47 {
                    continue;
                }

                let start_index = ind_qrs - 25;
                let end_index = ind_qrs + 46;
                let start_index1 = ind_qrs1 - 25;
                let end_index1 = ind_qrs1 + 46;

                let qrs1 = &leads.lead1[start_index..end_index].to_vec();
                let qrs2 = &leads.lead2[start_index..end_index].to_vec();
                let qrs3 = &leads.lead3[start_index..end_index].to_vec();

                let mut coef_cor1 = vec![0.0; 41];
                let mut coef_cor2 = vec![0.0; 41];
                let mut coef_cor3 = vec![0.0; 41];

                for j in 0..41 {
                    let qrs11 = &leads.lead1[start_index1 + j - 20..end_index1 + j - 20].to_vec();
                    coef_cor1[j] = get_coef_cor(&qrs1, &qrs11);
                    let qrs22 = &leads.lead2[start_index1 + j - 20..end_index1 + j - 20].to_vec();
                    coef_cor2[j] = get_coef_cor(&qrs2, &qrs22);
                    let qrs33 = &leads.lead3[start_index1 + j - 20..end_index1 + j - 20].to_vec();
                    coef_cor3[j] = get_coef_cor(&qrs3, &qrs33);
                }
                let max_cor1 = max_vec(&coef_cor1);
                let max_cor2 = max_vec(&coef_cor2);
                let max_cor3 = max_vec(&coef_cor3);

                if max_cor1 > 0.93 && max_cor2 > 0.93 && max_cor3 > 0.93 {
                    self.ref_qrs1 = qrs1.to_owned();
                    self.ref_qrs2 = qrs2.to_owned();
                    self.ref_qrs3 = qrs3.to_owned();
                    break;
                }
            }
        }
    }
}