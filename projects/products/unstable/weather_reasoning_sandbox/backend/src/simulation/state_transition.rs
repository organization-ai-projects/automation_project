use crate::domain::observation_record::ObservationRecord;
use crate::domain::weather_state::WeatherState;

pub struct StateTransition;

impl StateTransition {
    pub fn derive(prior: &WeatherState, observations: &[ObservationRecord]) -> WeatherState {
        if observations.is_empty() {
            return prior.clone();
        }

        let count = observations.len() as f64;
        let avg_pressure: f64 = observations.iter().map(|o| o.pressure_hpa).sum::<f64>() / count;
        let avg_temp: f64 = observations.iter().map(|o| o.temperature_c).sum::<f64>() / count;
        let avg_humidity: f64 = observations.iter().map(|o| o.humidity_pct).sum::<f64>() / count;
        let avg_wind: f64 = observations.iter().map(|o| o.wind_speed_kmh).sum::<f64>() / count;
        let avg_cloud: f64 = observations.iter().map(|o| o.cloudiness_pct).sum::<f64>() / count;
        let avg_precip: f64 = observations.iter().map(|o| o.precipitation_mm).sum::<f64>() / count;

        let pressure_trend = avg_pressure - prior.pressure_hpa;
        let temperature_trend = avg_temp - prior.temperature_c;
        let humidity_trend = avg_humidity - prior.humidity_pct;
        let wind_trend = avg_wind - prior.wind_speed_kmh;

        let precipitation_likelihood =
            Self::compute_precip_likelihood(avg_humidity, avg_precip, avg_cloud);
        let storm_likelihood =
            Self::compute_storm_likelihood(pressure_trend, avg_wind, avg_humidity, avg_cloud);
        let instability_index =
            Self::compute_instability(pressure_trend, humidity_trend, wind_trend);
        let confidence = Self::compute_confidence(count, pressure_trend);

        WeatherState {
            pressure_hpa: avg_pressure,
            pressure_trend,
            temperature_c: avg_temp,
            temperature_trend,
            humidity_pct: avg_humidity,
            humidity_trend,
            wind_speed_kmh: avg_wind,
            wind_trend,
            cloudiness_pct: avg_cloud,
            precipitation_likelihood,
            storm_likelihood,
            instability_index,
            confidence,
        }
    }

    fn compute_precip_likelihood(humidity: f64, precip_mm: f64, cloud: f64) -> f64 {
        let base =
            (humidity / 100.0) * 0.4 + (precip_mm / 50.0).min(1.0) * 0.4 + (cloud / 100.0) * 0.2;
        base.clamp(0.0, 1.0)
    }

    fn compute_storm_likelihood(pressure_trend: f64, wind: f64, humidity: f64, cloud: f64) -> f64 {
        let pressure_factor = if pressure_trend < -5.0 {
            0.4
        } else if pressure_trend < -2.0 {
            0.2
        } else {
            0.0
        };
        let wind_factor = (wind / 100.0).min(1.0) * 0.3;
        let humidity_factor = if humidity > 80.0 { 0.2 } else { 0.0 };
        let cloud_factor = (cloud / 100.0) * 0.1;
        (pressure_factor + wind_factor + humidity_factor + cloud_factor).clamp(0.0, 1.0)
    }

    fn compute_instability(pressure_trend: f64, humidity_trend: f64, wind_trend: f64) -> f64 {
        let pt = pressure_trend.abs() / 20.0;
        let ht = humidity_trend.abs() / 50.0;
        let wt = wind_trend.abs() / 30.0;
        ((pt + ht + wt) / 3.0).clamp(0.0, 1.0)
    }

    fn compute_confidence(obs_count: f64, pressure_trend: f64) -> f64 {
        let obs_factor = (obs_count / 5.0).min(1.0) * 0.5;
        let stability_factor = (1.0 - pressure_trend.abs() / 20.0).max(0.0) * 0.5;
        (obs_factor + stability_factor).clamp(0.1, 1.0)
    }
}
