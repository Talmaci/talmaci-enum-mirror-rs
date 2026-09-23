use talmaci_macros::EnumMirror;

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
}

#[test]
fn converts_target_to_source() {
    let source: Source = Target::Disabled.into();

    assert_eq!(source, Source::Disabled);
}