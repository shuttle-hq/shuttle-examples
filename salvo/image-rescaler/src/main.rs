use salvo::prelude::*;

const IMAGE: &[u8] = include_bytes!("logo-square.png");

#[handler]
async fn get_image(req: &mut Request, res: &mut Response) {
    let width = req.param::<u32>("width").unwrap();
    let height = req.param::<u32>("height").unwrap();

    let img = image::load_from_memory_with_format(IMAGE, image::ImageFormat::Png).unwrap();
    let img = img.resize_exact(width, height, image::imageops::FilterType::Triangle);
    let mut buffer = std::io::BufWriter::new(std::io::Cursor::new(Vec::new()));
    img.write_to(&mut buffer, image::ImageOutputFormat::Png)
        .unwrap();
    let bytes = buffer.into_inner().unwrap().into_inner();

    res.add_header("content-type", "image/png", true).unwrap();
    let _ = res.write_body(bytes);
}

#[shuttle_runtime::main]
async fn salvo() -> shuttle_salvo::ShuttleSalvo {
    let router = Router::with_path("<width:num>/<height:num>").get(get_image);

    Ok(router.into())
}
