use std::io::Error;

use hall::core::{CompanyType, MarketType};
use vagabond::data::VagabondBuild;

use crate::data::DbBuild;
use crate::save_load::save_data_single;
use crate::save_load::vagabond::make_glyph_char;

fn make_vagabond_build(build_instance: &DbBuild) -> VagabondBuild {
    VagabondBuild {
        build: build_instance.build,
        number: build_instance.number,
        title: build_instance.title.clone(),
    }
}

pub(crate) fn output_builds_for_vagabond(builds: &[DbBuild], company: Vec<(CompanyType, String, String)>, market: Vec<(MarketType, String, String)>) -> Result<(), Error> {
    let vagabond_builds = builds.iter().map(make_vagabond_build).collect::<Vec<_>>();
    let vagabond_company = company.into_iter().map(make_glyph_char).collect::<Vec<_>>();
    let vagabond_market = market.into_iter().map(make_glyph_char).collect::<Vec<_>>();
    save_data_single(vagabond_builds, "output/vagabond_builds.ron")?;
    save_data_single((vagabond_company, vagabond_market), "output/vagabond_builds_meta.ron")?;

    Ok(())
}
