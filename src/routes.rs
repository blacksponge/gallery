use actix_web::{HttpRequest, Result, Either, fs::NamedFile};

use crate::utils::*;
use crate::models::{Album, Photo, PhotoThumbnail};
use crate::config::Config;


pub fn gallery_route(req: &HttpRequest<Config>) -> Result<Either<Album, Photo>> {
    let path = get_album_canonical_path(req.match_info().query("path")?, req.state());
    if is_path_album(&path) {
        Ok(Either::A(Album::from_path(path, req.state())?))
    } else {
        Ok(Either::B(Photo::from_path(path)?))
    }
}

pub fn small_thumbnail_route(req: &HttpRequest<Config>) -> Result<NamedFile> {
    let path = get_album_canonical_path(req.match_info().query("path")?, req.state());
    let config = req.state();

    Ok(NamedFile::open(PhotoThumbnail::get_image(
        path,
        config.small_thumbnail.size,
        config.small_thumbnail.square,
        config
    )?)?)
}

pub fn full_photo_route(req: &HttpRequest<Config>) -> Result<NamedFile> {
    let path = get_album_canonical_path(req.match_info().query("path")?, req.state());

    Ok(NamedFile::open(path)?)
}
