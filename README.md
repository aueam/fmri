# FMRI

Implementation of IPS package identifier - FMRI. Provides FMRI, Publisher and Version types

## Example

```rust
use std::cmp::Ordering;
use fmri::{
    FMRI,
    compare::Compare,
    publisher::Publisher,
};

fn main() {
    let raw_fmri = &"pkg://publisher/system/library@0.5.11,5.11-0.175.1.0.0.2.1".to_owned();

    // create fmri
    let mut fmri = FMRI::parse_raw(raw_fmri);

    // create new publisher
    let publisher = Publisher::new("test".to_owned());
    fmri.change_publisher(publisher);

    // remove version
    fmri.remove_version();

    assert_eq!(format!("{}", fmri), "pkg://test/system/library");

    // get package name from fmri
    assert_eq!(fmri.get_package_name_as_string(), "system/library");

    // compare two fmris
    let fmri_a = FMRI::parse_raw(&"test@1".to_owned());
    let fmri_b = FMRI::parse_raw(&"test@2".to_owned());

    // fmri_a is older than fmri_b
    assert_eq!(fmri_a.compare(&fmri_b), Ordering::Less)
}
```