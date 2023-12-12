mod file_ecg;
mod intervals;
mod my_lib;
mod ref_qrs;
mod qrs_lib;
mod qrs_forms;

use file_ecg::Ecg;
use crate::intervals::IntervalsR;
use crate::my_lib::{clean_ch, del_isoline, pre_proc_r};
use crate::qrs_forms::QrsForm;
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
    refqrs.get_ref_forms(&leads, &intervals.ind_r);
    let mut qrs1 = QrsForm::new();
    let rem = qrs1.get_form_indexes(&leads.lead1, &leads.lead2, &leads.lead3, &refqrs, &intervals.ind_r);
    println!("{:?}", &qrs1.form_indexes[0..5]);

    refqrs.get_ref_forms(&leads, &rem);
    let mut qrs2 = QrsForm::new();
    let rem = qrs2.get_form_indexes(&leads.lead1, &leads.lead2, &leads.lead3, &refqrs, &rem);
    println!("{:?}", &qrs2.form_indexes[0..5]);


    println!("{:?}", &leads.file_name);
    // println!("{:?}", &intervals.intervals_r[1]);
    // dbg!(intervals.div_intervals[6]);
}
