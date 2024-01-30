use std::fs::File;
use crate::file_ecg::Ecg;
use crate::intervals::IntervalsR;
use crate::my_lib::{clean_ch, del_isoline, multi_del_isoline, pre_proc_r};
use crate::qrs_lib::{get_coef_cor, max_vec};
use crate::ref_qrs::RefQrs;
use std::io::prelude::*;
use std::io::BufWriter;
use encoding_rs::WINDOWS_1251;
use std::time::Instant;


pub struct QrsForm<'a> {
    pub form_indexes: Vec<usize>,
    pub mean_div_intervals: f32,
    pub mean_double_intervals: f32,
    pub form_char: &'a str,
}

impl QrsForm<'_> {
    pub fn new() -> QrsForm<'static> {
        QrsForm {
            form_indexes: vec![],
            mean_div_intervals: 0.0,
            mean_double_intervals: 0.0,
            form_char: "",
        }
    }

    pub  fn get_form_indexes(&mut self, leads: &Ecg, refs: &RefQrs, rem_indexes: &Vec<usize>, ind_r: &Vec<usize>) -> Vec<usize> {
        let start = Instant::now();
        let mut rem_out = vec![];
        let mut count1 = 0;
        let mut count2 = 0;
        let mut count3 = 0;
        if rem_indexes.len() > 1 {
            for i in 0..rem_indexes.len() {
                let ind_qrs = ind_r[rem_indexes[i]];
                let ind_start = ind_qrs - 45;
                let ind_stop = ind_qrs + 67;
                let qrs_area1 = leads.lead1[ind_start..ind_stop].to_vec();
                let qrs_area2 = leads.lead2[ind_start..ind_stop].to_vec();
                let qrs_area3 = leads.lead3[ind_start..ind_stop].to_vec();
                let mut coef_cor1 = vec![0.0; 41];
                let mut coef_cor2 = vec![0.0; 41];
                let mut coef_cor3 = vec![0.0; 41];
                for i in 0..40 {
                    let qrs1 = qrs_area1[i..i+71].to_vec();
                    coef_cor1[i] = get_coef_cor(&qrs1, &refs.ref_qrs1);
                    let qrs2 = qrs_area2[i..i+71].to_vec();
                    coef_cor2[i] = get_coef_cor(&qrs2, &refs.ref_qrs2);
                    let qrs3 = qrs_area3[i..i+71].to_vec();
                    coef_cor3[i] = get_coef_cor(&qrs3, &refs.ref_qrs3);
                }
                let max_cor1 = max_vec(&coef_cor1);
                let max_cor2 = max_vec(&coef_cor2);
                let max_cor3 = max_vec(&coef_cor3);

                if max_cor1 > 0.95 || max_cor2 > 0.95 || max_cor3 > 0.95 {   // 0.945
                    let _ = &self.form_indexes.push(rem_indexes[i]);
                    count1 += 1;
                } else if max_cor1 > 0.88 && max_cor2 > 0.88 && max_cor3 > 0.88 { // 0.83
                    let _ = &self.form_indexes.push(rem_indexes[i]);
                    count2 += 1;
                } else if (max_cor1 > 0.91 && max_cor2 > 0.91 && max_cor3 > 0.2) // 0.86 0.7
                    || (max_cor1 > 0.91 && max_cor2 > 0.2 && max_cor3 > 0.91)
                    || (max_cor1 > 0.2 && max_cor2 > 0.91 && max_cor3 > 0.91) {
                    let _ = &self.form_indexes.push(rem_indexes[i]);
                    count3 += 1;
                } else {
                    rem_out.push(rem_indexes[i]);
                    // if (ind_r[rem_indexes[i]] > 22800) && (ind_r[rem_indexes[i]] < 23200) {
                    //     println!("{} = {}, {}, {}", ind_r[rem_indexes[i]], max_cor1, max_cor2, max_cor3)
                    // }
                }

            }
        }
        println!("{}, {}, {}", count1, count2, count3);
        let duration = start.elapsed().as_millis();
        println!("Время выполнения get_form_indexes: {} ms", duration);
        rem_out
    }

    pub fn get_mean_double_intervals(&mut self, intervals: &Vec<usize>) {
        if !self.form_indexes.is_empty() {
            let mut count: usize = 0;
            let mut sum: usize = 0;
            let mut mean_intervals: f32 = 0.0;
            for item in &self.form_indexes {
                sum += &intervals[*item];
                count += 1;
            }
            mean_intervals = sum as f32 / count as f32;
            self.mean_double_intervals = mean_intervals;
            println!("{}", self.mean_double_intervals);
        }
    }

    pub fn get_mean_div_intervals(&mut self, div_intervals: &Vec<f32>) {
        if !self.form_indexes.is_empty() {
            let mut count_div: i32 = 0;
            let mut sum_div: f32 = 0.0;
            for item in &self.form_indexes {
                if (div_intervals[*item] < 3.0) && (div_intervals[*item] > 0.4) {
                    sum_div += &div_intervals[*item];
                    count_div += 1;
                }
            }
            self.mean_div_intervals = sum_div / count_div as f32;
            // println!("{}", count_div);
        }
    }
}

pub struct Forms<'a> {
    pub form1: QrsForm<'a>,
    pub form2: QrsForm<'a>,
    pub form3: QrsForm<'a>,
    pub form4: QrsForm<'a>,
    pub form5: QrsForm<'a>,
    pub form6: QrsForm<'a>,
    pub form7: QrsForm<'a>,
    pub form8: QrsForm<'a>,
    pub form9: QrsForm<'a>,
}

impl Forms<'_> {
    pub fn new() -> Forms<'static> {
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
        let start = Instant::now();
        let sum_leads = pre_proc_r(&mut leads);
        let duration = start.elapsed().as_millis();
        println!("Время выполнения pre_proc_r: {} ms", duration);
        // let start = Instant::now();
        let intervals = IntervalsR::new(&sum_leads);
        // let duration = start.elapsed().as_millis();
        // println!("Время выполнения IntervalsR: {} ms", duration);

        let mut refqrs = RefQrs {
            ref_qrs1: vec![],
            ref_qrs2: vec![],
            ref_qrs3: vec![],
        };

        leads.lead1 = multi_del_isoline(&leads.lead1);
        leads.lead2 = multi_del_isoline(&leads.lead2);
        leads.lead3 = multi_del_isoline(&leads.lead3);

        println!("Всего R: {}", intervals.ind_r.len());

        let mut rem: Vec<usize> = (0..intervals.ind_r.len()).collect();
        let mut ind_num_types = vec![0; intervals.ind_r.len()];

        let start = Instant::now();
        refqrs.get_ref_forms(&leads, &rem, &intervals.ind_r, 0.9);
        let duration = start.elapsed().as_millis();
        println!("Время выполнения get_ref_forms: {} ms", duration);
        // let start = Instant::now();
        rem = self.form1.get_form_indexes(&leads, &refqrs, &rem, &intervals.ind_r);
        // let duration = start.elapsed().as_millis();
        // println!("Время выполнения get_form_indexes: {} ms", duration);
        self.form1.get_mean_div_intervals(&intervals.div_intervals);
        self.form1.get_mean_double_intervals(&intervals.intervals_r);
        let deviation1 = self.form1.mean_double_intervals / intervals.mean_intervals_r;
        if self.form1.mean_div_intervals >= 0.94 {
            self.form1.form_char = "N";
        }
        if (self.form1.mean_div_intervals < 0.94) && (self.form1.mean_div_intervals > 0.1) {
            self.form1.form_char = "V";
        }
        if deviation1 < 0.65 {
            self.form1.form_char = "V";
        }
        for i in 0..self.form1.form_indexes.len() {
            ind_num_types[self.form1.form_indexes[i]] = 1;
        }

        if !rem.is_empty() {
            refqrs.get_ref_forms(&leads, &rem, &intervals.ind_r, 0.9);
            rem = self.form2.get_form_indexes(&leads, &refqrs, &rem, &intervals.ind_r);
            self.form2.get_mean_div_intervals(&intervals.div_intervals);
            self.form2.get_mean_double_intervals(&intervals.intervals_r);
            let deviation2 = self.form2.mean_double_intervals / intervals.mean_intervals_r;
            if self.form2.mean_div_intervals >= 0.93 {
                self.form2.form_char = "N";
            }
            if (self.form2.mean_div_intervals < 0.93) && (self.form2.mean_div_intervals > 0.1) {
                self.form2.form_char = "V";
            }
            if deviation2 < 0.65 {
                self.form2.form_char = "V";
            }
            for i in 0..self.form2.form_indexes.len() {
                ind_num_types[self.form2.form_indexes[i]] = 2;
            }
        }
        if !rem.is_empty() {
            refqrs.get_ref_forms(&leads, &rem, &intervals.ind_r, 0.9);
            rem = self.form3.get_form_indexes(&leads, &refqrs, &rem, &intervals.ind_r);
            self.form3.get_mean_div_intervals(&intervals.div_intervals);
            self.form3.get_mean_double_intervals(&intervals.intervals_r);
            let deviation3 = self.form3.mean_double_intervals / intervals.mean_intervals_r;
            if self.form3.mean_div_intervals >= 0.92 {
                self.form3.form_char = "N";
            }
            if (self.form3.mean_div_intervals < 0.92) && (self.form3.mean_div_intervals > 0.1) {
                self.form3.form_char = "V";
            }
            if deviation3 < 0.65 {
                self.form3.form_char = "V";
            }
            for i in 0..self.form3.form_indexes.len() {
                ind_num_types[self.form3.form_indexes[i]] = 3;
            }
        }
        if !rem.is_empty() {
            refqrs.get_ref_forms(&leads, &rem, &intervals.ind_r, 0.9);
            rem = self.form4.get_form_indexes(&leads, &refqrs, &rem, &intervals.ind_r);
            self.form4.get_mean_div_intervals(&intervals.div_intervals);
            self.form4.get_mean_double_intervals(&intervals.intervals_r);
            let deviation4 = self.form4.mean_double_intervals / intervals.mean_intervals_r;
            if self.form4.mean_div_intervals >= 0.92 {
                self.form4.form_char = "N";
            }
            if (self.form4.mean_div_intervals < 0.92) && (self.form4.mean_div_intervals > 0.1) {
                self.form4.form_char = "V";
            }
            if deviation4 < 0.65 {
                self.form4.form_char = "V";
            }
            for i in 0..self.form4.form_indexes.len() {
                ind_num_types[self.form4.form_indexes[i]] = 4;
            }
        }
        if !rem.is_empty() {
            refqrs.get_ref_forms(&leads, &rem, &intervals.ind_r, 0.9);
            rem = self.form5.get_form_indexes(&leads, &refqrs, &rem, &intervals.ind_r);
            self.form5.get_mean_div_intervals(&intervals.div_intervals);
            self.form5.get_mean_double_intervals(&intervals.intervals_r);
            let deviation5 = self.form5.mean_double_intervals / intervals.mean_intervals_r;
            if self.form5.mean_div_intervals >= 0.92 {
                self.form5.form_char = "N";
            }
            if (self.form5.mean_div_intervals < 0.92) && (self.form5.mean_div_intervals > 0.1) {
                self.form5.form_char = "V";
            }
            if deviation5 < 0.65 {
                self.form5.form_char = "V";
            }
            for i in 0..self.form5.form_indexes.len() {
                ind_num_types[self.form5.form_indexes[i]] = 5;
            }
        }
        if !rem.is_empty() {
            refqrs.get_ref_forms(&leads, &rem, &intervals.ind_r, 0.9);
            rem = self.form6.get_form_indexes(&leads, &refqrs, &rem, &intervals.ind_r);

            self.form6.get_mean_div_intervals(&intervals.div_intervals);
            self.form6.get_mean_double_intervals(&intervals.intervals_r);
            let deviation6 = self.form6.mean_double_intervals / intervals.mean_intervals_r;
            if self.form6.mean_div_intervals >= 0.92 {
                self.form6.form_char = "N";
            }
            if (self.form6.mean_div_intervals < 0.92) && (self.form6.mean_div_intervals > 0.1) {
                self.form6.form_char = "V";
            }
            if deviation6 < 0.65 {
                self.form6.form_char = "V";
            }
            for i in 0..self.form6.form_indexes.len() {
                ind_num_types[self.form6.form_indexes[i]] = 6;
            }
        }
        if !rem.is_empty() {
            refqrs.get_ref_forms(&leads, &rem, &intervals.ind_r, 0.9);
            rem = self.form7.get_form_indexes(&leads, &refqrs, &rem, &intervals.ind_r);

            self.form7.get_mean_div_intervals(&intervals.div_intervals);
            self.form7.get_mean_double_intervals(&intervals.intervals_r);
            let deviation7 = self.form7.mean_double_intervals / intervals.mean_intervals_r;
            if self.form7.mean_div_intervals >= 0.92 {
                self.form7.form_char = "N";
            }
            if (self.form7.mean_div_intervals < 0.92) && (self.form7.mean_div_intervals > 0.1) {
                self.form7.form_char = "V";
            }
            if deviation7 < 0.65 {
                self.form7.form_char = "V";
            }
            for i in 0..self.form7.form_indexes.len() {
                ind_num_types[self.form7.form_indexes[i]] = 7;
            }
        }
        if !rem.is_empty() {
            refqrs.get_ref_forms(&leads, &rem, &intervals.ind_r, 0.9);
            rem = self.form8.get_form_indexes(&leads, &refqrs, &rem, &intervals.ind_r);

            self.form8.get_mean_div_intervals(&intervals.div_intervals);
            self.form8.get_mean_double_intervals(&intervals.intervals_r);
            let deviation8 = self.form8.mean_double_intervals / intervals.mean_intervals_r;
            if self.form8.mean_div_intervals >= 0.92 {
                self.form8.form_char = "N";
            }
            if (self.form8.mean_div_intervals < 0.92) && (self.form8.mean_div_intervals > 0.1) {
                self.form8.form_char = "V";
            }
            if deviation8 < 0.65 {
                self.form8.form_char = "V";
            }
            for i in 0..self.form8.form_indexes.len() {
                ind_num_types[self.form8.form_indexes[i]] = 8;
            }
        }
        if !rem.is_empty() {
            refqrs.get_ref_forms(&leads, &rem, &intervals.ind_r, 0.9);
            let _rem = self.form9.get_form_indexes(&leads, &refqrs, &rem, &intervals.ind_r);

            self.form9.get_mean_div_intervals(&intervals.div_intervals);
            self.form9.get_mean_double_intervals(&intervals.intervals_r);
            let deviation9 = self.form9.mean_double_intervals / intervals.mean_intervals_r;
            if self.form9.mean_div_intervals >= 0.92 {
                self.form9.form_char = "N";
            }
            if (self.form9.mean_div_intervals < 0.92) && (self.form9.mean_div_intervals > 0.1) {
                self.form9.form_char = "V";
            }
            if deviation9 < 0.65 {
                self.form9.form_char = "V";
            }
            for i in 0..self.form9.form_indexes.len() {
                ind_num_types[self.form9.form_indexes[i]] = 9;
            }
        }

        let file = File::create("C:\\EcgVar\\B.txt").expect("Не удалось создать файл");
        let mut writer = BufWriter::new(file);

        let text = "\nДата:\nПациент:\nВозраст:\nПол:\n№ палаты:\n№ истории болезни:\n\n\n\n\n\n";
        let (encoded_text, _encoding_used, _errors) = WINDOWS_1251.encode(&text);
        writer
            .write(&encoded_text)
            .expect("Не удалось записать в файл");

        for i in 0..intervals.ind_r.len() {
            let arg3 = match &ind_num_types[i] {
                1 => self.form1.form_char,
                2 => self.form2.form_char,
                3 => self.form3.form_char,
                4 => self.form4.form_char,
                5 => self.form5.form_char,
                6 => self.form6.form_char,
                7 => self.form7.form_char,
                8 => self.form8.form_char,
                9 => self.form9.form_char,
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
