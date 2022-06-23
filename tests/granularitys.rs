use druid_io::query::definitions::Granularity;

#[test]
fn test_period_granularity() {
    let str = "{\"type\":\"period\",\"period\":\"P2D\",\"timeZone\":\"America/Los_Angeles\"}";
    
    let test_str = serde_json::to_string(&Granularity::period("P2D", "America/Los_Angeles"));
    assert_eq!(test_str.unwrap(), str);
}
