use once_cell::sync::Lazy;
use owned_ttf_parser::{AsFaceRef, OwnedFace};
use std::sync::Arc;

#[derive(Debug)]
pub struct Font {
    pub(crate) face: OwnedFace,
    pub(crate) scale: f32,
}

pub(crate) static SANS_SERIF_FACE: Lazy<Arc<Font>> = Lazy::new(|| {
    use fontdb::{Database, Family, Query};

    let mut db = Database::new();
    db.set_sans_serif_family("Arial");
    db.load_system_fonts();

    let id = db
        .query(&Query {
            families: &[Family::SansSerif],
            ..Default::default()
        })
        .expect("no default font");

    let face = db
        .with_face_data(id, |data, i| OwnedFace::from_vec(data.to_owned(), i).unwrap())
        .unwrap();
    let scale = 16. / face.as_face_ref().units_per_em().unwrap() as f32;

    Arc::new(Font { face, scale })
});
