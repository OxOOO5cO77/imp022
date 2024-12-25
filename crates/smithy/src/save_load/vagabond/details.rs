use crate::data::DbDetail;
use crate::save_load::save_data_single;
use hall::data::core::{GeneralType, SpecificType};
use std::collections::HashMap;
use std::io::Error;
use vagabond::data::VagabondDetail;

fn make_vagabond_detail(detail_instance: &DbDetail) -> VagabondDetail {
    VagabondDetail {
        detail: detail_instance.detail,
        number: detail_instance.number,
        title: detail_instance.title.clone(),
    }
}

pub(crate) fn output_details_for_vagabond(details: &[DbDetail], general: HashMap<GeneralType, String>, specific: HashMap<SpecificType, String>) -> Result<(), Error> {
    let vagabond_details = details.iter().map(make_vagabond_detail).collect::<Vec<_>>();
    save_data_single(vagabond_details, "output/vagabond_details.ron")?;
    save_data_single((general, specific), "output/vagabond_details_meta.ron")?;

    Ok(())
}
