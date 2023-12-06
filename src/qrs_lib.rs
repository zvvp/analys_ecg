use ndarray::Array1;

fn norm_qrs(qrs: &Vec<f32>) -> Vec<f32> {
    let mut min: f32 = 0.0;
    let mut max: f32 = 0.0;
    let mut out = qrs.to_owned();
    for i in 0..out.len() {
        if out[i] < min {
            min = out[i];
        }
    }
    for i in 0..out.len() {
        out[i] -= min;
    }
    for i in 0..out.len() {
        if out[i] > max {
            max = out[i];
        }
    }
    if max != 0.0 {
        for i in 0..out.len() {
            out[i] /= max;
        }
    }
    out
}

pub fn get_coef_cor(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    let norm_x = norm_qrs(&x);
    let norm_y = norm_qrs(&y);
    let arr_x = Array1::from_vec(norm_x);
    let arr_y = Array1::from_vec(norm_y);
    let mean_x = &arr_x.mean().unwrap();
    let mean_y = &arr_y.mean().unwrap();
    let arr_xy = &arr_x * &arr_y;
    let mean_xy = Array1::from(arr_xy).mean().unwrap();
    let std_x = &arr_x.std(0.0);
    let std_y = &arr_y.std(0.0);
    let std_xy = std_x * std_y;
    // let mut out: f32 = 0.0;
    if std_xy != 0.0 {
        (mean_xy - mean_x * mean_y) / std_xy
    } else { 0.0 }
    // out
}

pub fn max_cor(cor1: f32, cor2: f32, cor3: f32) -> f32 {
    let mut vec_cor = vec![cor1, cor2, cor3];
    vec_cor.sort_by(|a, b| a.partial_cmp(b).unwrap());
    vec_cor[2]
}

pub fn max_vec(vec: &Vec<f32>) -> f32 {
    let mut max_v = 0.0;
    for v in vec.iter() {
        if *v > max_v { max_v = *v };
    }
    max_v
}