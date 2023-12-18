mod file_ecg;
mod intervals;
mod my_lib;
mod ref_qrs;
mod qrs_lib;
mod qrs_forms;

use crate::qrs_forms::Forms;


fn main() {
    let mut forms = Forms::new();
    forms.get_types_qrs();
    println!("{:?}", &forms.form0.form_indexes[0..10]);
    println!("{}", &forms.form0.mean_div_intervals);
    println!("{:?}", &forms.form1.form_indexes[0..10]);
    println!("{}", &forms.form1.mean_div_intervals);

}
