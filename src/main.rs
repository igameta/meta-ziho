use vvcore::*;
use mp3lame_encoder::{Bitrate, Builder, Encoder, FlushNoGap, MonoPcm, Quality};

enum Speaker{
    MetanAma = 0,
    ZundAma = 1,
    Chibi =42,
}

fn main(){
    let dict_dir = std::ffi::CString::new("open_jtalk_dic_utf_8-1.11").unwrap();
    let speaker: Speaker = Speaker::ZundAma;
    let vvc = VoicevoxCore::new_from_options(AccelerationMode::Auto, 0, false, dict_dir.as_c_str()).unwrap();
    vvc.load_model(speaker as u32).unwrap();
    
    for i in 0..=12 {
        let sound = vvc.tts_simple(&format!("{}時", i), speaker as u32).unwrap();
        let mut encoder = Builder::new().expect("Create LAME builder");
        encoder.set_num_channels(1).expect("set channels");
        encoder.set_sample_rate(44_100).expect("set sample rate");
        encoder.set_brate(Bitrate::Kbps16).expect("set brate");
        encoder.set_quality(Quality::Decent).expect("set quality");

        let mut encoder = encoder.build().expect("To innitialize LAME enocder");

        let pcm = MonoPcm(sound.as_slice());
        let mut out_buffer = Vec::new();
        out_buffer.reserve(mp3lame_encoder::max_required_buffer_size(pcm.len()));
        let encoded_size = mp3_encoder.flush::<FlushNoGap>(out_buffer.spare_capacity_mut(), pcm).expect("To encode");
    }


}