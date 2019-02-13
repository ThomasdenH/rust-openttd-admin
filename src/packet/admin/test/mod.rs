use super::server_packets;
use crate::packet::serde::from_bytes;

#[test]
fn test_company_economy_deser() {
    let buffer = &vec![
        0, 58, 133, 1, 0, 0, 0, 0, 0, 160, 134, 1, 0, 0, 0, 0, 0, 154, 254, 255, 255, 255, 255,
        255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let expected = server_packets::CompanyEconomy {
        id: 0,
        money: 99642,
        loan: 100000,
        income: 0,
        delivered_cargo: 0,
        company_value_last: 0,
        performance_last: 0,
        delivered_cargo_last: 0,
        company_value_previous: 0,
        performance_previous: 0,
        delivered_previous: 0,
    };
    assert_eq!(
        from_bytes::<server_packets::CompanyEconomy>(&buffer).unwrap(),
        expected
    );
}
