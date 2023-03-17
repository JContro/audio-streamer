
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample};
use std::fs::File;
use std::io::BufWriter;
use std::sync::{Arc, Mutex};
use anyhow;


fn main() -> Result<(), anyhow::Error> {
    
    let host = cpal::default_host();

    // List all available input devices and save them in a vector.
    let mut input_devices = vec![];
    let mut index = 1;
    println!("Please select an input device by index:");
    
    for device in host.input_devices()? {
        println!("{}. {}", index, device.name()?);
        input_devices.push(device);
        index += 1;
    }

    // Get the user's input device selection.
    let mut input_device_index = String::new();
    std::io::stdin().read_line(&mut input_device_index)?;

    println!("You selected: {}", input_device_index);
    let input_device_index: usize = input_device_index.trim().parse::<usize>().unwrap() -1;

    // Get the device from the vector.
    let device = input_devices.get(input_device_index).unwrap();

    println!("Input device: {}", device.name()?);
    

    
    let config = device
        .default_input_config()
        .expect("Failed to get default input config");
    println!("Default input config: {:?}", config);

    // The WAV file we're recording to.
    const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/recorded.wav");
    // println!("Recording to {}", PATH);
    // let spec = wav_spec_from_config(&config);
    // let writer = hound::WavWriter::create(PATH, spec)?;
    // let writer = Arc::new(Mutex::new(Some(writer)));

    // A flag to indicate that recording is in progress.
    // println!("Begin recording...");

    // Run the input stream on a separate thread.
    // let writer_2 = writer.clone();

    let err_fn = move |err| {
        eprintln!("an error occurred on stream: {}", err);
    };

    let stream = match config.sample_format() {
        cpal::SampleFormat::I8 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| print_input_data(data),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::I16 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| print_input_data(data),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::I32 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| print_input_data(data),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| print_input_data(data),
            err_fn,
            None,
        )?,
        sample_format => {
            return Err(anyhow::Error::msg(format!(
                "Unsupported sample format '{sample_format}'"
            )))
        }
    };

    stream.play()?;

    // Let recording go for roughly three seconds.
    std::thread::sleep(std::time::Duration::from_secs(1));
    drop(stream);
    // writer.lock().unwrap().take().unwrap().finalize()?;
    // println!("Recording {} complete!", PATH);

    // let mut reader = hound::WavReader::open(PATH)?;
    // let samples: Vec<f32> = reader.samples().map(|s| s.unwrap()).collect();
    // println!("Samples: {:?}", samples);

    Ok(())

}

fn sample_format(format: cpal::SampleFormat) -> hound::SampleFormat {
    if format.is_float() {
        hound::SampleFormat::Float
    } else {
        hound::SampleFormat::Int
    }
}

fn wav_spec_from_config(config: &cpal::SupportedStreamConfig) -> hound::WavSpec {
    hound::WavSpec {
        channels: config.channels() as _,
        sample_rate: config.sample_rate().0 as _,
        bits_per_sample: (config.sample_format().sample_size() * 8) as _,
        sample_format: sample_format(config.sample_format()),
    }
}

type WavWriterHandle = Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>;


fn print_input_data(input: &[f32]) {
    println!("Input data: {:?}", input);
    panic!("Stop recording")
}