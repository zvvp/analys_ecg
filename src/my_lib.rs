use crate::file_ecg::Ecg;
use std::sync::mpsc;
use std::thread;

fn my_diff(ch: &Vec<f32>) -> Vec<f32> {
    let ch_copy = ch.to_owned();
    let mut ch_copy1 = ch.to_owned();
    ch_copy1.remove(0);
    ch_copy1.push(0.0);
    let diff_ch: Vec<f32> = ch_copy.iter()
        .zip(ch_copy1.iter())
        .map(|(val1, val2)| val1 - val2).collect();
    diff_ch
}

fn get_mean_diff(ch: &Vec<f32>) -> f32 {
    let d_ch = my_diff(&ch);
    let abs_d_ch: Vec<f32> = d_ch.iter().map(|val| val.abs()).collect();
    let sum_d_ch: f32 = abs_d_ch.iter().sum();
    let mean_d_ch = sum_d_ch / d_ch.len() as f32;
    let (count, sum): (usize, f32) = abs_d_ch
        .iter()
        .filter(|&x| *x > mean_d_ch)
        .fold((0, 0.0), |(count, sum), &x| (count + 1, sum + x));
    let mean_d_ch1 = sum / count as f32;
    let (count, sum): (usize, f32) = abs_d_ch
        .iter()
        .filter(|&x| *x > mean_d_ch1)
        .fold((0, 0.0), |(count, sum), &x| (count + 1, sum + x));
    let mean_d_ch2 = sum / count as f32;
    let out = mean_d_ch + mean_d_ch1 + mean_d_ch2;
    out
}

fn cut_impuls1(ch: &Vec<f32>) -> Vec<f32> {
    let mut out = ch.clone();
    let mean_d = get_mean_diff(&ch);
    for i in 5..ch.len() - 6 {
        let d_out0 = (&out[i] - &out[i - 1]) / &mean_d;
        let d_out1 = (&out[i + 1] - &out[i]) / &mean_d;
        let d_out2 = (&out[i + 2] - &out[i + 1]) / &mean_d;
        let sign0 = d_out0.signum();
        let sign1 = d_out1.signum();
        let sign2 = d_out2.signum();
        let abs0 = d_out0.abs();
        let abs1 = d_out1.abs();
        let abs2 = d_out2.abs();
        if (sign0 != sign1) && (sign1 != sign2) && (abs1 > 1.4)
            && (((abs0 - abs1).abs() < abs1 * 0.88) || ((abs1 - abs2).abs() < abs1 * 0.88)) {
            let win: Vec<&f32> = ch.iter().skip(i - 5).take(11).collect();
            let mut sort_win = win.to_owned();
            sort_win.sort_by(|a, b| a.partial_cmp(b).unwrap());
            out[i - 2] = *sort_win[5];
            out[i - 1] = *sort_win[5];
            out[i] = *sort_win[5];
            out[i + 1] = *sort_win[5];
            out[i + 2] = *sort_win[5];
            out[i + 3] = *sort_win[5];
            if ((&out[i + 4] - &out[i + 3]) / &mean_d).abs() > 0.15 {
                out[i + 4] = *sort_win[5];
                out[i + 5] = *sort_win[5];
            }
        }
        if (sign0 != sign1) && (abs0 > 1.0) && (abs1 > 1.0) && ((abs0 - abs1).abs() < (abs0 + abs1) * 0.5) {
            let win: Vec<&f32> = ch.iter().skip(i - 5).take(11).collect();
            let mut sort_win = win.to_owned();
            sort_win.sort_by(|a, b| a.partial_cmp(b).unwrap());
            out[i - 2] = *sort_win[5];
            out[i - 1] = *sort_win[5];
            out[i] = *sort_win[5];
            out[i + 1] = *sort_win[5];
            out[i + 2] = *sort_win[5];
            if ((&out[i + 2] - &out[i + 1]) / &mean_d).abs() > 0.05 {
                out[i + 2] = *sort_win[5];
                out[i + 3] = *sort_win[5];
            }
        }
    }
    out
}

fn my_filtfilt(b: &Vec<f32>, a: &Vec<f32>, ch: &Vec<f32>) -> Vec<f32> {
    let mut temp = ch.to_owned();
    let mut out = ch.to_owned();
    let len_b = b.len();
    let len_a = a.len();
    let len_ch = ch.len();

    for i in (len_b - 1..len_ch).step_by(1) {
        temp[i] = b[0] * ch[i];
        for j in 1..len_b {
            temp[i] += b[j] * ch[i - j];
        }
        for j in 1..len_a {
            temp[i] -= a[j] * temp[i - j];
        }
    }

    for i in (1..=(len_ch - len_b)).rev() {
        out[i] = b[0] * temp[i];
        for j in 1..len_b {
            out[i] += b[j] * temp[i + j];
        }
        for j in 1..len_a {
            out[i] -= a[j] * out[i + j];
        }
    }
    out
}

fn get_spec24(ch: &Vec<f32>) -> Vec<f32> {
    let bp = vec![0.05695238, 0.0, -0.05695238];
    let ap = vec![1.0, -1.55326091, 0.88609524];

    let bl = vec![0.11216024, 0.11216024];
    let al = vec![1.0, -0.77567951];

    let bh = vec![0.97547839, -0.97547839];
    let ah = vec![1.0, -0.95095678];

    let spec24 = my_filtfilt(&bp, &ap, &ch);
    let spec24 = spec24.iter().map(|&x| (x * 4.0).abs()).collect::<Vec<f32>>();
    let spec24 = my_filtfilt(&bl, &al, &spec24);
    let spec24 = my_filtfilt(&bh, &ah, &spec24);

    spec24
}

fn get_spec50(ch: &Vec<f32>) -> Vec<f32> {
    let bp = vec![0.13672874, 0.0, -0.13672874];
    let ap = vec![1.0, -0.53353098, 0.72654253];

    let bl = vec![0.24523728, 0.24523728];
    let al = vec![1.0, -0.50952545];

    let spec50 = my_filtfilt(&bp, &ap, &ch);
    let spec50 = spec50.iter().map(|&x| x.abs()).collect::<Vec<f32>>();
    let spec50 = my_filtfilt(&bl, &al, &spec50);

    spec50
}

pub fn clean_ch(ch: &Vec<f32>) -> Vec<f32> {
    // b, a = butter(2, 20, 'lp', fs=250)
    let b = vec![0.0461318, 0.0922636, 0.0461318];
    let a = vec![1.0, -1.30728503, 0.49181224];

    let ch_del_ks = cut_impuls1(&ch);
    let ch_del_ks = cut_impuls1(&ch_del_ks);
    let ch_del_ks = cut_impuls1(&ch_del_ks);
    let mut fch = ch_del_ks.clone();

    let spec24 = get_spec24(&ch_del_ks);
    let spec50 = get_spec50(&ch_del_ks);
    let clean_ch = my_filtfilt(&b, &a, &ch_del_ks);

    for i in 0..fch.len() {
        if spec50[i] > spec24[i] {
            fch[i] = clean_ch[i];
        }
    }
    fch
}

fn get_trs(p2p: &Vec<f32>) -> f32 {
    let sum_p2p: f32 = p2p
        .iter()
        .sum();
    let mean_p2p: f32 = sum_p2p / p2p.len() as f32;
    let (count, sum): (usize, f32) = p2p
        .iter()
        .filter(|&x| *x > mean_p2p)
        .fold((0, 0.0), |(count, sum), &x| (count + 1, sum + x));
    let trs = sum / count as f32;
    trs
}

fn get_p2p(ch: &Vec<f32>, win: usize, sqr: bool) -> Vec<f32> {
    let half_win = win / 2;
    let len_ch = ch.len();
    let mut p2p = vec![0.0; len_ch];
    for i in (0..len_ch - &win).step_by(2) {
        let win_ch = &ch[i..i + &win];
        let win_max = win_ch.iter().fold(f32::NEG_INFINITY, |max, &x| x.max(max));
        let win_min = win_ch.iter().fold(f32::INFINITY, |min, &x| x.min(min));
        let mut p2pw = &win_max - &win_min;
        if sqr {
            p2pw = &p2pw * &p2pw;
        }
        p2p[i + half_win - 2] = p2pw;
        p2p[i + half_win - 1] = p2pw;
        p2p[i + half_win] = p2pw;
        p2p[i + half_win + 1] = p2pw;
        p2p[i + half_win + 2] = p2pw;
    }
    if sqr {
        let trs = get_trs(&p2p);
        if trs > 0.0 {
            for i in 0..p2p.len() {
                p2p[i] /= trs;
                if p2p[i] > 0.5 { p2p[i] = 0.5 + p2p[i] * 0.1; }
            }
        }
    }
    p2p
}

fn del_artifacts(ch: &Vec<f32>, p2p: &Vec<f32>) -> (Vec<f32>, Vec<f32>) {
    let len_ch = ch.len();
    let mut out = ch.to_owned();
    let mut mask = vec![1.0; len_ch];

    let signs = vec_sign(&ch);
    let diff_signs: Vec<f32> = my_diff(&signs);

    let mut prev_ind = 0;
    let mut flag: bool = false;
    let trs = get_trs(&p2p);

    for i in 0..diff_signs.len() - 1 {
        if (p2p[i] > trs * 6.0) || p2p[i] > 5.5 {                 // 3.8 2.6
            flag = true;
        }
        if diff_signs[i] != 0.0 {
            if flag == true {
                let zeros = vec![0.0; i - prev_ind + 1];
                let range = prev_ind..=i;
                out.splice(range.clone(), zeros.clone());
                mask.splice(range.clone(), zeros.clone());
            }
            prev_ind = i;
            flag = false;
        }
    }
    (out, mask)
}

fn sign(x: &f32) -> f32 {
    if *x > 0.0 {
        1.0
    } else if *x < 0.0 {
        -1.0
    } else {
        0.0
    }
}

fn vec_sign(ch: &Vec<f32>) -> Vec<f32> {
    let out = ch.iter().map(|&val| sign(&val)).collect();
    out
}

fn del_nouse(ch: &Vec<f32>, mask: &Vec<f32>) -> Vec<f32> {
    // bhn, ahn = butter(2, 6, 'hp', fs=250)
    let bhn = vec![0.89884553, -1.79769105, 0.89884553];
    let ahn = vec![1.0, -1.78743252, 0.80794959];
    // bln, aln = butter(6, 25, 'lp', fs=250)
    let bln = vec![0.00034054, 0.00204323, 0.00510806, 0.00681075, 0.00510806, 0.00204323, 0.00034054];
    let aln = vec![1.0, -3.5794348, 5.65866717, -4.96541523, 2.52949491, -0.70527411, 0.08375648];

    let fch = my_filtfilt(&bhn, &ahn, &ch);
    let fch = my_filtfilt(&bln, &aln, &fch);

    let result: Vec<f32> = fch
        .iter()
        .zip(mask.iter())
        .map(|(&x, &y)| x * y)
        .collect();
    result
}

fn filt_r(ch: &Vec<f32>) -> Vec<f32> {
    //blr, alr = butter(1, 0.15, 'lp', fs=250)
    let blr = vec![0.00188141, 0.00188141];
    let alr = vec![1.0, -0.99623718];
    //blr, alr = butter(1, 0.1, 'lp', fs=250)
    // let blr = vec![0.00125506, 0.00125506];
    // let alr = vec![1.0, -0.99748988];
    // bhr, ahr = butter(1, 6.1, 'hp', fs=250)
    let bhr = vec![0.92867294, -0.92867294];
    let ahr = vec![1.0, -0.85734589];
    //bhr, ahr = butter(1, 5.5, 'hp', fs=250)
    // let bhr = vec![0.9521018, -0.9521018];
    // let ahr = vec![1.0, -0.90420359];

    let out = my_filtfilt(&blr, &alr, &ch);
    let out = out
        .iter()
        .map(|&x| x * 2000.0)
        .collect();
    let out = my_filtfilt(&bhr, &ahr, &out);
    out
}

fn sum_ch(ch1: &Vec<f32>, ch2: &Vec<f32>, ch3: &Vec<f32>) -> Vec<f32> {
    let mut sum_ch: Vec<f32> = ch1.to_owned();
    for i in 0..ch1.len() - 1 {
        if ((ch1[i] > ch2[i]) && (ch1[i] < ch3[i])) || ((ch1[i] > ch3[i]) && (ch1[i] < ch2[i])) {
            sum_ch[i] = ch1[i];
        }
        if ((ch2[i] > ch1[i]) && (ch2[i] < ch3[i])) || ((ch2[i] > ch3[i]) && (ch2[i] < ch1[i])) {
            sum_ch[i] = ch2[i];
        }
        if ((ch3[i] > ch1[i]) && (ch3[i] < ch2[i])) || ((ch3[i] > ch2[i]) && (ch3[i] < ch1[i])) {
            sum_ch[i] = ch3[i];
        }
    }
    sum_ch
}

pub fn del_isoline(ch: &Vec<f32>) -> Vec<f32> {
    let len_win = 120;
    let mut out = ch.to_owned();
    let mut buff = vec![0.0; len_win];
    for i in 63..ch.len() {
        let j = i % len_win;
        buff[j] = ch[i];
        let mut sort_buff: Vec<_> = buff.to_vec().iter().step_by(7).cloned().collect();
        sort_buff.sort_by(|a, b| a.partial_cmp(b).unwrap());
        out[i - 60 - 3] = ch[i - 60 - 3] - sort_buff[8];
        out[i - 60 - 2] = ch[i - 60 - 2] - sort_buff[8];
        out[i - 60 - 1] = ch[i - 60 - 1] - sort_buff[8];
        out[i - 60] = ch[i - 60] - sort_buff[8];
        out[i - 60 + 1] = ch[i - 60 + 1] - sort_buff[8];
        out[i - 60 + 2] = ch[i - 60 + 2] - sort_buff[8];
        out[i - 60 + 3] = ch[i - 60 + 3] - sort_buff[8];
    }
    out
}

fn pre_proc_lead(ch: &Vec<f32>) -> Vec<f32> {
    let ch = clean_ch(&ch);
    let p2p_ch = get_p2p(&ch, 40, false);
    let art = del_artifacts(&ch, &p2p_ch);
    let ch = del_isoline(&art.0);
    let fch = del_nouse(&ch, &art.1);
    let fch = get_p2p(&fch, 30, true);
    let fch = filt_r(&fch);
    fch
}

pub fn pre_proc_r(leads: &mut Ecg) -> Vec<f32> {
    let ch1 = leads.lead1.to_owned();
    let ch2 = leads.lead2.to_owned();
    let ch3 = leads.lead3.to_owned();

    let (tx1, rx1) = mpsc::channel::<Vec<f32>>();
    let (tx11, rx11) = mpsc::channel::<Vec<f32>>();
    let (tx2, rx2) = mpsc::channel::<Vec<f32>>();
    let (tx22, rx22) = mpsc::channel::<Vec<f32>>();
    let (tx3, rx3) = mpsc::channel::<Vec<f32>>();
    let (tx33, rx33) = mpsc::channel::<Vec<f32>>();

    tx1.send(ch1).unwrap();
    let t1 = thread::spawn(move || {
        let received = rx1.recv().unwrap();
        let fch = pre_proc_lead(&received);
        tx11.send(fch).unwrap();
    });
    tx2.send(ch2).unwrap();
    let t2 = thread::spawn(move || {
        let received = rx2.recv().unwrap();
        let fch = pre_proc_lead(&received);
        tx22.send(fch).unwrap();
    });
    tx3.send(ch3).unwrap();
    let t3 = thread::spawn(move || {
        let received = rx3.recv().unwrap();
        let fch = pre_proc_lead(&received);
        tx33.send(fch).unwrap();
    });

    let fch1 = rx11.recv().unwrap();
    let fch2 = rx22.recv().unwrap();
    let fch3 = rx33.recv().unwrap();
    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();

    sum_ch(&fch1, &fch2, &fch3)
}


