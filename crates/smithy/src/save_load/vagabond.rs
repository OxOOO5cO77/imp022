mod builds;
mod cards;
mod details;

pub(crate) use builds::output_builds_for_vagabond;
pub(crate) use cards::output_cards_for_vagabond;
pub(crate) use details::output_details_for_vagabond;

fn make_glyph_char<T>((id, name, glyph): (T, String, String)) -> (T, String, char) {
    (id, name, glyph.chars().next().unwrap())
}
