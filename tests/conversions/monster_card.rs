use duelingbook_extractor as dbxt;


#[test] fn kind()
{
    assert_eq!( dbxt::Kind::try_from("Normal" .to_string()).ok(), Some(dbxt::Kind::NORMAL)  );
    assert_eq!( dbxt::Kind::try_from("Effect" .to_string()).ok(), Some(dbxt::Kind::EFFECT)  );
    assert_eq!( dbxt::Kind::try_from("Ritual" .to_string()).ok(), Some(dbxt::Kind::RITUAL)  );
    assert_eq!( dbxt::Kind::try_from("Fusion" .to_string()).ok(), Some(dbxt::Kind::FUSION)  );
    assert_eq!( dbxt::Kind::try_from("Synchro".to_string()).ok(), Some(dbxt::Kind::SYNCHRO) );
    assert_eq!( dbxt::Kind::try_from("Xyz"    .to_string()).ok(), Some(dbxt::Kind::XYZ)     );
    assert_eq!( dbxt::Kind::try_from("Link"   .to_string()).ok(), Some(dbxt::Kind::LINK)    );
}
