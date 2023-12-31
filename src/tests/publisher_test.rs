use crate::publisher::Publisher;

#[test]
fn get() {
    let publisher = Publisher::new("/publisher/".to_owned());
    assert_eq!(publisher.get_as_ref_string(), &"publisher");

    let publisher = Publisher::new("/publisher".to_owned());
    assert_eq!(publisher.get_as_ref_string(), &"publisher");

    let publisher = Publisher::new("publisher/".to_owned());
    assert_eq!(publisher.get_as_ref_string(), &"publisher");

    let publisher = Publisher::new("//publisher//".to_owned());
    assert_eq!(publisher.get_as_ref_string(), &"publisher")
}

#[test]
#[should_panic]
fn get_panic() {
    Publisher::new("publ@sher".to_owned());
    Publisher::new("pu:lisher".to_owned());
    Publisher::new("publis-er".to_owned());
    Publisher::new(",ublisher".to_owned());
}

#[test]
fn parse_publisher_from_raw_fmri() {
    let publisher = Publisher::parse_publisher_from_raw_fmri("fmri=pkg://publisher/test/test@1-0.1".to_owned()).unwrap();
    assert_eq!(publisher, Publisher::new("publisher".to_owned()));

    let publisher = Publisher::parse_publisher_from_raw_fmri("pkg:/test/test@1-0.1".to_owned());
    assert_eq!(publisher, None);
}

#[test]
#[should_panic]
fn parse_publisher_from_raw_fmri_panic() {
    Publisher::parse_publisher_from_raw_fmri("fmri=publisher/pkg://test/test@1-0.1".to_owned()).unwrap();
    Publisher::parse_publisher_from_raw_fmri("pkg://publisher".to_owned());
}