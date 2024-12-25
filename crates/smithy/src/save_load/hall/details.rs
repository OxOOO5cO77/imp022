use crate::data::DbDetail;
use crate::save_load::save_data_single;
use hall::data::hall::HallDetail;
use std::io::Error;

fn make_hall_detail(detail_instance: &DbDetail) -> HallDetail {
    HallDetail {
        detail: detail_instance.detail,
        number: detail_instance.number,
        cards: detail_instance.cards.clone(),
    }
}

pub(crate) fn output_details_for_hall(details: &[DbDetail]) -> Result<(), Error> {
    let hall_details = details.iter().map(make_hall_detail).collect::<Vec<_>>();
    save_data_single(hall_details, "output/hall_details.ron")
}
