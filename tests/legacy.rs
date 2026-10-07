use aupfile::exedit::{ExEditProject, LayerFlag, TrackbarFlag, TrackbarType};
use aupfile::{AviUtlProject, FilterProject, RawFilterProject};

#[test]
fn legacy_layouts_decode_export_and_round_trip() {
    for (version, data) in [
        (
            8000,
            include_bytes!("../testdata/Legacy/Exedit8000.dat").as_slice(),
        ),
        (
            9000,
            include_bytes!("../testdata/Legacy/Exedit9000.dat").as_slice(),
        ),
        (
            9015,
            include_bytes!("../testdata/Legacy/Exedit9015.dat").as_slice(),
        ),
        (
            9100,
            include_bytes!("../testdata/Legacy/Exedit9100.dat").as_slice(),
        ),
    ] {
        let mut project = AviUtlProject::default();
        project
            .filter_projects
            .push(FilterProject::Raw(RawFilterProject {
                name: ExEditProject::FILTER_NAME.to_owned(),
                data: data.to_vec(),
            }));
        let mut aup = Vec::new();
        project.write(&mut aup).unwrap();
        let mut project = AviUtlProject::read(aup.as_slice()).unwrap();
        let edit_handle = project.edit_handle.clone();
        let exedit = project.decode_exedit().unwrap().unwrap();
        assert_eq!(exedit.version, version);
        assert_eq!(exedit.layers.len(), 2);
        assert_eq!(exedit.layers[0].name, "Legacy hidden");
        assert_eq!(exedit.layers[0].flag, LayerFlag::HIDE);
        assert_eq!(exedit.layers[1].layer_index, 1);
        assert_eq!(exedit.layers[1].flag, LayerFlag::LOCK);
        assert_eq!(exedit.objects.len(), 2);
        let object = &exedit.objects[0];
        assert_eq!(object.layer_index, 1);
        assert_eq!(object.scene_index, 0);
        assert_eq!(object.group, 9);
        assert_eq!(object.unknown_0x4b8, 0x12345678);
        assert_eq!(object.effects.len(), 2);
        let media = &object.effects[0];
        assert_eq!(media.ext_data, [1, 2, 0xfe, 0xff]);
        assert_eq!(media.checkboxes, [1]);
        assert_eq!(media.trackbars[0].current, -123);
        assert_eq!(media.trackbars[0].next, 321);
        assert_eq!(media.trackbars[0].transition_type, TrackbarType::LINEAR);
        assert_eq!(media.trackbars[0].flag, TrackbarFlag::ACCELERATION);
        assert_eq!(media.trackbars[1].flag, TrackbarFlag::DECELERATION);
        assert_eq!(
            media.trackbars[0].parameter,
            if version == 8000 { 0 } else { 17 }
        );
        let draw = &object.effects[1];
        assert_eq!(draw.trackbars[0].current, 789);
        assert_eq!(draw.trackbars[0].script_index, i32::from(version == 9100));
        assert_eq!(draw.checkboxes, [-2]);
        assert!(exedit.objects[1].chain);
        assert!(exedit.objects[1].effects[0].ext_data.is_empty());
        let exo = exedit
            .export_object(0, &edit_handle)
            .unwrap()
            .to_text()
            .unwrap();
        assert!(exo.contains("[1]\r\nstart=11\r\nend=20\r\nlayer=2"));
        assert!(exo.contains("chain=1\r\n"));

        let normalized = exedit.to_bytes().unwrap();
        let reparsed = ExEditProject::from_bytes(&normalized).unwrap();
        assert_eq!(normalized, reparsed.to_bytes().unwrap());
        let mut aup = Vec::new();
        project.write(&mut aup).unwrap();
        let mut reparsed = AviUtlProject::read(aup.as_slice()).unwrap();
        assert_eq!(
            reparsed
                .decode_exedit()
                .unwrap()
                .unwrap()
                .to_bytes()
                .unwrap(),
            normalized
        );

        assert!(ExEditProject::from_bytes(&data[..data.len() - 1]).is_err());
        let mut invalid = data.to_vec();
        invalid[0x20..0x24].copy_from_slice(&65u32.to_le_bytes());
        assert!(ExEditProject::from_bytes(&invalid).is_err());
    }
}

#[test]
fn truncated_preview_preserves_original_bytes_and_remains_editable() {
    use aupfile::exedit::TimelineObject;

    let mut object = TimelineObject::default();
    object.raw_base[0x10..0x4e].fill(b'a');
    object.raw_base[0x4e] = 0x82;
    object.preview = "a".repeat(62);
    let mut project = ExEditProject::default();
    project.objects.push(object);
    let original = project.to_bytes().unwrap();
    let mut parsed = ExEditProject::from_bytes(&original).unwrap();
    assert_eq!(parsed.objects[0].preview, "a".repeat(62));
    assert_eq!(parsed.to_bytes().unwrap(), original);
    parsed.objects[0].preview = "変更".to_owned();
    let changed = ExEditProject::from_bytes(&parsed.to_bytes().unwrap()).unwrap();
    assert_eq!(changed.objects[0].preview, "変更");

    for preview in [b"\x82\0".as_slice(), b"\x82\x20\0".as_slice()] {
        let mut invalid = original.clone();
        let offset =
            0x100 + project.trackbar_scripts.len() * 128 + project.effect_types.len() * 112 + 0x10;
        invalid[offset..offset + preview.len()].copy_from_slice(preview);
        assert!(ExEditProject::from_bytes(&invalid).is_err());
    }
}
