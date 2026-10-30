//! Times a model on the CPU and on a GPU through ONNX Runtime: gpuprobe <model> <size> <cpu|dml:N|webgpu> <runs>
use std::time::Instant;

use ort::ep;
use ort::session::Session;
use ort::value::Tensor;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (model, size, provider, runs) = (
        &args[1],
        args[2].parse::<usize>().unwrap(),
        args[3].as_str(),
        args[4].parse::<usize>().unwrap(),
    );
    let environment = ort::init().with_telemetry(false).build().expect("environment");
    let mut builder = Session::builder(&environment).expect("builder");
    if let Some(device) = provider.strip_prefix("dml:") {
        let device: i32 = device.parse().unwrap();
        builder = builder
            .with_execution_providers([ep::DirectML::default().with_device_id(device).build().error_on_failure()])
            .expect("directml");
    }
    let started = Instant::now();
    let mut session = builder.commit_from_file(model).expect("load");
    let load = started.elapsed();
    let name = session.inputs()[0].name().to_string();
    let values: Vec<f32> = (0..3 * size * size).map(|i| ((i * 31 % 255) as f32) / 255.0).collect();
    let mut times = Vec::new();
    let mut checksum = 0.0f32;
    for _ in 0..runs + 3 {
        let input = Tensor::from_array(([1usize, 3, size, size], values.clone())).unwrap();
        let started = Instant::now();
        let outputs = session.run(ort::inputs![name.as_str() => input]).expect("run");
        let (_, answer) = outputs[0].try_extract_tensor::<f32>().unwrap();
        checksum = answer.iter().take(1000).sum::<f32>();
        times.push(started.elapsed().as_secs_f64() * 1000.0);
    }
    let measured = &times[3..];
    let mean = measured.iter().sum::<f64>() / measured.len() as f64;
    println!(
        "{provider}: load {:.0} ms, first run {:.0} ms, mean of {runs} runs {mean:.1} ms, checksum {checksum:.3}",
        load.as_secs_f64() * 1000.0,
        times[0]
    );
}
