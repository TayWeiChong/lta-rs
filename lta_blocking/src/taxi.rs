use crate::lta_client::LTAClient;
use lta_models::taxi::taxi_avail;
use crate::build_req;
use lta_utils_commons::{LTAResult, Coordinates};

/// Returns location coordinates of all Taxis that are currently available for
/// hire. Does not include "Hired" or "Busy" Taxis.
///
/// **Update freq**: 1min
pub fn get_taxi_avail(client: &LTAClient) -> LTAResult<Vec<Coordinates>> {
    build_req::<taxi_avail::TaxiAvailResp, _>(client, taxi_avail::URL)
}