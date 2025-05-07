fn main() {
    let start = std::time::Instant::now();
    let retry_after = "Fri, 01 Jan 2100 00:00:00 GMT";
    // Second option: Retry-After is a future HTTP date string that tells us when to retry.
    let expected = jiff::Zoned::now()
        .until(
            &jiff::civil::date(2100, 1, 1)
                .at(0, 0, 0, 0)
                .to_zoned(jiff::tz::TimeZone::UTC)
                .unwrap(),
        )
        .unwrap()
        .total(jiff::Unit::Millisecond)
        .unwrap() as u64;

    let retry_time = jiff::fmt::rfc2822::parse(retry_after).unwrap();
    let actual = jiff::Timestamp::now()
        .until(&retry_time)
        .unwrap()
        .total(jiff::Unit::Millisecond)
        .unwrap() as u64;
    let diff = expected.abs_diff(actual.into());
    eprintln!("diff={diff:?}",);
    eprintln!("elapsed={:?}", start.elapsed());
}
