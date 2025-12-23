use duelingbook_extractor as db;


#[test] fn monster_kind()
{
    assert_eq!( db::Kind::try_from("Normal" .to_string()).ok(), Some(db::Kind::NORMAL)  );
    assert_eq!( db::Kind::try_from("Effect" .to_string()).ok(), Some(db::Kind::EFFECT)  );
    assert_eq!( db::Kind::try_from("Ritual" .to_string()).ok(), Some(db::Kind::RITUAL)  );
    assert_eq!( db::Kind::try_from("Fusion" .to_string()).ok(), Some(db::Kind::FUSION)  );
    assert_eq!( db::Kind::try_from("Synchro".to_string()).ok(), Some(db::Kind::SYNCHRO) );
    assert_eq!( db::Kind::try_from("Xyz"    .to_string()).ok(), Some(db::Kind::XYZ)     );
    assert_eq!( db::Kind::try_from("Link"   .to_string()).ok(), Some(db::Kind::LINK)    );
}
