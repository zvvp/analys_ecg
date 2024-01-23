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

    // println!("Type1: {:?}", &forms.form1.form_indexes.len());
    // println!("Type1: {:?}", &forms.form1.form_indexes[..20]);
    // println!("Type1: {:?}", &ind_num_qrs[..60]);
    // println!("  {}", &forms.form1.mean_div_intervals);
    //
    // println!("Type2: {:?}", &forms.form2.form_indexes.len());
    // println!("  {}", &forms.form2.mean_div_intervals);
    //
    // println!("Type3: {:?}", &forms.form3.form_indexes.len());
    // println!("  {}", &forms.form3.mean_div_intervals);
    //
    // println!("Type4: {:?}", &forms.form4.form_indexes.len());
    // println!("  {}", &forms.form4.mean_div_intervals);
    //
    // println!("Type5: {:?}", &forms.form5.form_indexes.len());
    // println!("  {}", &forms.form5.mean_div_intervals);
    //
    // println!("Type6: {:?}", &forms.form6.form_indexes.len());
    // println!("  {}", &forms.form6.mean_div_intervals);
    //
    // println!("Type7: {:?}", &forms.form7.form_indexes.len());
    // println!("  {}", &forms.form7.mean_div_intervals);
    //
    // println!("Type8: {:?}", &forms.form8.form_indexes.len());
    // println!("  {}", &forms.form8.mean_div_intervals);
    //
    // println!("Type9: {:?}", &forms.form9.form_indexes.len());
    // println!("  {}", &forms.form9.mean_div_intervals);
}
