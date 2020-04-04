use covidcotra::*;

#[test]
fn test_tracking() {
    let authority = authority::Authority::unique();

    let user_2 = auth::Identity::unique();

    let mut user_1_log = contactlog::ContactLog::new();
    user_1_log.add(&user_2.new_share_id(authority.public_key()));

    let log = user_1_log.decode(authority.secret_key()).unwrap();

    assert_eq!(&log[0].0, user_2.unique_id());
    assert_eq!(&log[0].0.hash(), user_2.hashed_id());
}
