use std::io::Error;

use hall_lib::hall::HallBuild;

use crate::data::DbBuild;
use crate::save_load::save_data_single;

fn make_hall_build(build_instance: &DbBuild) -> HallBuild {
    HallBuild {
        build: build_instance.build,
        number: build_instance.number,
        cards: build_instance.cards.clone(),
    }
}

pub(crate) fn output_builds_for_hall(builds: &[DbBuild]) -> Result<(), Error> {
    let hall_builds = builds.iter().map(make_hall_build).collect::<Vec<_>>();
    save_data_single(hall_builds, "output/hall_builds.ron")
}
