use std::fs::File;
use std::io::Read;
use std::path::PathBuf;


pub struct Ecg {
    pub file_name: PathBuf,
    pub lead1: Vec<f32>,
    pub lead2: Vec<f32>,
    pub lead3: Vec<f32>,
}

impl Ecg {
    pub fn new() -> Ecg {
        let files = glob::glob("*.ecg").expect("Failed to read files");
        let fname = files.filter_map(Result::ok).next().unwrap();
        println!("{:?}", fname);
        let mut file = File::open(&fname).expect("Failed to open file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Failed to read file");

        // let ecg_data: Vec<i16> = buffer[1024..9_000_000]
        let ecg_data: Vec<i16> = buffer[1024..]
            .chunks(2)
            .map(|chunk| ((chunk[1] as i32) << 8 | (chunk[0] as i32)) as i16)
            .collect();

        let ch_1: Vec<f32> = ecg_data
            .iter()
            .step_by(3)
            .map(|&val| (-val as f32 + 1024.0) / 100.0)
            .collect();

        let ch_2: Vec<f32> = ecg_data
            .iter()
            .skip(1)
            .step_by(3)
            .map(|&val| (-val as f32 + 1024.0) / 100.0)
            .collect();

        let ch_3: Vec<f32> = ecg_data
            .iter()
            .skip(2)
            .step_by(3)
            .map(|&val| (-val as f32 + 1024.0) / 100.0)
            .collect();

        let mean_ch_1: f32 = ch_1.iter().sum::<f32>() / ch_1.len() as f32;
        let mean_ch_2: f32 = ch_2.iter().sum::<f32>() / ch_2.len() as f32;
        let mean_ch_3: f32 = ch_3.iter().sum::<f32>() / ch_3.len() as f32;

        let ch_1: Vec<f32> = ch_1.iter().map(|&val| val - mean_ch_1).collect();
        let ch_2: Vec<f32> = ch_2.iter().map(|&val| val - mean_ch_2).collect();
        let ch_3: Vec<f32> = ch_3.iter().map(|&val| val - mean_ch_3).collect();

        Ecg {
            file_name: fname,
            lead1: ch_1,
            lead2: ch_2,
            lead3: ch_3,
        }
    }
}

#[test]
fn it_works() {
    let leads = Ecg::new();
    assert_ne!(leads.lead1.len(), 0);
}