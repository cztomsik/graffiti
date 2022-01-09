use fontdb::{Database, Family, Query};
use once_cell::sync::Lazy;

pub use ab_glyph::{Font, FontArc, Glyph, GlyphId, ScaleFont};

pub static FONT_DB: Lazy<Database> = Lazy::new(|| {
    let mut db = Database::new();
    db.load_system_fonts();

    let id = db
        .query(&Query {
            families: &[
                Family::Name("Helvetica"),
                Family::Name("FreeSans"),
                Family::Name("Arial"),
            ],
            ..Query::default()
        })
        .expect("no sans-serif found");

    db.set_sans_serif_family(db.face(id).unwrap().family.clone());

    db
});

pub(crate) static SANS_SERIF_FONT: Lazy<FontArc> = Lazy::new(|| {
    let id = FONT_DB
        .query(&Query {
            families: &[Family::SansSerif],
            ..Query::default()
        })
        .unwrap();

    FONT_DB
        .with_face_data(id, |data, _| FontArc::try_from_vec(data.to_owned()).unwrap())
        .unwrap()
});

/*
use crate::util::{Atom, SlotMap};
use fontdb::{Database, Family, Query};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FontQuery {
    family: Atom<String>,
    weight: u16,
    italic: bool,
}
*/
