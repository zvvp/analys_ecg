mod file_ecg;
mod intervals;
mod my_lib;
mod ref_qrs;
mod qrs_lib;
mod qrs_forms;

use file_ecg::Ecg;
use crate::intervals::IntervalsR;
use crate::my_lib::pre_proc_r;

fn main() {
    let leads = Ecg::new();
    let sum_leads = pre_proc_r(&leads);
    let mut intervals = IntervalsR::new(&sum_leads);

    println!("{:?}", &leads.file_name);
    println!("{:?}", &intervals.intervals_r[1]);
    dbg!(intervals.div_intervals[6]);
}
