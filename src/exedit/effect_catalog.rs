// This table is derived from AupDotNet v0.2.0's EffectType.Defaults.
effect_catalog! {
    VideoFile => (0, 0x04000448, 2, 3, 284, "動画ファイル",
    [
      Some(TrackbarDefinition { name: "再生位置".to_owned(), scale: 1, min: 0, max: 0, default: 1 }),
      Some(TrackbarDefinition { name: "再生速度".to_owned(), scale: 10, min: -20000, max: 20000, default: 1000 }),
    ], [
      Some(CheckboxDefinition { name: "ループ再生".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "アルファチャンネルを読み込む".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "参照ファイル".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    ImageFile => (1, 0x04000408, 0, 1, 260, "画像ファイル",
    [
    ], [
      Some(CheckboxDefinition { name: "参照ファイル".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    AudioFile => (2, 0x04200408, 2, 3, 280, "音声ファイル",
    [
      Some(TrackbarDefinition { name: "再生位置".to_owned(), scale: 100, min: 0, max: 0, default: 0 }),
      Some(TrackbarDefinition { name: "再生速度".to_owned(), scale: 10, min: 100, max: 8000, default: 1000 }),
    ], [
      Some(CheckboxDefinition { name: "ループ再生".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "動画ファイルと連携".to_owned(), is_checkbox: true, default: 1 }),
      Some(CheckboxDefinition { name: "参照ファイル".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    Text => (3, 0x04000408, 2, 5, 2096, "テキスト",
    [
      Some(TrackbarDefinition { name: "サイズ".to_owned(), scale: 1, min: 1, max: 1000, default: 34 }),
      Some(TrackbarDefinition { name: "表示速度".to_owned(), scale: 10, min: 0, max: 8000, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "文字毎に個別オブジェクト".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "移動座標上に表示する".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "自動スクロール".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "B".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "I".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    Figure => (4, 0x04000408, 3, 2, 264, "図形",
    [
      Some(TrackbarDefinition { name: "サイズ".to_owned(), scale: 1, min: 0, max: 4000, default: 100 }),
      Some(TrackbarDefinition { name: "縦横比".to_owned(), scale: 10, min: -1000, max: 1000, default: 0 }),
      Some(TrackbarDefinition { name: "ライン幅".to_owned(), scale: 1, min: 0, max: 4000, default: 4000 }),
    ], [
      Some(CheckboxDefinition { name: "背景".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "色の設定".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    FrameBuffer => (5, 0x04000008, 0, 1, 0, "フレームバッファ",
    [
    ], [
      Some(CheckboxDefinition { name: "フレームバッファをクリア".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    Waveform => (6, 0x04000408, 4, 5, 304, "音声波形表示",
    [
      Some(TrackbarDefinition { name: "横幅".to_owned(), scale: 1, min: 0, max: 2000, default: 640 }),
      Some(TrackbarDefinition { name: "高さ".to_owned(), scale: 1, min: 0, max: 2000, default: 240 }),
      Some(TrackbarDefinition { name: "音量".to_owned(), scale: 10, min: 0, max: 5000, default: 1000 }),
      Some(TrackbarDefinition { name: "再生位置".to_owned(), scale: 100, min: 0, max: 0, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "Type1".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "波形の色".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "参照ファイル".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "編集全体の音声を元にする".to_owned(), is_checkbox: true, default: 1 }),
      Some(CheckboxDefinition { name: "設定".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    Scene => (7, 0x04000408, 2, 2, 4, "シーン",
    [
      Some(TrackbarDefinition { name: "再生位置".to_owned(), scale: 1, min: 1, max: 1, default: 1 }),
      Some(TrackbarDefinition { name: "再生速度".to_owned(), scale: 10, min: -20000, max: 20000, default: 1000 }),
    ], [
      Some(CheckboxDefinition { name: "ループ再生".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "シーン選択".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    SceneAudio => (8, 0x04200408, 2, 3, 4, "シーン(音声)",
    [
      Some(TrackbarDefinition { name: "再生位置".to_owned(), scale: 1, min: 1, max: 1, default: 1 }),
      Some(TrackbarDefinition { name: "再生速度".to_owned(), scale: 10, min: 100, max: 20000, default: 1000 }),
    ], [
      Some(CheckboxDefinition { name: "ループ再生".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "シーンと連携".to_owned(), is_checkbox: true, default: 1 }),
      Some(CheckboxDefinition { name: "シーン選択".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    PreviousObject => (9, 0x04000008, 0, 0, 0, "直前オブジェクト",
    [
    ], [
    ]),
    StandardDraw => (10, 0x440004D0, 6, 1, 4, "標準描画",
    [
      Some(TrackbarDefinition { name: "X".to_owned(), scale: 10, min: -999999, max: 999999, default: 0 }),
      Some(TrackbarDefinition { name: "Y".to_owned(), scale: 10, min: -999999, max: 999999, default: 0 }),
      Some(TrackbarDefinition { name: "Z".to_owned(), scale: 10, min: -999999, max: 999999, default: 0 }),
      Some(TrackbarDefinition { name: "拡大率".to_owned(), scale: 100, min: 0, max: 500000, default: 10000 }),
      Some(TrackbarDefinition { name: "透明度".to_owned(), scale: 10, min: 0, max: 1000, default: 0 }),
      Some(TrackbarDefinition { name: "回転".to_owned(), scale: 100, min: -360000, max: 360000, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "通常".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    ExtendedDraw => (11, 0x440004D0, 12, 2, 4, "拡張描画",
    [
      Some(TrackbarDefinition { name: "X".to_owned(), scale: 10, min: -999999, max: 999999, default: 0 }),
      Some(TrackbarDefinition { name: "Y".to_owned(), scale: 10, min: -999999, max: 999999, default: 0 }),
      Some(TrackbarDefinition { name: "Z".to_owned(), scale: 10, min: -999999, max: 999999, default: 0 }),
      Some(TrackbarDefinition { name: "拡大率".to_owned(), scale: 100, min: 0, max: 500000, default: 10000 }),
      Some(TrackbarDefinition { name: "透明度".to_owned(), scale: 10, min: 0, max: 1000, default: 0 }),
      Some(TrackbarDefinition { name: "縦横比".to_owned(), scale: 10, min: -1000, max: 1000, default: 0 }),
      Some(TrackbarDefinition { name: "X軸回転".to_owned(), scale: 100, min: -360000, max: 360000, default: 0 }),
      Some(TrackbarDefinition { name: "Y軸回転".to_owned(), scale: 100, min: -360000, max: 360000, default: 0 }),
      Some(TrackbarDefinition { name: "Z軸回転".to_owned(), scale: 100, min: -360000, max: 360000, default: 0 }),
      Some(TrackbarDefinition { name: "中心X".to_owned(), scale: 10, min: -20000, max: 20000, default: 0 }),
      Some(TrackbarDefinition { name: "中心Y".to_owned(), scale: 10, min: -20000, max: 20000, default: 0 }),
      Some(TrackbarDefinition { name: "中心Z".to_owned(), scale: 10, min: -20000, max: 20000, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "通常".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "裏面を表示しない".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    StandardPlayback => (12, 0x04200090, 2, 0, 0, "標準再生",
    [
      Some(TrackbarDefinition { name: "音量".to_owned(), scale: 10, min: 0, max: 5000, default: 1000 }),
      Some(TrackbarDefinition { name: "左右".to_owned(), scale: 10, min: -1000, max: 1000, default: 0 }),
    ], [
    ]),
    Particle => (13, 0x44000450, 16, 5, 4, "パーティクル出力",
    [
      Some(TrackbarDefinition { name: "X".to_owned(), scale: 10, min: -999999, max: 999999, default: 0 }),
      Some(TrackbarDefinition { name: "Y".to_owned(), scale: 10, min: -999999, max: 999999, default: 0 }),
      Some(TrackbarDefinition { name: "Z".to_owned(), scale: 10, min: -999999, max: 999999, default: 0 }),
      Some(TrackbarDefinition { name: "出力頻度".to_owned(), scale: 10, min: 0, max: 5000, default: 200 }),
      Some(TrackbarDefinition { name: "出力速度".to_owned(), scale: 10, min: 0, max: 200000, default: 4000 }),
      Some(TrackbarDefinition { name: "加速度".to_owned(), scale: 10, min: -200000, max: 200000, default: 0 }),
      Some(TrackbarDefinition { name: "出力方向".to_owned(), scale: 10, min: -36000, max: 36000, default: 0 }),
      Some(TrackbarDefinition { name: "拡散角度".to_owned(), scale: 10, min: 0, max: 3600, default: 300 }),
      Some(TrackbarDefinition { name: "透過率".to_owned(), scale: 10, min: 0, max: 1000, default: 0 }),
      Some(TrackbarDefinition { name: "透過速度".to_owned(), scale: 10, min: -2000, max: 2000, default: 0 }),
      Some(TrackbarDefinition { name: "拡大率".to_owned(), scale: 100, min: 0, max: 80000, default: 10000 }),
      Some(TrackbarDefinition { name: "拡大速度".to_owned(), scale: 100, min: -80000, max: 80000, default: 0 }),
      Some(TrackbarDefinition { name: "回転角".to_owned(), scale: 100, min: -360000, max: 360000, default: 0 }),
      Some(TrackbarDefinition { name: "回転速度".to_owned(), scale: 100, min: -1000, max: 1000, default: 0 }),
      Some(TrackbarDefinition { name: "重力".to_owned(), scale: 10, min: -20000, max: 20000, default: 0 }),
      Some(TrackbarDefinition { name: "生存時間".to_owned(), scale: 10, min: 0, max: 6000, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "出力方向の基準を移動方向にする".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "移動範囲の座標からランダムに出力".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "3Dランダム回転".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "通常".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "終了点で全て消えるように調節する".to_owned(), is_checkbox: true, default: 1 }),
    ]),
    SceneChange => (14, 0x04000400, 2, 3, 516, "シーンチェンジ",
    [
      Some(TrackbarDefinition { name: "調整".to_owned(), scale: 100, min: 0, max: 0, default: 0 }),
      Some(TrackbarDefinition { name: "track1".to_owned(), scale: 100, min: 0, max: 0, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "反転".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "check0".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "クロスフェード".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    ColorCorrection => (15, 0x04000020, 5, 1, 0, "色調補正",
    [
      Some(TrackbarDefinition { name: "明るさ".to_owned(), scale: 10, min: 0, max: 2000, default: 1000 }),
      Some(TrackbarDefinition { name: "ｺﾝﾄﾗｽﾄ".to_owned(), scale: 10, min: 0, max: 2000, default: 1000 }),
      Some(TrackbarDefinition { name: "色相".to_owned(), scale: 10, min: -36000, max: 36000, default: 0 }),
      Some(TrackbarDefinition { name: "輝度".to_owned(), scale: 10, min: 0, max: 2000, default: 1000 }),
      Some(TrackbarDefinition { name: "彩度".to_owned(), scale: 10, min: 0, max: 2000, default: 1000 }),
    ], [
      Some(CheckboxDefinition { name: "飽和する".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    ColorCorrectionFilter => (16, 0x04000000, 5, 1, 0, "色調補正",
    [
      Some(TrackbarDefinition { name: "明るさ".to_owned(), scale: 10, min: 0, max: 2000, default: 1000 }),
      Some(TrackbarDefinition { name: "ｺﾝﾄﾗｽﾄ".to_owned(), scale: 10, min: 0, max: 2000, default: 1000 }),
      Some(TrackbarDefinition { name: "色相".to_owned(), scale: 10, min: -36000, max: 36000, default: 0 }),
      Some(TrackbarDefinition { name: "輝度".to_owned(), scale: 10, min: 0, max: 2000, default: 1000 }),
      Some(TrackbarDefinition { name: "彩度".to_owned(), scale: 10, min: 0, max: 2000, default: 1000 }),
    ], [
      Some(CheckboxDefinition { name: "飽和する".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    Clipping => (17, 0x04000020, 4, 1, 0, "クリッピング",
    [
      Some(TrackbarDefinition { name: "上".to_owned(), scale: 1, min: 0, max: 4000, default: 0 }),
      Some(TrackbarDefinition { name: "下".to_owned(), scale: 1, min: 0, max: 4000, default: 0 }),
      Some(TrackbarDefinition { name: "左".to_owned(), scale: 1, min: 0, max: 4000, default: 0 }),
      Some(TrackbarDefinition { name: "右".to_owned(), scale: 1, min: 0, max: 4000, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "中心の位置を変更".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    Blur => (18, 0x04000020, 3, 1, 0, "ぼかし",
    [
      Some(TrackbarDefinition { name: "範囲".to_owned(), scale: 1, min: 0, max: 1000, default: 5 }),
      Some(TrackbarDefinition { name: "縦横比".to_owned(), scale: 10, min: -1000, max: 1000, default: 0 }),
      Some(TrackbarDefinition { name: "光の強さ".to_owned(), scale: 1, min: 0, max: 60, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "サイズ固定".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    BorderBlur => (19, 0x04000020, 2, 1, 0, "境界ぼかし",
    [
      Some(TrackbarDefinition { name: "範囲".to_owned(), scale: 1, min: 0, max: 2000, default: 5 }),
      Some(TrackbarDefinition { name: "縦横比".to_owned(), scale: 10, min: -1000, max: 1000, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "透明度の境界をぼかす".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    BlurFilter => (20, 0x04000000, 3, 0, 0, "ぼかし",
    [
      Some(TrackbarDefinition { name: "範囲".to_owned(), scale: 1, min: 0, max: 1000, default: 5 }),
      Some(TrackbarDefinition { name: "縦横比".to_owned(), scale: 10, min: -1000, max: 1000, default: 0 }),
      Some(TrackbarDefinition { name: "光の強さ".to_owned(), scale: 1, min: 0, max: 60, default: 0 }),
    ], [
    ]),
    Mosaic => (21, 0x04000020, 1, 1, 0, "モザイク",
    [
      Some(TrackbarDefinition { name: "サイズ".to_owned(), scale: 1, min: 1, max: 2000, default: 12 }),
    ], [
      Some(CheckboxDefinition { name: "タイル風".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    MosaicFilter => (22, 0x04000000, 1, 1, 0, "モザイク",
    [
      Some(TrackbarDefinition { name: "サイズ".to_owned(), scale: 1, min: 1, max: 2000, default: 12 }),
    ], [
      Some(CheckboxDefinition { name: "タイル風".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    Emission => (23, 0x04000420, 4, 2, 4, "発光",
    [
      Some(TrackbarDefinition { name: "強さ".to_owned(), scale: 10, min: 0, max: 2000, default: 1000 }),
      Some(TrackbarDefinition { name: "拡散".to_owned(), scale: 1, min: 10, max: 2000, default: 250 }),
      Some(TrackbarDefinition { name: "しきい値".to_owned(), scale: 10, min: 0, max: 2000, default: 800 }),
      Some(TrackbarDefinition { name: "拡散速度".to_owned(), scale: 1, min: 0, max: 60, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "光色の設定".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "サイズ固定".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    EmissionFilter => (24, 0x04000400, 4, 1, 4, "発光",
    [
      Some(TrackbarDefinition { name: "強さ".to_owned(), scale: 10, min: 0, max: 2000, default: 1000 }),
      Some(TrackbarDefinition { name: "拡散".to_owned(), scale: 1, min: 10, max: 2000, default: 250 }),
      Some(TrackbarDefinition { name: "しきい値".to_owned(), scale: 10, min: 0, max: 2000, default: 800 }),
      Some(TrackbarDefinition { name: "拡散速度".to_owned(), scale: 1, min: 0, max: 60, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "光色の設定".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    Flash => (25, 0x04000420, 3, 3, 8, "閃光",
    [
      Some(TrackbarDefinition { name: "強さ".to_owned(), scale: 10, min: 0, max: 1000, default: 1000 }),
      Some(TrackbarDefinition { name: "X".to_owned(), scale: 1, min: -2000, max: 2000, default: 0 }),
      Some(TrackbarDefinition { name: "Y".to_owned(), scale: 1, min: -2000, max: 2000, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "前方に合成".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "光色の設定".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "サイズ固定".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    Diffuse => (26, 0x04000020, 2, 1, 0, "拡散光",
    [
      Some(TrackbarDefinition { name: "強さ".to_owned(), scale: 10, min: 0, max: 1000, default: 500 }),
      Some(TrackbarDefinition { name: "拡散".to_owned(), scale: 1, min: 0, max: 500, default: 12 }),
    ], [
      Some(CheckboxDefinition { name: "サイズ固定".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    DiffuseFilter => (27, 0x04000000, 2, 0, 0, "拡散光",
    [
      Some(TrackbarDefinition { name: "強さ".to_owned(), scale: 10, min: 0, max: 1000, default: 500 }),
      Some(TrackbarDefinition { name: "拡散".to_owned(), scale: 1, min: 0, max: 500, default: 12 }),
    ], [
    ]),
    Glow => (28, 0x04000420, 4, 3, 8, "グロー",
    [
      Some(TrackbarDefinition { name: "強さ".to_owned(), scale: 10, min: 0, max: 4000, default: 400 }),
      Some(TrackbarDefinition { name: "拡散".to_owned(), scale: 1, min: 0, max: 200, default: 30 }),
      Some(TrackbarDefinition { name: "しきい値".to_owned(), scale: 10, min: 0, max: 2000, default: 400 }),
      Some(TrackbarDefinition { name: "ぼかし".to_owned(), scale: 1, min: 0, max: 50, default: 1 }),
    ], [
      Some(CheckboxDefinition { name: "通常".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "光色の設定".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "光成分のみ".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    GlowFilter => (29, 0x04000400, 4, 2, 8, "グロー",
    [
      Some(TrackbarDefinition { name: "強さ".to_owned(), scale: 10, min: 0, max: 4000, default: 400 }),
      Some(TrackbarDefinition { name: "拡散".to_owned(), scale: 1, min: 0, max: 200, default: 30 }),
      Some(TrackbarDefinition { name: "しきい値".to_owned(), scale: 10, min: 0, max: 2000, default: 400 }),
      Some(TrackbarDefinition { name: "ぼかし".to_owned(), scale: 1, min: 0, max: 50, default: 1 }),
    ], [
      Some(CheckboxDefinition { name: "通常".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "光色の設定".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    ChromaKey => (30, 0x04000420, 3, 3, 12, "クロマキー",
    [
      Some(TrackbarDefinition { name: "色相範囲".to_owned(), scale: 1, min: 0, max: 256, default: 24 }),
      Some(TrackbarDefinition { name: "彩度範囲".to_owned(), scale: 1, min: 0, max: 256, default: 96 }),
      Some(TrackbarDefinition { name: "境界補正".to_owned(), scale: 1, min: 0, max: 5, default: 1 }),
    ], [
      Some(CheckboxDefinition { name: "色彩補正".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "透過補正".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "キー色の取得".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    ColorKey => (31, 0x04000420, 3, 1, 12, "カラーキー",
    [
      Some(TrackbarDefinition { name: "輝度範囲".to_owned(), scale: 1, min: 0, max: 4096, default: 0 }),
      Some(TrackbarDefinition { name: "色差範囲".to_owned(), scale: 1, min: 0, max: 4096, default: 0 }),
      Some(TrackbarDefinition { name: "境界補正".to_owned(), scale: 1, min: 0, max: 5, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "キー色の取得".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    LuminanceKey => (32, 0x04000420, 2, 1, 4, "ルミナンスキー",
    [
      Some(TrackbarDefinition { name: "基準輝度".to_owned(), scale: 1, min: 0, max: 8192, default: 2048 }),
      Some(TrackbarDefinition { name: "ぼかし".to_owned(), scale: 1, min: 0, max: 4096, default: 512 }),
    ], [
      Some(CheckboxDefinition { name: "暗い部分を透過".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    Light => (33, 0x04000420, 3, 2, 4, "ライト",
    [
      Some(TrackbarDefinition { name: "強さ".to_owned(), scale: 10, min: 0, max: 3000, default: 1000 }),
      Some(TrackbarDefinition { name: "拡散".to_owned(), scale: 1, min: 0, max: 500, default: 25 }),
      Some(TrackbarDefinition { name: "比率".to_owned(), scale: 10, min: -1000, max: 1000, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "逆光".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "色の設定".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    Shadow => (34, 0x44000420, 4, 3, 260, "シャドー",
    [
      Some(TrackbarDefinition { name: "X".to_owned(), scale: 1, min: -1000, max: 1000, default: -40 }),
      Some(TrackbarDefinition { name: "Y".to_owned(), scale: 1, min: -1000, max: 1000, default: 24 }),
      Some(TrackbarDefinition { name: "濃さ".to_owned(), scale: 10, min: 0, max: 1000, default: 400 }),
      Some(TrackbarDefinition { name: "拡散".to_owned(), scale: 1, min: 0, max: 500, default: 10 }),
    ], [
      Some(CheckboxDefinition { name: "影を別オブジェクトで描画".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "影色の設定".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "パターン画像ファイル".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    Border => (35, 0x44000420, 2, 2, 260, "縁取り",
    [
      Some(TrackbarDefinition { name: "サイズ".to_owned(), scale: 1, min: 0, max: 500, default: 3 }),
      Some(TrackbarDefinition { name: "ぼかし".to_owned(), scale: 1, min: 0, max: 100, default: 10 }),
    ], [
      Some(CheckboxDefinition { name: "縁色の設定".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "パターン画像ファイル".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    Bevel => (36, 0x04000020, 3, 0, 0, "凸エッジ",
    [
      Some(TrackbarDefinition { name: "幅".to_owned(), scale: 1, min: 0, max: 100, default: 4 }),
      Some(TrackbarDefinition { name: "高さ".to_owned(), scale: 100, min: 0, max: 300, default: 100 }),
      Some(TrackbarDefinition { name: "角度".to_owned(), scale: 10, min: -3600, max: 3600, default: -450 }),
    ], [
    ]),
    EdgeExtraction => (37, 0x04000420, 2, 3, 4, "エッジ抽出",
    [
      Some(TrackbarDefinition { name: "強さ".to_owned(), scale: 10, min: 0, max: 10000, default: 1000 }),
      Some(TrackbarDefinition { name: "しきい値".to_owned(), scale: 100, min: -10000, max: 10000, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "輝度エッジを抽出".to_owned(), is_checkbox: true, default: 1 }),
      Some(CheckboxDefinition { name: "透明度エッジを抽出".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "色の設定".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    Sharpen => (38, 0x04000020, 2, 0, 0, "シャープ",
    [
      Some(TrackbarDefinition { name: "強さ".to_owned(), scale: 10, min: 0, max: 8000, default: 500 }),
      Some(TrackbarDefinition { name: "範囲".to_owned(), scale: 1, min: 0, max: 100, default: 5 }),
    ], [
    ]),
    Fade => (39, 0x04000020, 2, 0, 0, "フェード",
    [
      Some(TrackbarDefinition { name: "イン".to_owned(), scale: 100, min: 0, max: 1000, default: 50 }),
      Some(TrackbarDefinition { name: "アウト".to_owned(), scale: 100, min: 0, max: 1000, default: 50 }),
    ], [
    ]),
    Wipe => (40, 0x04000420, 3, 3, 260, "ワイプ",
    [
      Some(TrackbarDefinition { name: "イン".to_owned(), scale: 100, min: 0, max: 1000, default: 50 }),
      Some(TrackbarDefinition { name: "アウト".to_owned(), scale: 100, min: 0, max: 1000, default: 50 }),
      Some(TrackbarDefinition { name: "ぼかし".to_owned(), scale: 1, min: 0, max: 100, default: 2 }),
    ], [
      Some(CheckboxDefinition { name: "反転(イン)".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "反転(アウト)".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "ワイプ(円)".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    Mask => (41, 0x04000620, 6, 3, 264, "マスク",
    [
      Some(TrackbarDefinition { name: "X".to_owned(), scale: 10, min: -40000, max: 40000, default: 0 }),
      Some(TrackbarDefinition { name: "Y".to_owned(), scale: 10, min: -40000, max: 40000, default: 0 }),
      Some(TrackbarDefinition { name: "回転".to_owned(), scale: 100, min: -360000, max: 360000, default: 0 }),
      Some(TrackbarDefinition { name: "サイズ".to_owned(), scale: 1, min: 0, max: 4000, default: 100 }),
      Some(TrackbarDefinition { name: "縦横比".to_owned(), scale: 10, min: -1000, max: 1000, default: 0 }),
      Some(TrackbarDefinition { name: "ぼかし".to_owned(), scale: 1, min: 0, max: 1000, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "背景".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "マスクの反転".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "元のサイズに合わせる".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    DiagonalClipping => (42, 0x04000020, 5, 0, 0, "斜めクリッピング",
    [
      Some(TrackbarDefinition { name: "中心X".to_owned(), scale: 1, min: -2000, max: 2000, default: 0 }),
      Some(TrackbarDefinition { name: "中心Y".to_owned(), scale: 1, min: -2000, max: 2000, default: 0 }),
      Some(TrackbarDefinition { name: "角度".to_owned(), scale: 10, min: -36000, max: 36000, default: 0 }),
      Some(TrackbarDefinition { name: "ぼかし".to_owned(), scale: 1, min: 0, max: 2000, default: 1 }),
      Some(TrackbarDefinition { name: "幅".to_owned(), scale: 1, min: -2000, max: 2000, default: 0 }),
    ], [
    ]),
    RadialBlur => (43, 0x04000020, 3, 1, 0, "放射ブラー",
    [
      Some(TrackbarDefinition { name: "範囲".to_owned(), scale: 10, min: 0, max: 750, default: 200 }),
      Some(TrackbarDefinition { name: "X".to_owned(), scale: 1, min: -2000, max: 2000, default: 0 }),
      Some(TrackbarDefinition { name: "Y".to_owned(), scale: 1, min: -2000, max: 2000, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "サイズ固定".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    RadialBlurFilter => (44, 0x44000000, 3, 0, 0, "放射ブラー",
    [
      Some(TrackbarDefinition { name: "範囲".to_owned(), scale: 10, min: 0, max: 1000, default: 200 }),
      Some(TrackbarDefinition { name: "X".to_owned(), scale: 1, min: -2000, max: 2000, default: 0 }),
      Some(TrackbarDefinition { name: "Y".to_owned(), scale: 1, min: -2000, max: 2000, default: 0 }),
    ], [
    ]),
    DirectionalBlur => (45, 0x04000020, 2, 1, 0, "方向ブラー",
    [
      Some(TrackbarDefinition { name: "範囲".to_owned(), scale: 1, min: 0, max: 500, default: 20 }),
      Some(TrackbarDefinition { name: "角度".to_owned(), scale: 10, min: -36000, max: 36000, default: 500 }),
    ], [
      Some(CheckboxDefinition { name: "サイズ固定".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    DirectionalBlurFilter => (46, 0x44000000, 2, 0, 0, "方向ブラー",
    [
      Some(TrackbarDefinition { name: "範囲".to_owned(), scale: 1, min: 0, max: 500, default: 20 }),
      Some(TrackbarDefinition { name: "角度".to_owned(), scale: 10, min: -36000, max: 36000, default: 500 }),
    ], [
    ]),
    LensBlur => (47, 0x04000020, 2, 1, 0, "レンズブラー",
    [
      Some(TrackbarDefinition { name: "範囲".to_owned(), scale: 1, min: 0, max: 1000, default: 5 }),
      Some(TrackbarDefinition { name: "光の強さ".to_owned(), scale: 1, min: 0, max: 60, default: 32 }),
    ], [
      Some(CheckboxDefinition { name: "サイズ固定".to_owned(), is_checkbox: true, default: 1 }),
    ]),
    LensBlurFilter => (48, 0x04000000, 2, 0, 0, "レンズブラー",
    [
      Some(TrackbarDefinition { name: "範囲".to_owned(), scale: 1, min: 0, max: 1000, default: 5 }),
      Some(TrackbarDefinition { name: "光の強さ".to_owned(), scale: 1, min: 0, max: 60, default: 32 }),
    ], [
    ]),
    MotionBlur => (49, 0x04000020, 2, 3, 0, "モーションブラー",
    [
      Some(TrackbarDefinition { name: "間隔".to_owned(), scale: 1, min: 0, max: 100, default: 1 }),
      Some(TrackbarDefinition { name: "分解能".to_owned(), scale: 1, min: 1, max: 25, default: 10 }),
    ], [
      Some(CheckboxDefinition { name: "残像".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "オフスクリーン描画".to_owned(), is_checkbox: true, default: 1 }),
      Some(CheckboxDefinition { name: "出力時に分解能を上げる".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    MotionBlurFilter => (50, 0x04000000, 2, 1, 0, "モーションブラー",
    [
      Some(TrackbarDefinition { name: "間隔".to_owned(), scale: 1, min: 0, max: 100, default: 1 }),
      Some(TrackbarDefinition { name: "分解能".to_owned(), scale: 1, min: 1, max: 25, default: 10 }),
    ], [
      Some(CheckboxDefinition { name: "出力時に分解能を上げる".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    Position => (51, 0x04008020, 3, 0, 0, "座標",
    [
      Some(TrackbarDefinition { name: "X".to_owned(), scale: 10, min: -999999, max: 999999, default: 0 }),
      Some(TrackbarDefinition { name: "Y".to_owned(), scale: 10, min: -999999, max: 999999, default: 0 }),
      Some(TrackbarDefinition { name: "Z".to_owned(), scale: 10, min: -999999, max: 999999, default: 0 }),
    ], [
    ]),
    Zoom => (52, 0x04008020, 3, 0, 0, "拡大率",
    [
      Some(TrackbarDefinition { name: "拡大率".to_owned(), scale: 100, min: 0, max: 500000, default: 10000 }),
      Some(TrackbarDefinition { name: "X".to_owned(), scale: 100, min: 0, max: 500000, default: 10000 }),
      Some(TrackbarDefinition { name: "Y".to_owned(), scale: 100, min: 0, max: 500000, default: 10000 }),
    ], [
    ]),
    Transparency => (53, 0x04008020, 1, 0, 0, "透明度",
    [
      Some(TrackbarDefinition { name: "透明度".to_owned(), scale: 10, min: 0, max: 1000, default: 0 }),
    ], [
    ]),
    Rotation => (54, 0x04008020, 3, 0, 0, "回転",
    [
      Some(TrackbarDefinition { name: "X".to_owned(), scale: 100, min: -360000, max: 360000, default: 0 }),
      Some(TrackbarDefinition { name: "Y".to_owned(), scale: 100, min: -360000, max: 360000, default: 0 }),
      Some(TrackbarDefinition { name: "Z".to_owned(), scale: 100, min: -360000, max: 360000, default: 0 }),
    ], [
    ]),
    AreaExpansion => (55, 0x04008020, 4, 1, 0, "領域拡張",
    [
      Some(TrackbarDefinition { name: "上".to_owned(), scale: 1, min: 0, max: 4000, default: 0 }),
      Some(TrackbarDefinition { name: "下".to_owned(), scale: 1, min: 0, max: 4000, default: 0 }),
      Some(TrackbarDefinition { name: "左".to_owned(), scale: 1, min: 0, max: 4000, default: 0 }),
      Some(TrackbarDefinition { name: "右".to_owned(), scale: 1, min: 0, max: 4000, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "塗りつぶし".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    Resize => (56, 0x04008020, 3, 2, 0, "リサイズ",
    [
      Some(TrackbarDefinition { name: "拡大率".to_owned(), scale: 100, min: 0, max: 500000, default: 10000 }),
      Some(TrackbarDefinition { name: "X".to_owned(), scale: 100, min: 0, max: 500000, default: 10000 }),
      Some(TrackbarDefinition { name: "Y".to_owned(), scale: 100, min: 0, max: 500000, default: 10000 }),
    ], [
      Some(CheckboxDefinition { name: "補間なし".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "ドット数でサイズ指定".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    Rotation90 => (57, 0x04008020, 1, 0, 0, "ローテーション",
    [
      Some(TrackbarDefinition { name: "90度回転".to_owned(), scale: 1, min: -4, max: 4, default: 0 }),
    ], [
    ]),
    Vibration => (58, 0x04000020, 4, 2, 0, "振動",
    [
      Some(TrackbarDefinition { name: "X".to_owned(), scale: 1, min: -500, max: 500, default: 10 }),
      Some(TrackbarDefinition { name: "Y".to_owned(), scale: 1, min: -500, max: 500, default: 10 }),
      Some(TrackbarDefinition { name: "Z".to_owned(), scale: 1, min: -500, max: 500, default: 0 }),
      Some(TrackbarDefinition { name: "周期".to_owned(), scale: 1, min: 1, max: 100, default: 1 }),
    ], [
      Some(CheckboxDefinition { name: "ランダムに強さを変える".to_owned(), is_checkbox: true, default: 1 }),
      Some(CheckboxDefinition { name: "複雑に振動".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    VibrationFilter => (59, 0x04000000, 4, 2, 0, "振動",
    [
      Some(TrackbarDefinition { name: "X".to_owned(), scale: 1, min: -500, max: 500, default: 10 }),
      Some(TrackbarDefinition { name: "Y".to_owned(), scale: 1, min: -500, max: 500, default: 10 }),
      Some(TrackbarDefinition { name: "Z".to_owned(), scale: 1, min: -500, max: 500, default: 0 }),
      Some(TrackbarDefinition { name: "周期".to_owned(), scale: 1, min: 1, max: 100, default: 1 }),
    ], [
      Some(CheckboxDefinition { name: "ランダムに強さを変える".to_owned(), is_checkbox: true, default: 1 }),
      Some(CheckboxDefinition { name: "複雑に振動".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    Inversion => (60, 0x04008020, 0, 5, 0, "反転",
    [
    ], [
      Some(CheckboxDefinition { name: "上下反転".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "左右反転".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "輝度反転".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "色相反転".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "透明度反転".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    InversionFilter => (61, 0x04000000, 0, 4, 0, "反転",
    [
    ], [
      Some(CheckboxDefinition { name: "上下反転".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "左右反転".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "輝度反転".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "色相反転".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    Mirror => (62, 0x04000420, 3, 2, 4, "ミラー",
    [
      Some(TrackbarDefinition { name: "透明度".to_owned(), scale: 10, min: 0, max: 1000, default: 0 }),
      Some(TrackbarDefinition { name: "減衰".to_owned(), scale: 10, min: 0, max: 5000, default: 0 }),
      Some(TrackbarDefinition { name: "境目調整".to_owned(), scale: 1, min: -2000, max: 2000, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "上側".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "中心の位置を変更".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    Raster => (63, 0x04000020, 3, 2, 0, "ラスター",
    [
      Some(TrackbarDefinition { name: "横幅".to_owned(), scale: 1, min: 0, max: 2000, default: 100 }),
      Some(TrackbarDefinition { name: "高さ".to_owned(), scale: 1, min: 0, max: 2000, default: 100 }),
      Some(TrackbarDefinition { name: "周期".to_owned(), scale: 100, min: -4000, max: 4000, default: 100 }),
    ], [
      Some(CheckboxDefinition { name: "縦ラスター".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "ランダム振幅".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    RasterFilter => (64, 0x04000000, 3, 2, 0, "ラスター",
    [
      Some(TrackbarDefinition { name: "横幅".to_owned(), scale: 1, min: 0, max: 2000, default: 100 }),
      Some(TrackbarDefinition { name: "高さ".to_owned(), scale: 1, min: 0, max: 2000, default: 100 }),
      Some(TrackbarDefinition { name: "周期".to_owned(), scale: 100, min: -4000, max: 4000, default: 100 }),
    ], [
      Some(CheckboxDefinition { name: "縦ラスター".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "ランダム振幅".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    Ripple => (65, 0x04000420, 5, 1, 12, "波紋",
    [
      Some(TrackbarDefinition { name: "中心X".to_owned(), scale: 1, min: -4000, max: 4000, default: 0 }),
      Some(TrackbarDefinition { name: "中心Y".to_owned(), scale: 1, min: -4000, max: 4000, default: 0 }),
      Some(TrackbarDefinition { name: "幅".to_owned(), scale: 10, min: 10, max: 4000, default: 300 }),
      Some(TrackbarDefinition { name: "高さ".to_owned(), scale: 10, min: -4000, max: 4000, default: 150 }),
      Some(TrackbarDefinition { name: "速度".to_owned(), scale: 10, min: -20000, max: 20000, default: 1500 }),
    ], [
      Some(CheckboxDefinition { name: "詳細設定".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    ImageLoop => (66, 0x04000020, 4, 1, 0, "画像ループ",
    [
      Some(TrackbarDefinition { name: "横回数".to_owned(), scale: 1, min: 1, max: 400, default: 1 }),
      Some(TrackbarDefinition { name: "縦回数".to_owned(), scale: 1, min: 1, max: 400, default: 1 }),
      Some(TrackbarDefinition { name: "速度X".to_owned(), scale: 10, min: -10000, max: 10000, default: 0 }),
      Some(TrackbarDefinition { name: "速度Y".to_owned(), scale: 10, min: -10000, max: 10000, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "個別オブジェクト".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    ImageLoopFilter => (67, 0x04000000, 4, 0, 0, "画像ループ",
    [
      Some(TrackbarDefinition { name: "横回数".to_owned(), scale: 1, min: 1, max: 400, default: 1 }),
      Some(TrackbarDefinition { name: "縦回数".to_owned(), scale: 1, min: 1, max: 400, default: 1 }),
      Some(TrackbarDefinition { name: "速度X".to_owned(), scale: 10, min: -10000, max: 10000, default: 0 }),
      Some(TrackbarDefinition { name: "速度Y".to_owned(), scale: 10, min: -10000, max: 10000, default: 0 }),
    ], [
    ]),
    PolarTransform => (68, 0x04000020, 4, 0, 0, "極座標変換",
    [
      Some(TrackbarDefinition { name: "中心幅".to_owned(), scale: 1, min: 0, max: 2000, default: 0 }),
      Some(TrackbarDefinition { name: "拡大率".to_owned(), scale: 10, min: 1000, max: 8000, default: 1000 }),
      Some(TrackbarDefinition { name: "回転".to_owned(), scale: 10, min: -36000, max: 36000, default: 0 }),
      Some(TrackbarDefinition { name: "渦巻".to_owned(), scale: 100, min: -800, max: 800, default: 0 }),
    ], [
    ]),
    Displacement => (69, 0x04000620, 8, 3, 268, "ディスプレイスメントマップ",
    [
      Some(TrackbarDefinition { name: "param0".to_owned(), scale: 10, min: -40000, max: 40000, default: 0 }),
      Some(TrackbarDefinition { name: "param1".to_owned(), scale: 10, min: -40000, max: 40000, default: 0 }),
      Some(TrackbarDefinition { name: "X".to_owned(), scale: 10, min: -40000, max: 40000, default: 0 }),
      Some(TrackbarDefinition { name: "Y".to_owned(), scale: 10, min: -40000, max: 40000, default: 0 }),
      Some(TrackbarDefinition { name: "回転".to_owned(), scale: 100, min: -360000, max: 360000, default: 0 }),
      Some(TrackbarDefinition { name: "サイズ".to_owned(), scale: 1, min: 0, max: 4000, default: 200 }),
      Some(TrackbarDefinition { name: "縦横比".to_owned(), scale: 10, min: -1000, max: 1000, default: 0 }),
      Some(TrackbarDefinition { name: "ぼかし".to_owned(), scale: 1, min: 0, max: 1000, default: 5 }),
    ], [
      Some(CheckboxDefinition { name: "背景".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "移動変形".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "元のサイズに合わせる".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    Noise => (70, 0x04000420, 7, 3, 16, "ノイズ",
    [
      Some(TrackbarDefinition { name: "強さ".to_owned(), scale: 10, min: 0, max: 2000, default: 1000 }),
      Some(TrackbarDefinition { name: "速度X".to_owned(), scale: 10, min: -4000, max: 8000, default: 0 }),
      Some(TrackbarDefinition { name: "速度Y".to_owned(), scale: 10, min: -4000, max: 8000, default: 0 }),
      Some(TrackbarDefinition { name: "変化速度".to_owned(), scale: 10, min: 0, max: 8000, default: 0 }),
      Some(TrackbarDefinition { name: "周期X".to_owned(), scale: 100, min: 0, max: 10000, default: 100 }),
      Some(TrackbarDefinition { name: "周期Y".to_owned(), scale: 100, min: 0, max: 10000, default: 100 }),
      Some(TrackbarDefinition { name: "しきい値".to_owned(), scale: 10, min: 0, max: 1000, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "アルファ値と乗算".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "Type1".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "設定".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    ColorShift => (71, 0x04000420, 3, 1, 4, "色ずれ",
    [
      Some(TrackbarDefinition { name: "ずれ幅".to_owned(), scale: 1, min: 0, max: 2000, default: 5 }),
      Some(TrackbarDefinition { name: "角度".to_owned(), scale: 10, min: -36000, max: 36000, default: 0 }),
      Some(TrackbarDefinition { name: "強さ".to_owned(), scale: 1, min: 0, max: 100, default: 100 }),
    ], [
      Some(CheckboxDefinition { name: "赤緑A".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    ColorShiftFilter => (72, 0x04000400, 3, 1, 4, "色ずれ",
    [
      Some(TrackbarDefinition { name: "ずれ幅".to_owned(), scale: 1, min: 0, max: 2000, default: 5 }),
      Some(TrackbarDefinition { name: "角度".to_owned(), scale: 10, min: -36000, max: 36000, default: 0 }),
      Some(TrackbarDefinition { name: "強さ".to_owned(), scale: 1, min: 0, max: 100, default: 100 }),
    ], [
      Some(CheckboxDefinition { name: "赤緑".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    Monochromatic => (73, 0x04000420, 1, 2, 4, "単色化",
    [
      Some(TrackbarDefinition { name: "強さ".to_owned(), scale: 10, min: 0, max: 1000, default: 1000 }),
    ], [
      Some(CheckboxDefinition { name: "色の設定".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "輝度を保持する".to_owned(), is_checkbox: true, default: 1 }),
    ]),
    MonochromaticFilter => (74, 0x04000400, 1, 2, 4, "単色化",
    [
      Some(TrackbarDefinition { name: "強さ".to_owned(), scale: 10, min: 0, max: 1000, default: 1000 }),
    ], [
      Some(CheckboxDefinition { name: "色の設定".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "輝度を保持する".to_owned(), is_checkbox: true, default: 1 }),
    ]),
    Gradation => (75, 0x04000420, 5, 4, 16, "グラデーション",
    [
      Some(TrackbarDefinition { name: "強さ".to_owned(), scale: 10, min: 0, max: 1000, default: 1000 }),
      Some(TrackbarDefinition { name: "中心X".to_owned(), scale: 1, min: -2000, max: 2000, default: 0 }),
      Some(TrackbarDefinition { name: "中心Y".to_owned(), scale: 1, min: -2000, max: 2000, default: 0 }),
      Some(TrackbarDefinition { name: "角度".to_owned(), scale: 10, min: -36000, max: 36000, default: 0 }),
      Some(TrackbarDefinition { name: "幅".to_owned(), scale: 1, min: 0, max: 2000, default: 100 }),
    ], [
      Some(CheckboxDefinition { name: "通常".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "線".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "開始色".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "終了色".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    ColorSettingEx => (76, 0x04000220, 3, 3, 0, "拡張色設定",
    [
      Some(TrackbarDefinition { name: "R".to_owned(), scale: 0, min: 0, max: 255, default: 0 }),
      Some(TrackbarDefinition { name: "G".to_owned(), scale: 0, min: 0, max: 255, default: 0 }),
      Some(TrackbarDefinition { name: "B".to_owned(), scale: 0, min: 0, max: 255, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "RGB⇔HSV".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "開始色".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "終了色".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    ColorSettingExFilter => (77, 0x04000200, 3, 3, 0, "拡張色設定",
    [
      Some(TrackbarDefinition { name: "R".to_owned(), scale: 0, min: 0, max: 255, default: 0 }),
      Some(TrackbarDefinition { name: "G".to_owned(), scale: 0, min: 0, max: 255, default: 0 }),
      Some(TrackbarDefinition { name: "B".to_owned(), scale: 0, min: 0, max: 255, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "RGB⇔HSV".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "開始色".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "終了色".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    GamutConversion => (78, 0x04000420, 3, 2, 16, "特定色域変換",
    [
      Some(TrackbarDefinition { name: "色相範囲".to_owned(), scale: 1, min: 0, max: 256, default: 8 }),
      Some(TrackbarDefinition { name: "彩度範囲".to_owned(), scale: 1, min: 0, max: 256, default: 8 }),
      Some(TrackbarDefinition { name: "境界補正".to_owned(), scale: 1, min: 0, max: 8, default: 2 }),
    ], [
      Some(CheckboxDefinition { name: "変換前の色の取得".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "変換後の色の取得".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    AnimationEffect => (79, 0x04000420, 4, 2, 516, "アニメーション効果",
    [
      Some(TrackbarDefinition { name: "track0".to_owned(), scale: 100, min: 0, max: 0, default: 0 }),
      Some(TrackbarDefinition { name: "track1".to_owned(), scale: 100, min: 0, max: 0, default: 0 }),
      Some(TrackbarDefinition { name: "track2".to_owned(), scale: 100, min: 0, max: 0, default: 0 }),
      Some(TrackbarDefinition { name: "track3".to_owned(), scale: 100, min: 0, max: 0, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "震える".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "check0".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    CustomObject => (80, 0x04000408, 4, 2, 516, "カスタムオブジェクト",
    [
      Some(TrackbarDefinition { name: "track0".to_owned(), scale: 100, min: 0, max: 0, default: 0 }),
      Some(TrackbarDefinition { name: "track1".to_owned(), scale: 100, min: 0, max: 0, default: 0 }),
      Some(TrackbarDefinition { name: "track2".to_owned(), scale: 100, min: 0, max: 0, default: 0 }),
      Some(TrackbarDefinition { name: "track3".to_owned(), scale: 100, min: 0, max: 0, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "集中線".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "check0".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    Script => (81, 0x04000420, 0, 0, 2048, "スクリプト制御",
    [
    ], [
    ]),
    VideoComposition => (82, 0x04000420, 5, 5, 284, "動画ファイル合成",
    [
      Some(TrackbarDefinition { name: "再生位置".to_owned(), scale: 1, min: 1, max: 1, default: 1 }),
      Some(TrackbarDefinition { name: "再生速度".to_owned(), scale: 10, min: -20000, max: 20000, default: 1000 }),
      Some(TrackbarDefinition { name: "X".to_owned(), scale: 1, min: -2000, max: 2000, default: 0 }),
      Some(TrackbarDefinition { name: "Y".to_owned(), scale: 1, min: -2000, max: 2000, default: 0 }),
      Some(TrackbarDefinition { name: "拡大率".to_owned(), scale: 10, min: 0, max: 8000, default: 1000 }),
    ], [
      Some(CheckboxDefinition { name: "ループ再生".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "動画ファイルの同期".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "ループ画像".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "参照ファイル".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "色情報を上書き".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    ImageComposition => (83, 0x04000420, 3, 3, 260, "画像ファイル合成",
    [
      Some(TrackbarDefinition { name: "X".to_owned(), scale: 1, min: -1000, max: 1000, default: 0 }),
      Some(TrackbarDefinition { name: "Y".to_owned(), scale: 1, min: -1000, max: 1000, default: 0 }),
      Some(TrackbarDefinition { name: "拡大率".to_owned(), scale: 10, min: 0, max: 8000, default: 1000 }),
    ], [
      Some(CheckboxDefinition { name: "ループ画像".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "参照ファイル".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "前方から合成".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    Deinterlacing => (84, 0x04000420, 0, 1, 4, "インターレース解除",
    [
    ], [
      Some(CheckboxDefinition { name: "奇数解除".to_owned(), is_checkbox: false, default: 0 }),
    ]),
    CameraOption => (85, 0x04000020, 0, 4, 0, "カメラ制御オプション",
    [
    ], [
      Some(CheckboxDefinition { name: "カメラの方を向く".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "カメラの方を向く(縦横方向のみ)".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "カメラの方を向く(横方向のみ)".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "シャドーの対象から外す".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    OffScreen => (86, 0x04000020, 0, 0, 0, "オフスクリーン描画",
    [
    ], [
    ]),
    Split => (87, 0x04000020, 2, 0, 0, "オブジェクト分割",
    [
      Some(TrackbarDefinition { name: "横分割数".to_owned(), scale: 1, min: 1, max: 100, default: 10 }),
      Some(TrackbarDefinition { name: "縦分割数".to_owned(), scale: 1, min: 1, max: 100, default: 10 }),
    ], [
    ]),
    PartialFilter => (88, 0x44000500, 6, 2, 260, "部分フィルタ",
    [
      Some(TrackbarDefinition { name: "X".to_owned(), scale: 10, min: -40000, max: 40000, default: 0 }),
      Some(TrackbarDefinition { name: "Y".to_owned(), scale: 10, min: -40000, max: 40000, default: 0 }),
      Some(TrackbarDefinition { name: "回転".to_owned(), scale: 100, min: -360000, max: 360000, default: 0 }),
      Some(TrackbarDefinition { name: "サイズ".to_owned(), scale: 1, min: 0, max: 4000, default: 100 }),
      Some(TrackbarDefinition { name: "縦横比".to_owned(), scale: 10, min: -1000, max: 1000, default: 0 }),
      Some(TrackbarDefinition { name: "ぼかし".to_owned(), scale: 1, min: 0, max: 1000, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "背景".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "マスクの反転".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    AudioFade => (89, 0x04200020, 2, 0, 0, "音量フェード",
    [
      Some(TrackbarDefinition { name: "イン".to_owned(), scale: 100, min: 0, max: 1000, default: 50 }),
      Some(TrackbarDefinition { name: "アウト".to_owned(), scale: 100, min: 0, max: 1000, default: 50 }),
    ], [
    ]),
    AudioDelay => (90, 0x04200020, 2, 0, 0, "音声ディレイ",
    [
      Some(TrackbarDefinition { name: "強さ".to_owned(), scale: 10, min: 0, max: 1000, default: 500 }),
      Some(TrackbarDefinition { name: "遅延(ms)".to_owned(), scale: 1, min: 0, max: 1000, default: 100 }),
    ], [
    ]),
    AudioDelayFilter => (91, 0x04200000, 2, 0, 0, "音声ディレイ",
    [
      Some(TrackbarDefinition { name: "強さ".to_owned(), scale: 10, min: 0, max: 1000, default: 500 }),
      Some(TrackbarDefinition { name: "遅延(ms)".to_owned(), scale: 1, min: 0, max: 1000, default: 100 }),
    ], [
    ]),
    Monaural => (92, 0x04200020, 1, 0, 0, "モノラル化",
    [
      Some(TrackbarDefinition { name: "比率".to_owned(), scale: 1, min: -1000, max: 1000, default: 0 }),
    ], [
    ]),
    TimeControl => (93, 0x05000400, 3, 1, 20, "時間制御",
    [
      Some(TrackbarDefinition { name: "位置".to_owned(), scale: 100, min: 0, max: 10000, default: 0 }),
      Some(TrackbarDefinition { name: "繰り返し".to_owned(), scale: 1, min: 1, max: 100, default: 1 }),
      Some(TrackbarDefinition { name: "コマ落ち".to_owned(), scale: 1, min: 1, max: 100, default: 1 }),
    ], [
      Some(CheckboxDefinition { name: "フレーム番号指定".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    GroupControl => (94, 0x45000420, 7, 2, 20, "グループ制御",
    [
      Some(TrackbarDefinition { name: "X".to_owned(), scale: 10, min: -999999, max: 999999, default: 0 }),
      Some(TrackbarDefinition { name: "Y".to_owned(), scale: 10, min: -999999, max: 999999, default: 0 }),
      Some(TrackbarDefinition { name: "Z".to_owned(), scale: 10, min: -999999, max: 999999, default: 0 }),
      Some(TrackbarDefinition { name: "拡大率".to_owned(), scale: 100, min: 0, max: 500000, default: 10000 }),
      Some(TrackbarDefinition { name: "X軸回転".to_owned(), scale: 100, min: -360000, max: 360000, default: 0 }),
      Some(TrackbarDefinition { name: "Y軸回転".to_owned(), scale: 100, min: -360000, max: 360000, default: 0 }),
      Some(TrackbarDefinition { name: "Z軸回転".to_owned(), scale: 100, min: -360000, max: 360000, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "上位グループ制御の影響を受ける".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "同じグループのオブジェクトを対象にする".to_owned(), is_checkbox: true, default: 1 }),
    ]),
    CameraControl => (95, 0x45800400, 10, 1, 20, "カメラ制御",
    [
      Some(TrackbarDefinition { name: "X".to_owned(), scale: 10, min: -999999, max: 999999, default: 0 }),
      Some(TrackbarDefinition { name: "Y".to_owned(), scale: 10, min: -999999, max: 999999, default: 0 }),
      Some(TrackbarDefinition { name: "Z".to_owned(), scale: 10, min: -999999, max: 999999, default: -10240 }),
      Some(TrackbarDefinition { name: "目標X".to_owned(), scale: 10, min: -999999, max: 999999, default: 0 }),
      Some(TrackbarDefinition { name: "目標Y".to_owned(), scale: 10, min: -999999, max: 999999, default: 0 }),
      Some(TrackbarDefinition { name: "目標Z".to_owned(), scale: 10, min: -999999, max: 999999, default: 0 }),
      Some(TrackbarDefinition { name: "目標ﾚｲﾔ".to_owned(), scale: 1, min: 0, max: 100, default: 0 }),
      Some(TrackbarDefinition { name: "傾き".to_owned(), scale: 100, min: -360000, max: 360000, default: 0 }),
      Some(TrackbarDefinition { name: "深度ぼけ".to_owned(), scale: 10, min: 0, max: 100, default: 0 }),
      Some(TrackbarDefinition { name: "視野角".to_owned(), scale: 100, min: 0, max: 12000, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "Zバッファ/シャドウマップを有効にする".to_owned(), is_checkbox: true, default: 1 }),
    ]),
    CameraControlEx => (96, 0x45800400, 8, 2, 4, "カメラ制御(拡張描画)",
    [
      None,
      None,
      None,
      None,
      None,
      None,
      None,
      None,
    ], [
      None,
      None,
    ]),
    CameraEffect => (97, 0x05000400, 4, 2, 516, "カメラ効果",
    [
      Some(TrackbarDefinition { name: "track0".to_owned(), scale: 100, min: 0, max: 0, default: 0 }),
      Some(TrackbarDefinition { name: "track1".to_owned(), scale: 100, min: 0, max: 0, default: 0 }),
      Some(TrackbarDefinition { name: "track2".to_owned(), scale: 100, min: 0, max: 0, default: 0 }),
      Some(TrackbarDefinition { name: "track3".to_owned(), scale: 100, min: 0, max: 0, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "手ぶれ".to_owned(), is_checkbox: false, default: 0 }),
      Some(CheckboxDefinition { name: "check0".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    CameraShadow => (98, 0x45000000, 5, 0, 0, "シャドー(カメラ制御)",
    [
      Some(TrackbarDefinition { name: "光源X".to_owned(), scale: 10, min: -200000, max: 200000, default: 10000 }),
      Some(TrackbarDefinition { name: "光源Y".to_owned(), scale: 10, min: -200000, max: 200000, default: -20000 }),
      Some(TrackbarDefinition { name: "光源Z".to_owned(), scale: 10, min: -200000, max: 200000, default: -20000 }),
      Some(TrackbarDefinition { name: "濃さ".to_owned(), scale: 10, min: 0, max: 1000, default: 400 }),
      Some(TrackbarDefinition { name: "精度".to_owned(), scale: 1, min: 20, max: 100, default: 50 }),
    ], [
    ]),
    CameraScript => (99, 0x05000400, 0, 0, 2048, "スクリプト(カメラ制御)",
    [
    ], [
    ]),
    DenoiseFilter => (100, 0x02000000, 3, 0, 0, "ノイズ除去フィルタ",
    [
      Some(TrackbarDefinition { name: "強さ".to_owned(), scale: 1, min: 0, max: 256, default: 256 }),
      Some(TrackbarDefinition { name: "範囲".to_owned(), scale: 1, min: 1, max: 3, default: 2 }),
      Some(TrackbarDefinition { name: "しきい値".to_owned(), scale: 1, min: 0, max: 256, default: 24 }),
    ], [
    ]),
    SharpenFilter => (101, 0x02000000, 4, 0, 0, "シャープフィルタ",
    [
      Some(TrackbarDefinition { name: "強さ".to_owned(), scale: 1, min: 0, max: 256, default: 256 }),
      Some(TrackbarDefinition { name: "範囲".to_owned(), scale: 1, min: 1, max: 3, default: 2 }),
      Some(TrackbarDefinition { name: "下限値".to_owned(), scale: 1, min: 0, max: 1024, default: 128 }),
      Some(TrackbarDefinition { name: "上限値".to_owned(), scale: 1, min: 0, max: 1024, default: 1024 }),
    ], [
    ]),
    BlurFilter2 => (102, 0x02000000, 4, 0, 0, "ぼかしフィルタ",
    [
      Some(TrackbarDefinition { name: "強さ".to_owned(), scale: 1, min: 0, max: 256, default: 256 }),
      Some(TrackbarDefinition { name: "範囲".to_owned(), scale: 1, min: 0, max: 3, default: 2 }),
      Some(TrackbarDefinition { name: "下限値".to_owned(), scale: 1, min: 0, max: 1024, default: 0 }),
      Some(TrackbarDefinition { name: "上限値".to_owned(), scale: 1, min: 0, max: 1024, default: 1024 }),
    ], [
    ]),
    ClipResizeFilter => (103, 0x02004400, 4, 0, 12, "クリッピング＆リサイズ",
    [
      Some(TrackbarDefinition { name: "上".to_owned(), scale: 1, min: 0, max: 1024, default: 0 }),
      Some(TrackbarDefinition { name: "下".to_owned(), scale: 1, min: 0, max: 1024, default: 0 }),
      Some(TrackbarDefinition { name: "左".to_owned(), scale: 1, min: 0, max: 1024, default: 0 }),
      Some(TrackbarDefinition { name: "右".to_owned(), scale: 1, min: 0, max: 1024, default: 0 }),
    ], [
    ]),
    FillBorder => (104, 0x02000000, 4, 2, 0, "縁塗りつぶし",
    [
      Some(TrackbarDefinition { name: "上".to_owned(), scale: 1, min: -1024, max: 1024, default: 0 }),
      Some(TrackbarDefinition { name: "下".to_owned(), scale: 1, min: -1024, max: 1024, default: 0 }),
      Some(TrackbarDefinition { name: "左".to_owned(), scale: 1, min: -1024, max: 1024, default: 0 }),
      Some(TrackbarDefinition { name: "右".to_owned(), scale: 1, min: -1024, max: 1024, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "中央に配置".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "縁の色で塗る".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    ColorCorrection2 => (105, 0x02001000, 6, 0, 0, "色調補正",
    [
      Some(TrackbarDefinition { name: "明るさ".to_owned(), scale: 1, min: -256, max: 256, default: 0 }),
      Some(TrackbarDefinition { name: "コントラスト".to_owned(), scale: 1, min: -256, max: 256, default: 0 }),
      Some(TrackbarDefinition { name: "ガンマ".to_owned(), scale: 1, min: -256, max: 256, default: 0 }),
      Some(TrackbarDefinition { name: "輝度".to_owned(), scale: 1, min: -256, max: 256, default: 0 }),
      Some(TrackbarDefinition { name: "色の濃さ".to_owned(), scale: 1, min: -256, max: 256, default: 0 }),
      Some(TrackbarDefinition { name: "色合い".to_owned(), scale: 1, min: -256, max: 256, default: 0 }),
    ], [
    ]),
    ColorCorrectionEx => (106, 0x02001000, 15, 3, 0, "拡張色調補正",
    [
      Some(TrackbarDefinition { name: "Y(offs)".to_owned(), scale: 1, min: -256, max: 256, default: 0 }),
      Some(TrackbarDefinition { name: "Y(gain)".to_owned(), scale: 1, min: -256, max: 256, default: 0 }),
      Some(TrackbarDefinition { name: "Cb(offs)".to_owned(), scale: 1, min: -256, max: 256, default: 0 }),
      Some(TrackbarDefinition { name: "Cb(gain)".to_owned(), scale: 1, min: -256, max: 256, default: 0 }),
      Some(TrackbarDefinition { name: "Cr(offs)".to_owned(), scale: 1, min: -256, max: 256, default: 0 }),
      Some(TrackbarDefinition { name: "Cr(gain)".to_owned(), scale: 1, min: -256, max: 256, default: 0 }),
      Some(TrackbarDefinition { name: "R(offs)".to_owned(), scale: 1, min: -256, max: 256, default: 0 }),
      Some(TrackbarDefinition { name: "R(gain)".to_owned(), scale: 1, min: -256, max: 256, default: 0 }),
      Some(TrackbarDefinition { name: "R(gamm)".to_owned(), scale: 1, min: -256, max: 256, default: 0 }),
      Some(TrackbarDefinition { name: "G(offs)".to_owned(), scale: 1, min: -256, max: 256, default: 0 }),
      Some(TrackbarDefinition { name: "G(gain)".to_owned(), scale: 1, min: -256, max: 256, default: 0 }),
      Some(TrackbarDefinition { name: "G(gamm)".to_owned(), scale: 1, min: -256, max: 256, default: 0 }),
      Some(TrackbarDefinition { name: "B(offs)".to_owned(), scale: 1, min: -256, max: 256, default: 0 }),
      Some(TrackbarDefinition { name: "B(gain)".to_owned(), scale: 1, min: -256, max: 256, default: 0 }),
      Some(TrackbarDefinition { name: "B(gamm)".to_owned(), scale: 1, min: -256, max: 256, default: 0 }),
    ], [
      Some(CheckboxDefinition { name: "RGBの同期".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "TV -> PC スケール補正".to_owned(), is_checkbox: true, default: 0 }),
      Some(CheckboxDefinition { name: "PC -> TV スケール補正".to_owned(), is_checkbox: true, default: 0 }),
    ]),
    VolumeFilter => (107, 0x02200000, 1, 0, 0, "音量の調整",
    [
      Some(TrackbarDefinition { name: "レベル".to_owned(), scale: 1, min: -256, max: 256, default: 0 }),
    ], [
    ]),
}
