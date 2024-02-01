mod file_ecg;
mod intervals;
mod my_lib;
mod ref_qrs;
mod qrs_lib;
mod qrs_forms;

use crate::qrs_forms::Forms;
// use std::time::Instant;


fn main() {
    // let start = Instant::now();
    let mut forms = Forms::new();
    let _ind_num_qrs = forms.get_types_qrs();
    // let duration = start.elapsed().as_millis();
    // println!("Время выполнения: {} ms", duration);

}
