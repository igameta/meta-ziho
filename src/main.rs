use vvcore::*;
use chrono::{Local, DateTime};
use rodio::Decoder;

fn main() {
    // 時刻の取得
    let now: DateTime<Local> = Local::now();
    let str_time = now.format("%H時%M分").to_string();
    print!("{}", str_time);

    // 音声合成と音声データの加工
    let jtalk_dict_dir = std::ffi::CString::new("open_jtalk_dic_utf_8").unwrap();
    let vvc = VoicevoxCore::new_from_options(AccelerationMode::Auto, 0, true, jtalk_dict_dir.as_c_str()).unwrap();
    let speaker: u32 = 1;
    let sound = vvc.tts_simple(&str_time, speaker).unwrap();
    let cursor = std::io::Cursor::new(sound.as_slice().to_vec());

    // 生成確認用
    // let mut file = std::fs::File::create("output.wav").unwrap();
    // file.write_all(&sound.as_slice()).unwrap();

    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    let source = Decoder::new(cursor).unwrap();
    sink.append(source);
    sink.sleep_until_end();
    
}
