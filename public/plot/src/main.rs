use std::{collections::BTreeMap, fs::File, path::Path};

use anyhow::Result;
use colorgrad::{Color, CustomGradient};
use image::{ImageBuffer, Rgb, Rgba};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    lon: f64,
    lat: f64,
    updated: u64,
}

fn main() -> Result<()> {
    let path = Path::new("/home/joel/Downloads/MLS-full-cell-export-final.csv.gz");
    let mut pixels = BTreeMap::new();
    for y in -900..900 {
        for x in -1800..1800 {
            pixels.insert((x, y), 0usize);
        }
    }

    let mut reader = csv::Reader::from_reader(flate2::read::MultiGzDecoder::new(File::open(path)?));
    let mut i = 0;
    for result in reader.deserialize() {
        let record: Record = result?;
        *pixels
            .get_mut(&(
                (record.lon * 10.0).floor() as i32,
                (record.lat * 10.0).floor() as i32 * -1,
            ))
            .unwrap() += 1;

        if (i % 1_000_000) == 0 {
            eprintln!("{}", i / 1_000_000);
            // break;
        }
        i += 1;
    }

    let g = CustomGradient::new()
        .colors(&[
            Color::from_rgba8(19, 78, 74, 255),
            Color::from_rgba8(45, 212, 191, 255),
        ])
        .build()?;
    let mut img = ImageBuffer::new(3600, 1800);
    for ((x, y), count) in pixels {
        let rgba = if count == 0 {
            [0, 0, 0, 0]
        } else {
            g.at(count as f64 / 300.0).to_rgba8()
        };

        let x = (x + 1800) as u32;
        let y = (y + 900) as u32;
        img.put_pixel(x, y, Rgba(rgba));
    }
    img.save("../plot.png")?;

    Ok(())
}
