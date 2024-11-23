use mutter::{Model, ModelType};

fn main() {
    // Attempt to download the model
    let model = match Model::download(&ModelType::BaseEn) {
        Ok(model) => model,
        Err(e) => {
            eprintln!("Failed to download the model: {}", e);
            return;
        }
    };

    // Provide the path to the audio file
    let audio_file_path = "C:\\Users\\Akshaj\\Desktop\\WhatsApp.mp3";

    // Attempt to transcribe the audio
    match model.transcribe_audio(audio_file_path, false, false, None) {
        Ok(result) => println!("{}", result.as_text()),
        Err(e) => eprintln!("Failed to transcribe audio: {}", e),
    }
}
