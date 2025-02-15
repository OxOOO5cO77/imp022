use std::io::Error;

use hall_lib::core::{GeneralType, SpecificType};
use vagabond_lib::data::VagabondDetail;

use crate::data::DbDetail;
use crate::save_load::save_data_single;
use crate::save_load::vagabond::make_glyph_char;

fn make_vagabond_detail(detail_instance: &DbDetail) -> VagabondDetail {
    VagabondDetail {
        detail: detail_instance.detail,
        number: detail_instance.number,
        title: detail_instance.title.clone(),
    }
}

pub(crate) fn output_details_for_vagabond(details: &[DbDetail], general: Vec<(GeneralType, String, String)>, specific: Vec<(SpecificType, String, String)>) -> Result<(), Error> {
    let vagabond_details = details.iter().map(make_vagabond_detail).collect::<Vec<_>>();
    let vagabond_general = general.into_iter().map(make_glyph_char).collect::<Vec<_>>();
    let vagabond_specific = specific.into_iter().map(make_glyph_char).collect::<Vec<_>>();
    save_data_single(vagabond_details, "output/vagabond_details.ron")?;
    save_data_single((vagabond_general, vagabond_specific), "output/vagabond_details_meta.ron")?;

    Ok(())
}
