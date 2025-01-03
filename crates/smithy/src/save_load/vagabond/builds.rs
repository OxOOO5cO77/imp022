use crate::data::DbBuild;
use crate::save_load::save_data_single;
use hall::data::core::{CompanyType, MarketType};
use std::io::Error;
use vagabond::data::VagabondBuild;

fn make_vagabond_build(build_instance: &DbBuild) -> VagabondBuild {
    VagabondBuild {
        build: build_instance.build,
        number: build_instance.number,
        title: build_instance.title.clone(),
    }
}

pub(crate) fn output_builds_for_vagabond(builds: &[DbBuild], company: Vec<(CompanyType, String)>, market: Vec<(MarketType, String)>) -> Result<(), Error> {
    let vagabond_builds = builds.iter().map(make_vagabond_build).collect::<Vec<_>>();
    save_data_single(vagabond_builds, "output/vagabond_builds.ron")?;
    save_data_single((company, market), "output/vagabond_builds_meta.ron")?;

    Ok(())
}
