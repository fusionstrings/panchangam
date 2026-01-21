use panchangam::vedic::tithi::{calculate_tithi, tithi_end_time, tithi_start_time};
use panchangam::vedic::nakshatra::{calculate_nakshatra, nakshatra_end_time};
use panchangam::vedic::yoga::{calculate_yoga, yoga_end_time};
use panchangam::vedic::karana::{calculate_karana, karana_end_time};
use panchangam::vedic::panchang::calculate_daily_panchang;
use panchangam::Location;
use panchangam::astronomy::ayanamsha::AyanamshaMode;
use panchangam::astronomy::planets::julian_day;

// Helper to check if time B is reasonably after time A
fn assert_future(start: f64, end: f64, name: &str) {
    if end <= start {
        panic!("{} end time {} is not after start time {}", name, end, start);
    }
    // Check it's not too far (e.g. > 2 days)
    if end - start > 2.0 {
        panic!("{} end time {} is too far from start time {} (diff {})", name, end, start, end - start);
    }
}

#[test]
fn test_tithi_consistency() {
    let jd = julian_day(2024, 1, 1, 12.0); // Noon
    
    let tithi = calculate_tithi(jd);
    let start = tithi_start_time(jd);
    let end = tithi_end_time(jd);
    
    println!("Tithi: {:?}, Start: {}, End: {}", tithi, start, end);
    
    assert!(start < jd, "Tithi start should be before current time");
    assert!(end > jd, "Tithi end should be after current time");
    assert_future(start, end, "Tithi");
    
    // Check continuity: End of Tithi X should be Start of Tithi X+1 (or X approx)
    let next_tithi = calculate_tithi(end + 0.01); // slightly after end
    let next_start = tithi_start_time(end + 0.01);
    
    assert!((end - next_start).abs() < 1e-4, "Continuity error: Tithi End {} vs Next Start {}", end, next_start);
    
    // Allow wrap around 30->1, otherwise index increment
    if tithi.index < 30 {
        assert_eq!(next_tithi.index, tithi.index + 1, "Tithi index should increment");
    } else {
        assert_eq!(next_tithi.index, 1, "Tithi 30 should wrap to 1");
    }
}

#[test]
fn test_nakshatra_consistency() {
    let jd = julian_day(2024, 1, 1, 12.0);
    let mode = AyanamshaMode::Lahiri;
    
    let nak = calculate_nakshatra(jd, mode);
    let end = nakshatra_end_time(jd, mode);
    
    println!("Nakshatra: {:?}, End: {}", nak, end);
    
    assert!(end > jd, "Nakshatra end should be after current time");
    assert_future(jd, end, "Nakshatra"); // roughly
    
    // Check continuity
    let next_nak = calculate_nakshatra(end + 0.01, mode);
    if nak.index < 27 {
        assert_eq!(next_nak.index, nak.index + 1);
    } else {
        assert_eq!(next_nak.index, 1);
    }
}

#[test]
fn test_yoga_consistency() {
    let jd = julian_day(2024, 1, 1, 12.0);
    let mode = AyanamshaMode::Lahiri;
    
    let yoga = calculate_yoga(jd, mode);
    let end = yoga_end_time(jd, mode);
    
    assert!(end > jd);
    assert_future(jd, end, "Yoga");
    
    let next_yoga = calculate_yoga(end + 0.01, mode);
    if yoga.index < 27 {
        assert_eq!(next_yoga.index, yoga.index + 1);
    } else {
        assert_eq!(next_yoga.index, 1);
    }
}

#[test]
fn test_karana_consistency() {
    let jd = julian_day(2024, 1, 1, 12.0);
    
    let karana = calculate_karana(jd);
    let end = karana_end_time(jd);
    
    assert!(end > jd);
    assert_future(jd, end, "Karana");
    
    // Karana index 1-60
    let next_karana = calculate_karana(end + 0.01);
    if karana.index < 60 {
        assert_eq!(next_karana.index, karana.index + 1);
    } else {
        assert_eq!(next_karana.index, 1);
    }
}

#[test]
fn test_daily_panchang_full() {
    let location = Location::new(28.6139, 77.2090, 0.0); // New Delhi
    let panchang = calculate_daily_panchang(2024, 1, 1, &location, 1).unwrap();
    
    println!("Full Panchang: {:?}", panchang);
    
    // 1. Check Start/End Times
    // Tithi
    assert!(panchang.tithi_end_time.is_some());
    if let Some(start) = panchang.tithi_start_time {
        assert!(start < panchang.tithi_end_time.unwrap());
        // Start should be around sunrise or before
        // assert!(start < panchang.sunrise + 86400000.0); // Rough check
    } else {
        // Start might be missing if it started way back? No, our logic searches 1.2 days back.
        // It should generally be found.
        panic!("Tithi start time not found");
    }
    
    // Nakshatra
    assert!(panchang.nakshatra_start_time.is_some());
    assert!(panchang.nakshatra_end_time.is_some());
    
    // 2. Check Dignity
    // Sun in Late Sagittarius (Jan 1) -> Friend/Own/etc?
    // Sun in Sag (9). Dignity::Friend.
    // Let's just check field exists and is valid enum variant
    let sun = panchang.planets.iter().find(|p| p.name == "Sun").unwrap();
    println!("Sun Dignity: {:?}", sun.dignity);
    
    // 3. Check Extended Muhurats
    assert!(panchang.muhurats.brahma_muhurta.end <= panchang.sunrise);
    assert!(panchang.muhurats.abhijit_muhurta.start > panchang.sunrise);
    assert!(panchang.muhurats.abhijit_muhurta.end < panchang.sunset);
}
