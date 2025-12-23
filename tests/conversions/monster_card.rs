use duelingbook_extractor as db;


#[test] fn monster_kind()
{
    assert_eq!( db::Kind::try_from("Normal" .to_string()), Ok(db::Kind::NORMAL)  );
    assert_eq!( db::Kind::try_from("Effect" .to_string()), Ok(db::Kind::EFFECT)  );
    assert_eq!( db::Kind::try_from("Ritual" .to_string()), Ok(db::Kind::RITUAL)  );
    assert_eq!( db::Kind::try_from("Fusion" .to_string()), Ok(db::Kind::FUSION)  );
    assert_eq!( db::Kind::try_from("Synchro".to_string()), Ok(db::Kind::SYNCHRO) );
    assert_eq!( db::Kind::try_from("Xyz"    .to_string()), Ok(db::Kind::XYZ)     );
    assert_eq!( db::Kind::try_from("Link"   .to_string()), Ok(db::Kind::LINK)    );
}
