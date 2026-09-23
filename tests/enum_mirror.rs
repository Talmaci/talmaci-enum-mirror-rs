use talmaci_enum_mirror::EnumMirror;

#[derive(Debug, PartialEq, Eq, EnumMirror)]
#[enum_mirror(Target)]
enum Source {
    Active,
    Disabled,
}

#[derive(Debug, PartialEq, Eq)]
enum Target {
    Active,
    Disabled,
}

#[test]
fn converts_source_to_target() {
    let target: Target = Source::Active.into();

    assert_eq!(target, Target::Active);
    assert_eq!(Target::from(Source::Disabled), Target::Disabled);
}

#[test]
fn converts_target_to_source() {
    let source: Source = Target::Disabled.into();

    assert_eq!(source, Source::Disabled);
    assert_eq!(Source::from(Target::Active), Source::Active);
}

#[allow(non_camel_case_types)]
mod application {
    #[derive(Debug, PartialEq)]
    pub enum Status {
        Disabled = 10,
        r#type = 20,
        Active = 30,
    }
}

#[allow(non_camel_case_types)]
mod persistence {
    use talmaci_enum_mirror::EnumMirror;

    #[derive(Debug, PartialEq, EnumMirror)]
    #[enum_mirror(crate::application::Status)]
    pub enum PgStatus {
        Active = 1,
        r#type = 2,
        #[cfg(any())]
        Excluded,
        Disabled = 3,
    }
}

#[test]
fn maps_names_across_modules_regardless_of_order_or_discriminants() {
    use application::Status;
    use persistence::PgStatus;

    for (source, expected) in [
        (PgStatus::Active, Status::Active),
        (PgStatus::Disabled, Status::Disabled),
        (PgStatus::r#type, Status::r#type),
    ] {
        let target = Status::from(source);
        assert_eq!(target, expected);
        assert_eq!(Status::from(PgStatus::from(target)), expected);
    }
}
