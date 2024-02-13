use std::io::{Read, Write};

use anyhow::*;
use cairo::Context;
use poppler::{PopplerDocument, PopplerPage};

pub fn pdf2png<T, S>(mut pdf_reader: T) -> anyhow::Result<S>
where
    T: Read,
    S: Write + Default,
{
    let mut buf: Vec<u8> = Vec::new();
    let _ = pdf_reader.read_to_end(&mut buf);
    let doc = PopplerDocument::new_from_data(&mut buf, None)?;

    let page: PopplerPage = doc.get_page(0).ok_or(anyhow!("PDF has no pages"))?;

    let (width, height) = page.get_size();

    let surface = cairo::PdfSurface::new(width, height, "output.pdf")?;

    let ctx = Context::new(&surface)?;
    ctx.set_source_rgb(1.0 as f64, 1.0 as f64, 1.0 as f64);
    let _ = ctx.paint()?;
    page.render(&ctx);

    let mut writer = S::default();
    surface.write_to_png(&mut writer)?;

    return Ok(writer);
}
