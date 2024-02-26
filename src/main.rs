use std::error::Error;
use std::io::Write;

type RenderResult = Result<(), Box<dyn Error>>;

fn main() -> RenderResult {
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;
    let log = &mut std::io::stderr();
    let output = &mut std::io::stdout();
    write_ppm_header(output, IMAGE_WIDTH, IMAGE_HEIGHT)?;
    for i in 0..IMAGE_HEIGHT {
        for j in 0..IMAGE_WIDTH {
            let r = ((i as f32 / (IMAGE_WIDTH - 1) as f32) * 256.0) as u8;
            let g = ((j as f32 / (IMAGE_HEIGHT - 1) as f32) * 256.0) as u8;
            let b = 0;
            writeln!(output, "{} {} {}", r, g, b)?;
        }
        let _ = writeln!(log, "Scanline progress: {}/{}", i + 1, IMAGE_HEIGHT);
    }
    output.flush()?;
    Ok(())
}

fn write_ppm_header(output: &mut impl Write, width: usize, height: usize) -> RenderResult {
    writeln!(output, "P3")?;
    writeln!(output, "{} {}", width, height)?;
    writeln!(output, "255")?;
    Ok(())
}
