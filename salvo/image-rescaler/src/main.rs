use salvo::prelude::*;

const IMAGE: &[u8] = include_bytes!("logo-square.png");

#[handler]
async fn get_image(res: &mut Response) {
    let _ = res.write_body(vec![97u8]);
}
// #[handler]
// async fn send_file(req: &mut Request, res: &mut Response) {
//     NamedFile::builder("/file/to/path").attached_name("image.png").send(req.headers(), res).await;
// }

#[shuttle_runtime::main]
async fn salvo() -> shuttle_salvo::ShuttleSalvo {
    let router = Router::new().get(get_image);

    Ok(router.into())
}
