use std::fs::File;
use crate::file_ecg::Ecg;
use crate::intervals::IntervalsR;
use crate::my_lib::pre_proc_r;
use crate::qrs_lib::{get_coef_cor, max_vec};
use crate::ref_qrs::RefQrs;
use std::io::prelude::*;
use std::io::BufWriter;
use encoding_rs::WINDOWS_1251;

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
        if rem_indexes.len() > 100 {
            for i in 0..rem_indexes.len() {
                let mut coef_cor1 = vec![0.0; 41];
                let mut coef_cor2 = vec![0.0; 41];
                let mut coef_cor3 = vec![0.0; 41];

                let ind_qrs = ind_r[rem_indexes[i]];
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
                    let _ = &self.form_indexes.push(rem_indexes[i]);
                } else if max_cor1 > 0.84 && max_cor2 > 0.84 && max_cor3 > 0.84 {
                    let _ = &self.form_indexes.push(rem_indexes[i]);
                } else {
                    rem_out.push(rem_indexes[i]);
                }
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
    pub form1: QrsForm,
    pub form2: QrsForm,
    pub form3: QrsForm,
    pub form4: QrsForm,
    pub form5: QrsForm,
    pub form6: QrsForm,
    pub form7: QrsForm,
    pub form8: QrsForm,
    pub form9: QrsForm,
}

impl Forms {
    pub fn new() -> Forms {
        Forms {
            form1: QrsForm::new(),
            form2: QrsForm::new(),
            form3: QrsForm::new(),
            form4: QrsForm::new(),
            form5: QrsForm::new(),
            form6: QrsForm::new(),
            form7: QrsForm::new(),
            form8: QrsForm::new(),
            form9: QrsForm::new(),
        }
    }

    pub fn get_types_qrs(&mut self) -> Vec<i32> {
        let mut leads = Ecg::new();
        let sum_leads = pre_proc_r(&mut leads);
        let intervals = IntervalsR::new(&sum_leads);

        let mut refqrs = RefQrs {
            ref_qrs1: vec![],
            ref_qrs2: vec![],
            ref_qrs3: vec![],
        };

        println!("Всего R: {}", intervals.ind_r.len());

        let mut rem: Vec<usize> = (0..intervals.ind_r.len()).collect();
        let mut ind_num_types = vec![0; intervals.ind_r.len()];

        refqrs.get_ref_forms(&leads, &rem, &intervals.ind_r);
        rem = self.form1.get_form_indexes(&leads, &refqrs, &rem, &intervals.ind_r);
        self.form1.get_mean_div_intervals(&intervals.div_intervals);
        for i in 0..self.form1.form_indexes.len() {
            ind_num_types[self.form1.form_indexes[i]] = 1;
        }

        if !rem.is_empty() {
            refqrs.get_ref_forms(&leads, &rem, &intervals.ind_r);
            rem = self.form2.get_form_indexes(&leads, &refqrs, &rem, &intervals.ind_r);
            self.form2.get_mean_div_intervals(&intervals.div_intervals);
            for i in 0..self.form2.form_indexes.len() {
                ind_num_types[self.form2.form_indexes[i]] = 2;
            }
        }
        if !rem.is_empty() {
            refqrs.get_ref_forms(&leads, &rem, &intervals.ind_r);
            rem = self.form3.get_form_indexes(&leads, &refqrs, &rem, &intervals.ind_r);
            self.form3.get_mean_div_intervals(&intervals.div_intervals);
            for i in 0..self.form3.form_indexes.len() {
                ind_num_types[self.form3.form_indexes[i]] = 3;
            }
        }
        if !rem.is_empty() {
            refqrs.get_ref_forms(&leads, &rem, &intervals.ind_r);
            rem = self.form4.get_form_indexes(&leads, &refqrs, &rem, &intervals.ind_r);
            self.form4.get_mean_div_intervals(&intervals.div_intervals);
            for i in 0..self.form4.form_indexes.len() {
                ind_num_types[self.form4.form_indexes[i]] = 4;
            }
        }
        if !rem.is_empty() {
            refqrs.get_ref_forms(&leads, &rem, &intervals.ind_r);
            rem = self.form5.get_form_indexes(&leads, &refqrs, &rem, &intervals.ind_r);
            self.form5.get_mean_div_intervals(&intervals.div_intervals);
            for i in 0..self.form5.form_indexes.len() {
                ind_num_types[self.form5.form_indexes[i]] = 5;
            }
        }
        if !rem.is_empty() {
            refqrs.get_ref_forms(&leads, &rem, &intervals.ind_r);
            rem = self.form6.get_form_indexes(&leads, &refqrs, &rem, &intervals.ind_r);
            self.form6.get_mean_div_intervals(&intervals.div_intervals);
            for i in 0..self.form6.form_indexes.len() {
                ind_num_types[self.form6.form_indexes[i]] = 6;
            }
        }
        if !rem.is_empty() {
            refqrs.get_ref_forms(&leads, &rem, &intervals.ind_r);
            rem = self.form7.get_form_indexes(&leads, &refqrs, &rem, &intervals.ind_r);
            self.form7.get_mean_div_intervals(&intervals.div_intervals);
            for i in 0..self.form7.form_indexes.len() {
                ind_num_types[self.form7.form_indexes[i]] = 7;
            }
        }
        if !rem.is_empty() {
            refqrs.get_ref_forms(&leads, &rem, &intervals.ind_r);
            rem = self.form8.get_form_indexes(&leads, &refqrs, &rem, &intervals.ind_r);
            self.form8.get_mean_div_intervals(&intervals.div_intervals);
            for i in 0..self.form8.form_indexes.len() {
                ind_num_types[self.form8.form_indexes[i]] = 8;
            }
        }
        if !rem.is_empty() {
            refqrs.get_ref_forms(&leads, &rem, &intervals.ind_r);
            let _rem = self.form9.get_form_indexes(&leads, &refqrs, &rem, &intervals.ind_r);
            self.form9.get_mean_div_intervals(&intervals.div_intervals);
            for i in 0..self.form9.form_indexes.len() {
                ind_num_types[self.form9.form_indexes[i]] = 9;
            }
        }

        let file = File::create("B.txt").expect("Не удалось создать файл");
        let mut writer = BufWriter::new(file);

        let text = "\nДата:\nПациент:\nВозраст:\nПол:\n№ палаты:\n№ истории болезни:\n\n\n\n\n\n";
        let (encoded_text, _encoding_used, _errors) = WINDOWS_1251.encode(&text);
        writer
            .write(&encoded_text)
            .expect("Не удалось записать в файл");

        for i in 0..intervals.ind_r.len() {
            let arg3 = match &ind_num_types[i] {
                1 => "N",
                2 => "V",
                _ => "A",
            };
            let text1 = format!("{};{};{}\n", &intervals.ind_r[i], &intervals.intervals_r[i], arg3);
            let (encoded_text, _encoding_used, _errors) = WINDOWS_1251.encode(&text1);
            writer
                .write(&encoded_text)
                .expect("Не удалось записать в файл");
        }
        ind_num_types
    }
}
