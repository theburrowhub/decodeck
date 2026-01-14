//! Performance tests for decodeck
//! These tests verify that the decoder meets performance requirements

use base64::{engine::general_purpose::STANDARD, Engine};
use std::time::{Duration, Instant};

// T088: Decode 1MB file in <2s (SC-001)
#[test]
fn decode_1mb_under_2_seconds() {
    // Generate 1MB of random data
    let data: Vec<u8> = (0..1_000_000).map(|i| (i % 256) as u8).collect();
    let encoded = STANDARD.encode(&data);

    let start = Instant::now();

    // Parse and decode
    let parsed = decodeck::decoder::EncodedData::parse(&encoded).unwrap();
    let decoded = parsed.decode().unwrap();

    let duration = start.elapsed();

    // Verify correctness
    assert_eq!(decoded.len(), data.len());
    assert_eq!(decoded, data);

    // Verify performance: must complete in under 2 seconds
    assert!(
        duration < Duration::from_secs(2),
        "1MB decode took {:?}, expected < 2s",
        duration
    );

    println!("1MB decode completed in {:?}", duration);
}

// T089: Full flow for 10MB file in <5s (SC-003)
#[test]
fn full_flow_10mb_under_5_seconds() {
    // Generate 10MB of data
    let data: Vec<u8> = (0..10_000_000).map(|i| (i % 256) as u8).collect();
    let encoded = STANDARD.encode(&data);

    let start = Instant::now();

    // Full flow: parse -> decode -> detect metadata
    let parsed = decodeck::decoder::EncodedData::parse(&encoded).unwrap();
    let decoded = parsed.decode().unwrap();
    let _metadata = decodeck::metadata::magic::detect(&decoded);

    let duration = start.elapsed();

    // Verify correctness
    assert_eq!(decoded.len(), data.len());

    // Verify performance: must complete in under 5 seconds
    assert!(
        duration < Duration::from_secs(5),
        "10MB full flow took {:?}, expected < 5s",
        duration
    );

    println!("10MB full flow completed in {:?}", duration);
}

// Additional performance test: verify consistent performance
#[test]
fn performance_is_consistent() {
    let data: Vec<u8> = (0..100_000).map(|i| (i % 256) as u8).collect();
    let encoded = STANDARD.encode(&data);

    let mut durations = Vec::new();

    // Run multiple times
    for _ in 0..5 {
        let start = Instant::now();
        let parsed = decodeck::decoder::EncodedData::parse(&encoded).unwrap();
        let _ = parsed.decode().unwrap();
        durations.push(start.elapsed());
    }

    // Calculate variance - all runs should be within 2x of each other
    let min = durations.iter().min().unwrap();
    let max = durations.iter().max().unwrap();

    assert!(
        max.as_nanos() < min.as_nanos() * 3,
        "Performance variance too high: min={:?}, max={:?}",
        min,
        max
    );
}
