use std::path::Path;

use aupfile::{AviUtlProject, FilterProject};

const AUP_FIXTURES: &[&str] = &[
    "testdata/EditHandle/640x480_2997-100fps_44100Hz.aup",
    "testdata/FilterProject/VariousFilters.aup",
    "testdata/Exedit/EffectSet01.aup",
    "testdata/Exedit/ExeditBpm.aup",
    "testdata/Exedit/ExeditCamera.aup",
    "testdata/Exedit/ExeditXY.aup",
    "testdata/Exedit/LayerScene.aup",
    "testdata/Exedit/Trackbar.aup",
    "testdata/Exedit/Chain.aup",
    "testdata/Exedit/Group.aup",
    "testdata/Exedit/DisabledEffect.aup",
];

const EXO_FIXTURES: &[(&str, u32, &str)] = &[
    (
        "testdata/Exedit/EffectSet01.aup",
        0,
        "testdata/Exedit/EffectSet01_0.exo",
    ),
    (
        "testdata/Exedit/LayerScene.aup",
        0,
        "testdata/Exedit/LayerScene_0.exo",
    ),
    (
        "testdata/Exedit/LayerScene.aup",
        1,
        "testdata/Exedit/LayerScene_1.exo",
    ),
    (
        "testdata/Exedit/LayerScene.aup",
        2,
        "testdata/Exedit/LayerScene_2.exo",
    ),
    (
        "testdata/Exedit/Trackbar.aup",
        0,
        "testdata/Exedit/Trackbar_0.exo",
    ),
    (
        "testdata/Exedit/Chain.aup",
        0,
        "testdata/Exedit/Chain_0.exo",
    ),
    (
        "testdata/Exedit/Chain.aup",
        1,
        "testdata/Exedit/Chain_1.exo",
    ),
    (
        "testdata/Exedit/Group.aup",
        0,
        "testdata/Exedit/Group_0.exo",
    ),
    (
        "testdata/Exedit/Group.aup",
        1,
        "testdata/Exedit/Group_1.exo",
    ),
    (
        "testdata/Exedit/DisabledEffect.aup",
        0,
        "testdata/Exedit/DisabledEffect_0.exo",
    ),
];

const FRAME_FIXTURES: &[(&str, &str)] = &[
    (
        "testdata/EditHandle/640x480_2997-100fps_44100Hz.aup",
        "testdata/EditHandle/640x480_2997-100fps_44100Hz_FrameData.csv",
    ),
    (
        "testdata/FilterProject/VariousFilters.aup",
        "testdata/FilterProject/VariousFilters_FrameData.csv",
    ),
    (
        "testdata/Exedit/EffectSet01.aup",
        "testdata/Exedit/EffectSet01_FrameData.csv",
    ),
    (
        "testdata/Exedit/LayerScene.aup",
        "testdata/Exedit/LayerScene_FrameData.csv",
    ),
    (
        "testdata/Exedit/Trackbar.aup",
        "testdata/Exedit/Trackbar_FrameData.csv",
    ),
    (
        "testdata/Exedit/Chain.aup",
        "testdata/Exedit/Chain_FrameData.csv",
    ),
    (
        "testdata/Exedit/Group.aup",
        "testdata/Exedit/Group_FrameData.csv",
    ),
];

#[test]
fn aup_write_is_stable_after_reading_its_output() {
    for fixture in AUP_FIXTURES {
        let project = AviUtlProject::open(fixture).unwrap_or_else(|error| {
            panic!("failed to read {fixture}: {error}");
        });
        let mut first = Vec::new();
        project.write(&mut first).unwrap();
        let reparsed = AviUtlProject::read(first.as_slice()).unwrap();
        let mut second = Vec::new();
        reparsed.write(&mut second).unwrap();
        assert_eq!(first, second, "{fixture}");
    }
}

#[test]
fn exedit_data_round_trips() {
    for fixture in AUP_FIXTURES
        .iter()
        .filter(|path| !path.contains("EditHandle"))
    {
        let mut project = AviUtlProject::open(fixture).unwrap();
        let raw = project
            .filter_projects
            .iter()
            .find_map(|filter| match filter {
                FilterProject::Raw(raw) if raw.name == "拡張編集" => Some(raw.data.clone()),
                _ => None,
            })
            .unwrap_or_else(|| panic!("no ExEdit data in {fixture}"));
        let exedit = project
            .decode_exedit()
            .unwrap()
            .unwrap_or_else(|| panic!("no ExEdit data in {fixture}"));
        let actual = exedit.to_bytes().unwrap();
        if raw != actual {
            let offset = raw
                .iter()
                .zip(&actual)
                .position(|(expected, actual)| expected != actual)
                .unwrap_or(raw.len().min(actual.len()));
            panic!(
                "{fixture}: first difference at {offset:#x}, expected {:?}, actual {:?}, lengths {} and {}",
                raw.get(offset),
                actual.get(offset),
                raw.len(),
                actual.len()
            );
        }
    }
}

#[test]
fn fixtures_exist_relative_to_the_crate() {
    for fixture in AUP_FIXTURES {
        assert!(Path::new(fixture).is_file(), "missing {fixture}");
    }
}

#[test]
fn exo_exports_match_aup_dot_net() {
    for (aup_path, scene, exo_path) in EXO_FIXTURES {
        let mut project = AviUtlProject::open(aup_path).unwrap();
        let edit_handle = project.edit_handle.clone();
        let exedit = project.decode_exedit().unwrap().unwrap();
        if let Some(script) = exedit
            .trackbar_scripts
            .iter_mut()
            .find(|script| script.name == "トラックバーサンプル")
        {
            script.enable_param = true;
            script.enable_speed = false;
        }
        let exo = exedit.export_object(*scene, &edit_handle).unwrap();
        let mut actual = Vec::new();
        exo.write(&mut actual).unwrap();
        let expected = std::fs::read(exo_path).unwrap();
        assert_eq!(expected, actual, "{aup_path}, scene {scene}");
    }
}

#[test]
fn frame_data_matches_aup_dot_net_snapshots() {
    for (aup_path, csv_path) in FRAME_FIXTURES {
        let project = AviUtlProject::open(aup_path).unwrap();
        let csv = std::fs::read_to_string(csv_path).unwrap();
        let rows = csv.lines().collect::<Vec<_>>();
        assert_eq!(project.edit_handle.frames.len(), rows.len(), "{aup_path}");
        for (index, (frame, row)) in project.edit_handle.frames.iter().zip(rows).enumerate() {
            let values = row
                .split(',')
                .map(|value| value.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            assert_eq!(values.len(), 10, "{csv_path}, row {index}");
            let actual = [
                frame.video,
                frame.audio,
                frame.field2,
                frame.field3,
                u32::from(frame.inter.0),
                u32::from(frame.index_24fps),
                u32::from(frame.edit_flag.bits()),
                u32::from(frame.config),
                u32::from(frame.vcm),
                u32::from(frame.clip),
            ];
            assert_eq!(actual.as_slice(), values, "{csv_path}, row {index}");
        }
    }
}

#[test]
fn raw_filter_projects_match_aup_dot_net_dumps() {
    const FILTERS: &[&str] = &[
        "拡張編集",
        "ごちゃまぜドロップス",
        "PSDToolKit",
        "拡張編集RAMプレビュー",
        "グラフエディタ",
    ];
    let project = AviUtlProject::open("testdata/FilterProject/VariousFilters.aup").unwrap();
    assert_eq!(project.filter_projects.len(), FILTERS.len());
    for (index, (filter, expected_name)) in project.filter_projects.iter().zip(FILTERS).enumerate()
    {
        assert_eq!(filter.name(), *expected_name);
        let path =
            format!("testdata/FilterProject/VariousFilters_Filter_{index}_{expected_name}.dat");
        assert_eq!(filter.dump_data().unwrap(), std::fs::read(path).unwrap());
    }
}
