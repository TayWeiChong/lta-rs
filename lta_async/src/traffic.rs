//! All APIs pertaining to traffic

use crate::lta_client::LTAClient;
use crate::{build_req_async_with_query, build_req_async_with_skip};
use lta_models::traffic::{
    bike_parking, carpark_avail, erp_rates, est_travel_time, faulty_traffic_lights, road,
    traffic_images, traffic_incidents, traffic_speed_bands, vms_emas,
};
use lta_utils_commons::LTAResult;

/// Returns ERP rates of all vehicle types across all timings for each
/// zone.
///
/// **Update freq**: Ad-Hoc
pub async fn get_erp_rates(
    client: &LTAClient,
    skip: Option<u32>,
) -> LTAResult<Vec<erp_rates::ErpRate>> {
    build_req_async_with_skip::<erp_rates::ErpRatesResp, _>(client, erp_rates::URL, skip).await
}

/// Returns no. of available lots for HDB, LTA and URA carpark data.
/// The LTA carpark data consist of major shopping malls and developments within
/// Orchard, Marina, HarbourFront, Jurong Lake District.
/// (Note: list of LTA carpark data available on this API is subset of those listed on
/// One.Motoring and MyTransport Portals)
///
/// **Update freq**: 1 min
pub async fn get_carpark_avail(
    client: &LTAClient,
    skip: Option<u32>,
) -> LTAResult<Vec<carpark_avail::CarPark>> {
    build_req_async_with_skip::<carpark_avail::CarparkAvailResp, _>(
        client,
        carpark_avail::URL,
        skip,
    )
    .await
}

/// Returns estimated travel times of expressways (in segments).
///
/// **Update freq**: 5min
pub async fn get_est_travel_time(
    client: &LTAClient,
    skip: Option<u32>,
) -> LTAResult<Vec<est_travel_time::EstTravelTime>> {
    build_req_async_with_skip::<est_travel_time::EstTravelTimeResp, _>(
        client,
        est_travel_time::URL,
        skip,
    )
    .await
}

/// Returns alerts of traffic lights that are currently faulty, or currently
/// undergoing scheduled maintenance.
///
/// **Update freq**: 2min or whenever there are updates
pub async fn get_faulty_traffic_lights(
    client: &LTAClient,
    skip: Option<u32>,
) -> LTAResult<Vec<faulty_traffic_lights::FaultyTrafficLight>> {
    build_req_async_with_skip::<faulty_traffic_lights::FaultyTrafficLightResp, _>(
        client,
        faulty_traffic_lights::URL,
        skip,
    )
    .await
}

/// Returns all planned road openings
///
/// **Update freq**: 24 hours – whenever there are updates
pub async fn get_road_details(
    client: &LTAClient,
    road_details_type: road::RoadDetailsType,
    skip: Option<u32>,
) -> LTAResult<Vec<road::RoadDetails>> {
    let url = match road_details_type {
        road::RoadDetailsType::RoadOpening => road::URL_ROAD_OPENING,
        road::RoadDetailsType::RoadWorks => road::URL_ROAD_WORKS,
    };

    build_req_async_with_skip::<road::RoadDetailsResp, _>(client, url, skip).await
}

/// Returns links to images of live traffic conditions along expressways and
/// Woodlands & Tuas Checkpoints.
///
/// **Update freq**: 1 to 5 minutes
pub async fn get_traffic_images(
    client: &LTAClient,
    skip: Option<u32>,
) -> LTAResult<Vec<traffic_images::TrafficImage>> {
    build_req_async_with_skip::<traffic_images::TrafficImageResp, _>(
        client,
        traffic_images::URL,
        skip,
    )
    .await
}

/// Returns current traffic speeds on expressways and arterial roads,
/// expressed in speed bands.
///
/// **Update freq**: 5 minutes
pub async fn get_traffic_incidents(
    client: &LTAClient,
    skip: Option<u32>,
) -> LTAResult<Vec<traffic_incidents::TrafficIncident>> {
    build_req_async_with_skip::<traffic_incidents::TrafficIncidentResp, _>(
        client,
        traffic_incidents::URL,
        skip,
    )
    .await
}

/// Returns current traffic speeds on expressways and arterial roads,
/// expressed in speed bands.
///
/// **Update freq**: 5 minutes
pub async fn get_traffic_speed_band(
    client: &LTAClient,
    skip: Option<u32>,
) -> LTAResult<Vec<traffic_speed_bands::TrafficSpeedBand>> {
    build_req_async_with_skip::<traffic_speed_bands::TrafficSpeedBandResp, _>(
        client,
        traffic_speed_bands::URL,
        skip,
    )
    .await
}

/// Returns traffic advisories (via variable message services) concerning
/// current traffic conditions that are displayed on EMAS signboards
/// along expressways and arterial roads.
///
/// **Update freq**: 2 minutes
pub async fn get_vms_emas(client: &LTAClient, skip: Option<u32>) -> LTAResult<Vec<vms_emas::VMS>> {
    build_req_async_with_skip::<vms_emas::VMSResp, _>(client, vms_emas::URL, skip).await
}

/// Returns bicycle parking locations within a radius
///
/// Dist is default to 0.5 even if you provide `None`
/// No skip, endpoint can return more than 500 records
/// **Update freq**: Monthly
pub async fn get_bike_parking(
    client: &LTAClient,
    lat: f64,
    long: f64,
    dist: Option<f64>,
) -> LTAResult<Vec<bike_parking::BikeParking>> {
    let dist = dist.unwrap_or(0.5);
    build_req_async_with_query::<bike_parking::BikeParkingResp, _, _>(
        client,
        bike_parking::URL,
        move |rb| rb.query(&[("Lat", lat), ("Long", long), ("Dist", dist)]),
    )
    .await
}
