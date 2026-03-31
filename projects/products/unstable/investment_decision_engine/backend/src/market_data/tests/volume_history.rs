use crate::market_data::volume_history::{VolumeEntry, VolumeHistory};

#[test]
fn average_volume_computes_correctly() {
    let vh = VolumeHistory::new(
        "AAPL",
        vec![
            VolumeEntry { date: "2025-01-13".to_string(), volume: 1000 },
            VolumeEntry { date: "2025-01-14".to_string(), volume: 2000 },
            VolumeEntry { date: "2025-01-15".to_string(), volume: 3000 },
        ],
    );
    let avg = vh.average_volume(3).unwrap();
    assert!((avg - 2000.0).abs() < f64::EPSILON);
}

#[test]
fn average_volume_recent_subset() {
    let vh = VolumeHistory::new(
        "AAPL",
        vec![
            VolumeEntry { date: "2025-01-13".to_string(), volume: 1000 },
            VolumeEntry { date: "2025-01-14".to_string(), volume: 2000 },
            VolumeEntry { date: "2025-01-15".to_string(), volume: 4000 },
        ],
    );
    let avg = vh.average_volume(2).unwrap();
    assert!((avg - 3000.0).abs() < f64::EPSILON);
}

#[test]
fn empty_volume_returns_none() {
    let vh = VolumeHistory::new("AAPL", vec![]);
    assert!(vh.average_volume(5).is_none());
}
