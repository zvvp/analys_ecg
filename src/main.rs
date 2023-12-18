mod file_ecg;
mod intervals;
mod my_lib;
mod ref_qrs;
mod qrs_lib;
mod qrs_forms;

use file_ecg::Ecg;
use crate::intervals::IntervalsR;
use crate::my_lib::{clean_ch, del_isoline, pre_proc_r};
use crate::qrs_forms::{QrsForm, Forms};
use crate::ref_qrs::RefQrs;

fn main() {
    let mut leads = Ecg::new();
    let sum_leads = pre_proc_r(&leads);
    let mut intervals = IntervalsR::new(&sum_leads);

    leads.lead1 = clean_ch(&leads.lead1);
    leads.lead2 = clean_ch(&leads.lead2);
    leads.lead3 = clean_ch(&leads.lead3);
    leads.lead1 = del_isoline(&leads.lead1);
    leads.lead2 = del_isoline(&leads.lead2);
    leads.lead3 = del_isoline(&leads.lead3);

    let mut refqrs = RefQrs {
        ref_qrs1: vec![],
        ref_qrs2: vec![],
        ref_qrs3: vec![],
    };

    let mut forms = Forms::new();

    let rem: Vec<usize> = (0..intervals.ind_r.len()).collect();
    refqrs.get_ref_forms(&leads, &rem, &intervals.ind_r);
    let rem = &forms.Form0.get_form_indexes(&leads, &refqrs, &rem, &intervals.ind_r);
    println!("{:?}", &forms.Form0.form_indexes[0..10]);
    println!("{:?}", &rem[0..10]);

    refqrs.get_ref_forms(&leads, &rem, &intervals.ind_r);
    let rem = &forms.Form1.get_form_indexes(&leads, &refqrs, &rem, &intervals.ind_r);
    println!("{:?}", &forms.Form1.form_indexes);
    println!("{:?}", &rem);

    // let mut qrs1 = QrsForm::new();
    //
    // let rem: Vec<usize> = (0..intervals.ind_r.len()).collect();
    //
    // refqrs.get_ref_forms(&leads, &rem, &intervals.ind_r);
    //
    // let rem = qrs1.get_form_indexes(&leads, &refqrs, &rem, &intervals.ind_r);
    //
    // qrs1.get_mean_div_intervals(&intervals.div_intervals);
    //
    // println!("{:?}", &qrs1.form_indexes[0..10]);
    // println!("{}", qrs1.mean_div_intervals);
    // println!("{:?}", &rem[0..10]);
    //
    // let mut qrs2 = QrsForm::new();
    //
    // refqrs.get_ref_forms(&leads, &rem, &intervals.ind_r);
    //
    // let rem = qrs2.get_form_indexes(&leads, &refqrs, &rem, &intervals.ind_r);
    //
    // qrs2.get_mean_div_intervals(&intervals.div_intervals);
    //
    // println!("{:?}", &qrs2.form_indexes[0..10]);
    // println!("{}", qrs2.mean_div_intervals);
    // println!("{:?}", &rem[0..rem.len()]);
    //
    // let mut qrs3 = QrsForm::new();
    //
    // refqrs.get_ref_forms(&leads, &rem, &intervals.ind_r);
    //
    // let rem = qrs3.get_form_indexes(&leads, &refqrs, &rem, &intervals.ind_r);
    //
    // qrs3.get_mean_div_intervals(&intervals.div_intervals);
    //
    // println!("{:?}", &qrs3.form_indexes[0..qrs3.form_indexes.len()]);
    // println!("{}", qrs3.mean_div_intervals);
    // println!("{:?}", &rem[0..rem.len()]);

    println!("{:?}", &leads.file_name);
}
