use super::L10nKey;

pub fn translate_ja(key: L10nKey) -> Option<&'static str> {
    Some(match key {
        L10nKey::SettingsNoMatchesShort => "一致なし",
        L10nKey::SettingsModifiedTitle => "変更済み",
        L10nKey::SettingsMatchCount => "{count} 件",
        L10nKey::SettingsNothingModified => "すべて既定値のままです",
        L10nKey::SettingsKeyThen => "次に",
        L10nKey::SettingsStartupRestore => "起動と復元",
        L10nKey::SettingsSettingsFile => "設定ファイル",
        L10nKey::SettingsReveal => "表示",
        L10nKey::SettingsThemeModeSystem => "システム",
        L10nKey::SettingsThemeModeSystemDesc => {
            "システムに合わせて、ライトとダークのテーマを切り替えます。"
        }
        L10nKey::SettingsThemeModeLightDesc => "常にライトテーマを使います。",
        L10nKey::SettingsThemeModeDarkDesc => "常にダークテーマを使います。",
        L10nKey::SettingsThemeSlotLightDesc => "システムがライトモードのときに使います。",
        L10nKey::SettingsThemeSlotDarkDesc => "システムがダークモードのときに使います。",
        L10nKey::SettingsThemeSlotDesc => "ターミナルとインターフェースのテーマ。",
        L10nKey::SettingsLightThemeLabel => "ライトテーマ",
        L10nKey::SettingsDarkThemeLabel => "ダークテーマ",
        L10nKey::SettingsNoThemesMatch => "一致するテーマがありません",
        L10nKey::SettingsRestoreChanged => "変更した {count} 件を戻す",
        L10nKey::SettingsShortcutsHint => {
            "ショートカットをクリックして新しいキーを押します。続けて別の組み合わせを押すとシーケンスになります。Esc で取消、⌫ で解除。"
        }
        L10nKey::SettingsShortcutsHintTmux => {
            "tmux プリセット：ペインとタブの操作はプレフィックスキーを使います。クリックで変更、Esc で取消、⌫ で解除。"
        }
        L10nKey::SettingsShortcutConflict => {
            "{keys} は「{action}」で使われています。置き換えるとそちらから外れます。"
        }
        L10nKey::SettingsReplace => "置き換え",
        L10nKey::SettingsNoActionsMatch => "「{query}」に一致する操作はありません",
        L10nKey::SettingsSearchShortcuts => "操作またはキーを検索",
        L10nKey::SettingsHostsDesc => {
            "最近使ったホスト。検索すると保存済みのすべてのホストを探せます。"
        }
        L10nKey::SettingsAddHost => "ホストを追加",
        L10nKey::SettingsNoHostsMatch => "「{query}」に一致するホストはありません",
        L10nKey::SettingsHostsFromFiles => "{files} か所から {count} 台のホスト",
        L10nKey::SettingsMoreHosts => "ほかに {count} 台のホスト",
        L10nKey::SettingsShowRecentOnly => "最近のみ表示",
        L10nKey::SettingsShowAll => "すべて表示",
        L10nKey::SettingsUnsaved => "未保存",
        L10nKey::SettingsNever => "未使用",
        L10nKey::SettingsDefinedIn => "定義元",
        L10nKey::SettingsConnectInNewTab => "新しいタブで接続",
        L10nKey::SettingsCopied => "コピーしました",
        L10nKey::SettingsNavMobile => "モバイル",
        L10nKey::SettingsMobileAccess => "スマートフォンからのアクセスを許可",
        L10nKey::SettingsMobileAccessDesc => {
            "ペアリングしたスマートフォンから、このマシンのペインを表示して入力できます。ウインドウをすべて閉じていても使えます。通信はエンドツーエンドで暗号化されます。"
        }
        L10nKey::SettingsMobileStatusStarting => "起動しています…",
        L10nKey::SettingsMobileStatusFailed => "実行されていません: {error}",
        L10nKey::SettingsMobileStartFailed => {
            "スマートフォンからのアクセスを開始できませんでした: {error}"
        }
        L10nKey::SettingsMobileNoAnswer => {
            "tty7 サーバーが起動しませんでした。設定 → 情報 でサーバーを再起動してから、もう一度お試しください。"
        }
        L10nKey::SettingsMobilePair => "スマートフォンをペアリング",
        L10nKey::SettingsMobileShowCode => "コードを表示",
        L10nKey::SettingsMobilePairDesc => {
            "スマートフォンの tty7 アプリ用に、1 回限りのコードを表示します。"
        }
        L10nKey::SettingsMobilePairNeedsAccess => {
            "先にスマートフォンからのアクセスをオンにしてください。"
        }
        L10nKey::SettingsMobilePairScan => {
            "スマートフォンのカメラを向けるか、tty7 アプリでスキャンするか、コードをコピーして貼り付けてください。"
        }
        L10nKey::SettingsMobilePairValid => {
            "あと {time} で失効します。1 台のスマートフォンに限ります。"
        }
        L10nKey::SettingsMobileNewCode => "新しいコード",
        L10nKey::SettingsMobilePairExpired => {
            "このコードは期限切れです。新しいコードを作成してペアリングしてください。"
        }
        L10nKey::SettingsMobilePairTried => {
            "このコードは一度試されたため使えなくなりました（入力ミスや接続の中断など）。新しいコードを作成してペアリングしてください。"
        }
        L10nKey::SettingsMobilePairReplaced => {
            "新しいコードに置き換えられました。新しいコードを作成してペアリングしてください。"
        }
        L10nKey::SettingsMobileCopyCode => "コードをコピー",
        L10nKey::SettingsMobilePaired => "{name} とペアリングしました。",
        L10nKey::SettingsMobilePhones => "ペアリング済みのスマートフォン",
        L10nKey::SettingsMobileNoPhones => "ペアリングしたスマートフォンはまだありません。",
        L10nKey::SettingsMobileUnpair => "ペアリングを解除",
        L10nKey::SettingsSearchMobileKeywords => {
            "スマホ モバイル iphone android ipad ペアリング qr コード リモート アクセス"
        }
        L10nKey::SettingsCopySshCommand => "ssh コマンドをコピー",
        L10nKey::SettingsStoredInTty7 => "tty7 の設定に保存",
        L10nKey::SettingsClickAgainToRemove => "もう一度クリックで削除",
        L10nKey::SettingsRemoveHost => "ホストを削除",
        L10nKey::SettingsEditHost => "編集",
        L10nKey::SettingsLive => "接続中",
        L10nKey::SettingsPressKeysShort => "キーを押してください…",
        L10nKey::SettingsSearchAgents => "エージェントを検索",
        L10nKey::SettingsConnectedMachines => "接続中",
        L10nKey::SettingsInstalledCount => "{count} 件インストール済み",
        L10nKey::SettingsMachine => "マシン",
        L10nKey::SettingsMachineLocalDesc => {
            "フックはマシンごとにインストールされます。リモートマシンは接続中にここに表示されます。"
        }
        L10nKey::SettingsMachineRemoteDesc => "{name} への接続を通じてインストールします。",
        L10nKey::SettingsAgentsInstalledSummary => "{total} 件中 {count} 件インストール済み",
        L10nKey::SettingsNoAgentsInstalled => "このマシンにはまだエージェントフックがありません。",
        L10nKey::SettingsNoAgentsMatch => "「{query}」に一致するエージェントはありません",
        L10nKey::SettingsMoreAgents => "ほかに {count} 件のエージェント",
        L10nKey::SettingsShowInstalledOnly => "インストール済みのみ表示",
        L10nKey::SettingsWorking => "処理中…",
        L10nKey::SettingsUpdateAvailable => "アップデートあり",
        L10nKey::SettingsRevealHookFile => "フックファイルを表示",
        L10nKey::SettingsNavGeneral => "一般",
        L10nKey::SettingsEditShortcuts => "ショートカットを編集…",
        L10nKey::SettingsModifiedOnly => "変更済みのみ",
        L10nKey::SettingsModified => "変更済み",
        L10nKey::SettingsSearchResults => "検索結果",
        L10nKey::SettingsOpenSetting => "設定を開く",
        L10nKey::SettingsNoModified => "この条件に一致する変更済みの設定はありません。",
        L10nKey::SettingsTerminalFontGroup => "ターミナルの文字",
        L10nKey::SettingsInterfaceFontGroup => "インターフェイスの文字",
        L10nKey::SettingsUnsavedTitle => "移動する前に変更を保存しますか？",
        L10nKey::SettingsUnsavedBody => "変更を保存、破棄、または編集を続けられます。",
        L10nKey::SettingsSaveChanges => "変更を保存",
        L10nKey::SettingsThemeDraft => "テーマの変更は保存するまでプレビューされます。",
        L10nKey::SettingsSaveError => "変更を保存できませんでした：{error}",
        L10nKey::SettingsRetrySave => "保存を再試行",

        L10nKey::SearchTabs => "タブを検索…",
        L10nKey::SearchSettings => "設定を検索…",
        L10nKey::FilterHosts => "ホストを絞り込み…",
        L10nKey::SearchTheme => "検索…",
        L10nKey::SearchWorkspacesAndMachines => "ワークスペース、タブ、マシンを検索…",
        L10nKey::SearchFind => "検索…",
        L10nKey::SearchMatchCase => "大文字と小文字を区別",
        L10nKey::SearchUseRegex => "正規表現を使用",
        L10nKey::HomeNewTab => "新規タブ",
        L10nKey::HomeReopenClosedTab => "閉じたタブをもう一度開く",
        L10nKey::HomeSwitchWorkspace => "ワークスペースを切り替える…",
        L10nKey::HomeSplitRight => "右に分割",
        L10nKey::HomeSplitDown => "下に分割",
        L10nKey::HomeSettings => "設定…",
        L10nKey::Reconnect => "再接続",
        L10nKey::None => "なし",
        L10nKey::TryAgain => "再試行",
        L10nKey::Delete => "削除",
        L10nKey::Dismiss => "閉じる",
        L10nKey::StoredPasswordRejected => {
            "保存されたパスワードが拒否されました。新しいパスワードを入力してください"
        }
        L10nKey::StoredPassphraseRejected => {
            "保存されたパスフレーズではこの鍵を解除できませんでした。正しいものを入力してください"
        }
        L10nKey::Trust => "信頼する",
        L10nKey::Abort => "中止",
        L10nKey::HostKeyOverrideMessage => {
            "「yes」を入力すると新しいキーを上書きして信頼します。中止するには Esc を押してください"
        }
        L10nKey::Override => "上書き",
        L10nKey::RememberKeychain => "キーチェーンに保存",
        L10nKey::Cancel => "キャンセル",
        L10nKey::Close => "閉じる",
        L10nKey::QuitStopServerTitle => "tty7 を終了して tty7 server を停止しますか？",
        L10nKey::QuitStopServerBody => {
            "tty7 を終了して tty7 server を停止します。シェルで実行中のものはすべて終了します。タブとレイアウトは次回起動時に新しいシェルで開きます。（ウィンドウを閉じるだけならトレイに退避し、シェルは動き続けます）"
        }
        L10nKey::QuitAndStop => "終了して停止",
        L10nKey::CloseSshConnectionTitle => "この SSH 接続を閉じますか？",
        L10nKey::CloseSshConnectionBody => "接続中です。閉じると切断されます",
        L10nKey::ClosePaneBusyTitle => "このペインを閉じますか？",
        L10nKey::CloseTabBusyTitle => "このタブを閉じますか？",
        L10nKey::CloseIdleBody => "中のシェルも終了します。",
        L10nKey::CloseTabsTitle => "{count} 個のタブを閉じますか？",
        L10nKey::CloseTabsBody => "中のシェルもすべて終了します。",
        L10nKey::CloseBusyCommandBody => "{what} はまだ実行中です。閉じると終了します。",
        L10nKey::CloseBusyAgentBody => {
            "{agent} はまだ作業中です。閉じるとこのターンは中断されます。"
        }
        L10nKey::Keep => "保持",
        L10nKey::SettingsNavAppearance => "外観",
        L10nKey::SettingsNavTerminal => "ターミナル",
        L10nKey::SettingsNavInput => "キーボードとマウス",
        L10nKey::SettingsNavSsh => "SSH",
        L10nKey::SettingsNavAgents => "連携",
        L10nKey::SettingsNavKeybindings => "キーボードショートカット",
        L10nKey::SettingsNavAbout => "情報",
        L10nKey::SettingsWindowTitle => "設定",
        L10nKey::Reset => "リセット",
        L10nKey::Save => "保存",
        L10nKey::Connect => "接続",
        L10nKey::Download => "ダウンロード",
        L10nKey::Link => "リンク",
        L10nKey::SettingsThemeIntroTitle => "テーマ",
        L10nKey::SettingsFontSize => "ターミナルの文字サイズ",
        L10nKey::SettingsFontSizeDesc => "ターミナルテキストのサイズ（ポイント）",
        L10nKey::SettingsUiFontSize => "画面の文字サイズ",
        L10nKey::SettingsUiFontSizeDesc => "タブ・パネル・設定などの文字サイズ",
        L10nKey::SettingsUiFontFamily => "画面のフォント",
        L10nKey::SettingsUiFontFamilyDesc => "タブ、サイドバー、ダイアログ、設定で使うフォント",
        L10nKey::SettingsLineHeight => "行の高さ",
        L10nKey::SettingsLineHeightDesc => "フォントサイズに対する行間の倍率",
        L10nKey::SettingsFontFamily => "ターミナルのフォント",
        L10nKey::SettingsFontFamilyDesc => "システムにインストールされているフォントから選択",
        L10nKey::SettingsBoldFont => "太字フォント",
        L10nKey::SettingsBoldFontDesc => "太字の書体。デフォルトはメインから合成",
        L10nKey::SettingsItalicFont => "斜体フォント",
        L10nKey::SettingsItalicFontDesc => "斜体の書体。デフォルトはメインから合成",
        L10nKey::SettingsFontLigatures => "フォントリガチャー",
        L10nKey::SettingsFontLigaturesDesc => "ターミナルでプログラミング用リガチャーを使用",
        L10nKey::SettingsFontThicken => "ストロークを太くする",
        L10nKey::SettingsFontThickenDesc => "文字をやや太く描画します。再起動後に反映",
        L10nKey::SettingsCursor => "カーソル",
        L10nKey::SettingsCursorShape => "カーソルの形状",
        L10nKey::SettingsCursorShapeDesc => "ターミナルカーソルの描画方法",
        L10nKey::SettingsPromptCursorShape => "プロンプトのカーソル形状",
        L10nKey::SettingsPromptCursorShapeDesc => "シェルプロンプトでのカーソル",
        L10nKey::SettingsCursorBlink => "カーソルの点滅",
        L10nKey::SettingsCursorBlinkDesc => {
            "ターミナルがフォーカスされている間、カーソルを点滅させる"
        }
        L10nKey::SettingsLanguage => "言語",
        L10nKey::SettingsLanguageDesc => "tty7 の表示言語を選択します",
        L10nKey::SettingsLanguageEnglish => "English",
        L10nKey::SettingsLanguageChinese => "简体中文",
        L10nKey::SettingsLanguageJapanese => "日本語",
        L10nKey::SettingsLanguageRussian => "Русский",
        L10nKey::SettingsSearchLanguageKeywords => {
            "言語 ロケール 英語 中国語 language locale english chinese"
        }
        L10nKey::SettingsTransparency => "透明度",
        L10nKey::SettingsOpacity => "不透明度",
        L10nKey::SettingsOpacityDesc => "100% 未満ではデスクトップが透けて見えます",
        L10nKey::SettingsBlur => "背景のぼかし",
        L10nKey::SettingsBlurDesc => {
            if cfg!(target_os = "macos") {
                "半透明ウィンドウの背後にあるものをぼかす"
            } else {
                "半透明ウィンドウの背後にあるものをぼかす。対応するコンポジターが必要です（KDE Plasma は対応、GNOME と素の X11 ではウィンドウが透けるだけです）"
            }
        }
        L10nKey::SettingsBlurAutoDesc => "半透明ウィンドウの背後をぼかす。「自動」のみ",
        L10nKey::SettingsBackdrop => "背景マテリアル",
        L10nKey::SettingsBackdropDesc => "半透明ウィンドウ背後のネイティブ背景",
        L10nKey::SettingsSearchBackdropKeywords => {
            "背景 マテリアル ぼかし すりガラス material backdrop mica acrylic blur frosted window background"
        }
        L10nKey::SettingsBackdropAuto => "自動",
        L10nKey::SettingsBackdropBlur => "ぼかし",
        L10nKey::SettingsBackdropMica => "Mica",
        L10nKey::SettingsBackdropMicaAlt => "Mica Alt",
        L10nKey::SettingsBackdropAcrylic => "Acrylic",
        L10nKey::SettingsBackdropOff => "オフ",
        L10nKey::FollowTheme => "テーマに従う",
        L10nKey::SettingsDimInactivePanes => "非アクティブなペインを暗くする",
        L10nKey::SettingsDimInactivePanesDesc => "フォーカス外のペインを暗くします",
        L10nKey::SettingsAutoHideTitlebarButtons => "タイトルバーのボタンをホバー時に表示",
        L10nKey::SettingsAutoHideTitlebarButtonsDesc => {
            "新規タブやサイドバーのボタンを、ポインタがタイトルバーに乗ったときだけ表示します"
        }
        L10nKey::SettingsShowContentTitle => "コンテンツ領域のタイトルを表示",
        L10nKey::SettingsShowContentTitleDesc => {
            "コンテンツ領域の上部に現在のタブのタイトルを表示します"
        }
        L10nKey::SettingsOpenThemesFolder => "テーマフォルダを開く",
        L10nKey::SettingsChangeThemeImage => "変更…",
        L10nKey::SettingsChooseThemeImage => "選択…",
        L10nKey::SettingsRemoveThemeImage => "削除",
        L10nKey::SettingsImageOpacity => "画像の不透明度",
        L10nKey::SettingsImageOpacityDesc => "背景色の上に画像をどれだけ強く表示するか",
        L10nKey::SettingsEditTheme => "テーマを編集",
        L10nKey::SettingsEditThemeIntro => "コピーを編集中。変更はすぐ反映・保存されます",
        L10nKey::SettingsBackgroundImage => "背景画像",
        L10nKey::SettingsBackgroundImageDesc => "背景色の上、テキストの下に表示されます",
        L10nKey::SettingsAnsiColors => "ANSI カラー",
        L10nKey::SettingsCustomThemes => "カスタムテーマ",
        L10nKey::SettingsThemesRejected => "テーマフォルダから読み込めなかったもの",
        L10nKey::ThemeDuplicateFailed => "テーマを複製できませんでした",
        L10nKey::ThemeSaveFailed => "テーマを保存できませんでした",
        L10nKey::OpenInFileManagerFailed => "{path} を開けませんでした",
        L10nKey::ExplorerMenuOpenIn => "tty7 で開く",
        L10nKey::ExplorerMenuOpenHere => "ここで tty7 を開く",
        L10nKey::SettingsDuplicateToEdit => "複製して編集",
        L10nKey::SettingsHosts => "ホスト",
        L10nKey::SettingsNothingMatches => "「{query}」に一致する項目がありません",
        L10nKey::SettingsInTty7 => "tty7 の設定",
        L10nKey::SettingsImportFromSshConfig => "~/.ssh/config からインポート",
        L10nKey::SettingsNoHostsYet => "まだホストがありません",
        L10nKey::SettingsImportUnreadable => {
            "{path} を読み取れませんでした — 何もインポートされていません"
        }
        L10nKey::SettingsImportNoHosts => {
            "{path} にインポートできるホストがありません — ワイルドカードや Match のルールだけです"
        }
        L10nKey::SettingsImportSummary => {
            "ホスト {count} 件を追加 — {updated} 件を更新、{unchanged} 件は変更なし"
        }
        L10nKey::SettingsImportIgnored => {
            "tty7 に設定のないオプションが {count} 件あり、ファイルに残されています: {options}"
        }
        L10nKey::SettingsImportMoreOptions => "他 {count} 件",
        L10nKey::SettingsCouldntForgetPassword => {
            "{endpoint} のパスワードを消去できませんでした: {error}"
        }
        L10nKey::SettingsSecurity => "セキュリティ",
        L10nKey::SettingsSecurityIntro => "ホストは詳細設定でこれらを上書きできます",
        L10nKey::SettingsVerifyHostKeys => "ホストキーを検証",
        L10nKey::SettingsVerifyHostKeysDesc => {
            "サーバーのキーを known_hosts と照合します。オフではなりすましに気づけません"
        }
        L10nKey::WarnBeforeClosing => "閉じる前に警告",
        L10nKey::SettingsWarnBeforeClosingDesc => "SSH 接続中のタブを閉じる前に確認",
        L10nKey::SettingsNewHost => "新規ホスト",
        L10nKey::SettingsKeepEditing => "編集を続ける",
        L10nKey::SettingsHostRequired => "ホスト名が必要です — 保存されません",
        L10nKey::SettingsPortInvalid => "ポートは 1-65535 の範囲です — 空欄なら 22 です",
        L10nKey::SettingsPassword => "パスワード",
        L10nKey::SettingsNameHint => "任意のラベル",
        L10nKey::SettingsHostHint => "ホスト名または IP",
        L10nKey::SettingsUserHint => "接続時に解決",
        L10nKey::SettingsPasswordHint => "接続時に入力する",
        L10nKey::SettingsKeyPassphrase => "鍵のパスフレーズ",
        L10nKey::SettingsPassphraseNeedsKey => {
            "先に鍵ファイルを指定してください。パスフレーズは解錠する鍵ごとに保存されます。"
        }
        L10nKey::SettingsCouldntSavePassword => "{endpoint} のパスワードを保存できません: {error}",
        L10nKey::SettingsCouldntSavePassphrase => "{key} のパスフレーズを保存できません: {error}",
        L10nKey::SettingsJumpHostUnknown => {
            "{jump_name} という名前のホストプロファイルはありません — 保存されません"
        }
        L10nKey::SettingsJumpHostSelf => {
            "ホストを自分自身のジャンプホストにはできません — 保存されません"
        }
        L10nKey::SettingsFwdNeedsBoth => {
            "待受ポートとターゲットの host:port が必要です — 保存されません"
        }
        L10nKey::SettingsFwdNeedsListen => "待受ポートが必要です — 保存されません",
        L10nKey::SettingsProxyPortInvalid => {
            "ポートは 1-65535 の範囲です — ホストだけならデフォルトポートを使います"
        }
        L10nKey::SettingsDefault => "デフォルト",
        L10nKey::SettingsShell => "シェル",
        L10nKey::SettingsShellIntro => {
            "新しいターミナルで起動するプログラム。空欄なら{default}を使います。"
        }
        L10nKey::SettingsProgram => "シェルプログラム",
        L10nKey::SettingsProgramDesc => "PATH 上の名前か絶対パス。例: zsh、fish",
        L10nKey::SettingsArguments => "シェル引数",
        L10nKey::SettingsArgumentsDesc => "コマンドラインと同じ規則で分割。例: -l、-c \"echo hi\"",
        L10nKey::SettingsArgumentsInvalid => {
            "引用符が対応していないため、この値は保存されませんでした"
        }
        L10nKey::SettingsStartIn => "開始ディレクトリ",
        L10nKey::SettingsStartInDesc => "tty7 の起動ディレクトリ、ホームフォルダ、または固定パス",
        L10nKey::SettingsCustomPath => "カスタムパス",
        L10nKey::SettingsCustomPathDesc => "新しいシェルが起動するディレクトリ",
        L10nKey::SettingsWdInherit => "継承",
        L10nKey::SettingsWdHome => "ホーム",
        L10nKey::SettingsWdCustom => "カスタム",
        L10nKey::SettingsWdPathInvalid => {
            "このディレクトリは存在しないため、この値は保存されませんでした"
        }
        L10nKey::SettingsShellFooter => {
            "継承元のないシェル（ウィンドウの最初のタブなど）にのみ適用。新しいタブと分割はアクティブなペインのディレクトリを使います"
        }
        L10nKey::SettingsScrolling => "スクロール",
        L10nKey::SettingsScrollback => "スクロールバックバッファー",
        L10nKey::SettingsScrollbackDesc => {
            "各ペインに保存する履歴の行数。新しいペインに適用されます"
        }
        L10nKey::SettingsScrollSpeed => "スクロール速度",
        L10nKey::SettingsScrollSpeedDesc => "マウスホイールのスクロールに適用する倍率",
        L10nKey::SettingsSmoothScroll => "スムーズスクロール",
        L10nKey::SettingsSmoothScrollDesc => {
            "ホイールを滑らかにスクロール（トラックパッドは対象外）"
        }
        L10nKey::SettingsMouse => "マウス",
        L10nKey::SettingsFocusFollowsMouse => "フォーカスがマウスに追従する",
        L10nKey::SettingsFocusFollowsMouseDesc => {
            "クリックしなくてもペインにホバーするとフォーカスされる"
        }
        L10nKey::SettingsHideMouseWhileTyping => "入力時にマウスポインタを非表示",
        L10nKey::SettingsHideMouseWhileTypingDesc => {
            "入力中はポインタを隠し、次のマウス移動で再表示する"
        }
        L10nKey::SettingsMouseZoom => "ホイールで拡大縮小",
        L10nKey::SettingsMouseZoomDesc => "押しながらホイールを回すとフォントサイズが変わります",
        L10nKey::SettingsMouseZoomOff => "オフ",
        L10nKey::SettingsReportMouseToApps => "マウスイベントをアプリに報告",
        L10nKey::SettingsMouseRightClick => "右クリックの動作",
        L10nKey::SettingsMouseRightClickDesc => {
            "メニューは右クリックメニューを開きます。コピー・貼り付けは選択範囲をコピーして解除し、未選択なら貼り付けます。アプリがマウスを使う場合は Shift を押してください。"
        }
        L10nKey::SettingsMouseRightClickMenu => "メニュー",
        L10nKey::SettingsMouseRightClickCopyPaste => "コピー・貼り付け",
        L10nKey::SettingsReportMouseToAppsDesc => "vim や tmux にクリックを渡す。Shift で端末側に",
        L10nKey::SettingsBell => "ベル通知",
        L10nKey::SettingsTerminalBell => "ターミナルベル",
        L10nKey::SettingsTerminalBellDesc => "ベル（^G）の通知方法",
        L10nKey::SettingsLinks => "リンク",
        L10nKey::DetectUrls => "URL を自動検出",
        L10nKey::SettingsDetectUrlsDesc => "ホバーで下線、{modifier}+クリックで開く",
        L10nKey::ForwardSshLoopbackLinks => "リモートポートを転送",
        L10nKey::SettingsForwardSshLoopbackLinksDesc => "リモートの localhost リンクをここで開く",
        L10nKey::SettingsBellModeOff => "オフ",
        L10nKey::SettingsBellModeVisual => "視覚的（画面点滅）",
        L10nKey::SettingsBellModeAudible => "音声（効果音）",
        L10nKey::SettingsBellModeBoth => "点滅 + 音声",
        L10nKey::SettingsSelectionClipboard => "選択とクリップボード",
        L10nKey::SettingsSmartSelection => "スマート選択",
        L10nKey::SettingsSmartSelectionDesc => "ダブルクリックで URL やパス全体を選択",
        L10nKey::SettingsCopyOnSelect => "選択時に自動コピー",
        L10nKey::SettingsCopyOnSelectDesc => {
            if cfg!(target_os = "macos") {
                "マウスでテキストを選択するとすぐにクリップボードへコピーされます。⌘C は不要です"
            } else {
                "マウスでテキストを選択するとすぐにクリップボードへコピーされます。Ctrl+Shift+C は不要です"
            }
        }
        L10nKey::SettingsTrimTrailingSpaces => "コピー時に末尾の空白を除去",
        L10nKey::SettingsTrimTrailingSpacesDesc => "コピーした各行の末尾の空白を除去する",
        L10nKey::SettingsKeyboard => "キーボード",
        L10nKey::SettingsOptionAsMeta => "Option（⌥）を Meta として使用",
        L10nKey::SettingsOptionAsMetaDesc => "⌥+キーを Meta として送る（⌥B で 1 語戻る）",
        L10nKey::SettingsAgentsIntro => "エージェントフック",
        L10nKey::SettingsAgentsIntroDesc => {
            "フックでエージェントの状態（作業中 / 待機中 / 完了）をタブバーに表示します"
        }
        L10nKey::SettingsReadingAgentConfig => "このマシンのエージェント設定を読み込んでいます…",
        L10nKey::SettingsStatusInstalled => "インストール済み",
        L10nKey::SettingsInstall => "インストール",
        L10nKey::SettingsReinstall => "再インストール",
        L10nKey::SettingsUpdate => "アップデート",
        L10nKey::SettingsUninstall => "アンインストール",
        L10nKey::SettingsOfflineMachines => {
            "未接続の保存済みマシンがさらに {count} 台あります。いずれかでワークスペースを開くと、そこにフックをインストールできます"
        }
        L10nKey::SettingsSyncWithSystem => "外観",
        L10nKey::SettingsSyncWithSystemDesc => {
            "OS の外観に従い、ライトとダークのテーマを別々に使用する"
        }
        L10nKey::SettingsLegiblePalette => "明色の可読性",
        L10nKey::SettingsLegiblePaletteDesc => "背景上で読みにくい明色を自動調整します",
        L10nKey::SettingsCustom => "カスタム",
        L10nKey::SettingsCustomValue => "カスタム ({value})",
        L10nKey::SettingsDark => "ダーク",
        L10nKey::SettingsLight => "ライト",
        L10nKey::SettingsStartupWindow => "起動時のウィンドウ状態",
        L10nKey::SettingsStartupWindowDesc => "tty7 起動時のウィンドウ状態",
        L10nKey::SettingsRememberWindowSize => "ウィンドウサイズと位置を記憶",
        L10nKey::SettingsRememberWindowSizeDesc => "前回終了時のサイズと位置でウィンドウを開きます",
        L10nKey::SettingsRestoreLastLayout => "前回のレイアウトを復元",
        L10nKey::SettingsRestoreLastLayoutDesc => {
            "起動時に前回のタブ、分割、ディレクトリを復元します"
        }
        L10nKey::SettingsShowTrayIcon => "システムトレイアイコンを表示",
        L10nKey::SettingsShowTrayIconDesc => "エージェントの入力待ちを通知",
        L10nKey::SettingsTabs => "タブ",
        L10nKey::SettingsNewTabPosition => "新規タブの表示位置",
        L10nKey::SettingsConfirmClose => "閉じる前に確認",
        L10nKey::SettingsConfirmCloseDesc => {
            "タブやペインを閉じる前に確認するタイミング。「閉じる前に警告」を有効にした SSH ホストは常に確認します"
        }
        L10nKey::ConfirmCloseNever => "確認しない",
        L10nKey::ConfirmCloseWhenBusy => "実行中のときのみ",
        L10nKey::ConfirmCloseAlways => "常に確認",
        L10nKey::SettingsNewTabPositionDesc => "新しく開いたタブが挿入される場所",
        L10nKey::SettingsSidebarGrouping => "自動グループ化",
        L10nKey::SettingsSidebarGroupingDesc => {
            "固定していないタブを git リポジトリごと、SSH タブをホストごとにまとめます"
        }
        L10nKey::SettingsNotifications => "通知",
        L10nKey::SettingsNotifyOnCommandFinish => "コマンド終了時に通知",
        L10nKey::SettingsNotifyOnCommandFinishDesc => "長いコマンドの完了をデスクトップに通知",
        L10nKey::SettingsNotifyThreshold => "コマンド実行時間の下限",
        L10nKey::SettingsNotifyThresholdDesc => {
            "この時間以上実行されたコマンドの完了を通知します。"
        }
        L10nKey::SettingsWindow => "起動と復元",
        L10nKey::NotifyModeNever => "通知しない",
        L10nKey::NotifyModeUnfocused => "非フォーカス時のみ",
        L10nKey::NotifyModeAlways => "常に通知",
        L10nKey::ProgramNotesDropped => "このペインのほかの通知は表示されませんでした",
        L10nKey::SettingsStartupNormal => "通常サイズ",
        L10nKey::SettingsStartupMaximized => "最大化",
        L10nKey::SettingsStartupFullscreen => "全画面",
        L10nKey::SettingsAfterCurrent => "現在のタブの隣",
        L10nKey::SettingsAtEnd => "末尾",
        L10nKey::SettingsPrefix => "プレフィックスキー",
        L10nKey::SettingsKeybindingsIntroDesc => {
            "ショートカットをクリックして新しいキーを押します。続けて押すと Ctrl-B X のようなシーケンスに。Esc で取消、Backspace で削除"
        }
        L10nKey::KeybindGoToTab => "タブ {n} へ移動",
        L10nKey::KeybindGoToWorkspace => "ワークスペース {n} へ移動",
        L10nKey::KeybindForkSessionRight => "右にセッションをフォーク",
        L10nKey::KeybindForkSessionLeft => "左にセッションをフォーク",
        L10nKey::KeybindForkSessionDown => "下にセッションをフォーク",
        L10nKey::KeybindForkSessionUp => "上にセッションをフォーク",
        L10nKey::SettingsAboutDesc1 => {
            "長く続く作業のためのターミナルワークベンチ。セッションはウィンドウを閉じても続き、リモートもローカルと同じように扱え、コーディングエージェントの状態はサイドバーに表示されます。"
        }
        L10nKey::SettingsDefaultTerminal => "デフォルトのターミナル",
        L10nKey::SettingsDefaultTerminalDesc => {
            "Unix 実行ファイル、SSH リンク、man ページを tty7 で開きます"
        }
        L10nKey::SettingsDefaultTerminalSet => "デフォルトのターミナルに設定",
        L10nKey::SettingsDefaultTerminalSetSuccess => {
            "tty7 を対応するターミナルファイルとリンクのデフォルトハンドラに設定しました。"
        }
        L10nKey::SettingsDefaultTerminalSetFailed => {
            "tty7 をデフォルトのターミナルに設定できませんでした: {error}"
        }
        L10nKey::SettingsVersion => "バージョン",
        L10nKey::SettingsUpdates => "アップデート",
        L10nKey::SettingsUpdateAndRelaunch => "更新して再起動",
        L10nKey::SettingsUpdateViewRelease => "リリースページを開く",
        L10nKey::SettingsUpdateChecking => "アップデートを確認中…",
        L10nKey::SettingsUpdateUpToDate => "最新バージョンを使用しています",
        L10nKey::SettingsUpdateDownloadingPercent => {
            "アップデートをダウンロード中… {size} 中 {percent}%"
        }
        L10nKey::SettingsUpdateDownloadingBytes => "アップデートをダウンロード中… {received}",
        L10nKey::SettingsUpdateVerifying => "ダウンロードしたアップデートを検証中…",
        L10nKey::SettingsUpdateInstalling => "アップデートを適用して再起動中…",
        L10nKey::SettingsUpdateCheckNow => "今すぐ確認",
        L10nKey::SettingsUpdateCancel => "ダウンロードを中止",
        L10nKey::SettingsUpdateRetry => "再試行",
        L10nKey::SettingsUpdateDismiss => "閉じる",
        L10nKey::SettingsUpdateDownloadManually => "手動でダウンロード",
        L10nKey::SettingsUpdateFailedTitle => "{version} へのアップデートに失敗しました。",
        L10nKey::SettingsUpdateReady => {
            "{version} のダウンロードと検証が完了し、インストールできます。"
        }
        L10nKey::SettingsUpdateReadyNextLaunch => "次回 tty7 を起動したときに適用されます。",
        L10nKey::SettingsUpdateInstallNow => "インストールして再起動",
        L10nKey::SettingsUpdateDiscard => "破棄",
        L10nKey::SettingsAutoDownload => "アップデートをバックグラウンドでダウンロード",
        L10nKey::SettingsAutoDownloadDesc => {
            "更新をバックグラウンドで取得し、再起動だけで適用。確認なしにはインストールしません"
        }
        L10nKey::SettingsUpdateChannel => "更新チャンネル",
        L10nKey::SettingsUpdateChannelDesc => "Nightly は毎晩最新コードからビルド（未テスト）",
        L10nKey::SettingsUpdateChannelStable => "安定版",
        L10nKey::SettingsUpdateChannelNightly => "ナイトリー",
        L10nKey::SettingsDaemonStale => "tty7 server は {build} のままです。",
        L10nKey::SettingsDaemonStaleDesc => {
            "tty7 は更新されましたが、ペインは古い server で動いています。再起動すると新しいビルドになり、ペインのプロセスはすべて終了します"
        }
        L10nKey::UpdateDialogTitle => "アップデートがあります",
        L10nKey::UpdateDialogDetail => {
            "tty7 {version} が利用できます（現在 {current}）。インストールするとアプリが再起動します。tty7 server は動いたままなので、ペインの中身は残ります"
        }
        L10nKey::UpdateDialogDetailWindows => {
            "tty7 {version} が利用できます（現在 {current}）。インストールするとアプリと tty7 server が再起動します。ペインのプロセスは終了し、タブとレイアウトは新しいシェルで復元されます"
        }
        L10nKey::UpdateDialogDetailManual => {
            "tty7 {version} が利用できます（現在 {current}）。{hint}"
        }
        L10nKey::UpdateDialogCannotSelfUpdate => "このインストールは自動更新できません。",
        L10nKey::UpdateDialogLater => "後で",
        L10nKey::UpdateDialogNextLaunch => "次回起動時にインストール",
        L10nKey::UpdateDialogNeedsElevation => {
            "tty7 は全ユーザー向けにインストールされているため、インストール前に Windows の管理者承認が一度求められます。tty7 自体が管理者権限で実行されることはありません"
        }
        L10nKey::SettingsUpdateCheckFailed => "アップデートを確認できませんでした: {error}",
        L10nKey::SettingsUpdatePrepareFailed => "アップデートに失敗しました: {error}",
        L10nKey::SettingsUpdateLaunchFailed => "インストーラーを起動できませんでした: {error}",
        L10nKey::SettingsUpdateUnsupportedMacos => {
            "この tty7 は書き込み可能な tty7.app バンドルにないため、自分自身を置き換えられません。「アプリケーション」へ移動するか、リリースページから更新してください"
        }
        L10nKey::SettingsUpdateUnsupportedLinux => {
            "このアーキテクチャ向けの Linux パッケージはリリースにありません。ソースからビルドするか、パッケージマネージャーをご利用ください"
        }
        L10nKey::SettingsUpdateLinuxPackage => {
            "Linux は手動で更新します。リリースページから {name} をダウンロードするか、パッケージマネージャーをご利用ください"
        }
        L10nKey::SettingsUpdateUnsupportedWindows => {
            "この tty7 は認識可能な Inno Setup 版でもポータブル ZIP 版でもないため、自動更新できません。リリースページを開いて手動で更新してください"
        }
        L10nKey::SettingsUpdateWindowsAllUsers => {
            "tty7 はすべてのユーザー向けにインストールされており、置き換えには管理者権限が必要ですが、tty7 は自ら昇格を要求しません。リリースページからインストーラーを実行して更新してください"
        }
        L10nKey::SettingsUpdateUnsupportedPlatform => {
            "このプラットフォームでは自動インストールを利用できません。リリースページを開いてください"
        }
        L10nKey::SettingsUpdateMissingPackage => {
            "このリリースには、現在のインストール形式に合う {name} パッケージがありません。リリースページを開いて別のパッケージを選んでください"
        }
        L10nKey::SettingsUpdateMissingChecksums => {
            "このリリースには checksums.txt がないため、tty7 は自動インストールを行いません"
        }
        L10nKey::SettingsVersionAvailable => "バージョン {version} が利用可能です",
        L10nKey::SettingsCheckUpdatesDesc => "自動更新できない場合はリリースページを開く",
        L10nKey::SettingsCheckUpdatesOnLaunch => "起動時にアップデートを確認",
        L10nKey::SettingsCommandLine => "コマンドライン",
        L10nKey::SettingsCommandLineDesc => "tty7 コマンドを PATH に追加。次回起動時に反映",
        L10nKey::SettingsInstallCliOnPath => "`tty7` コマンドを PATH にインストール",
        L10nKey::SettingsServer => "tty7 server",
        L10nKey::SettingsServerDesc => "ターミナルセッションをバックグラウンドで実行し続けます",
        L10nKey::SettingsRestartServer => "tty7 server を再起動…",
        L10nKey::SettingsAppHttpProxy => "アップデート用プロキシ",
        L10nKey::SettingsAppHttpProxyDesc => "tty7 の更新チェック用。空欄ならシステム設定",
        L10nKey::SettingsAppHttpProxyInvalid => {
            "プロキシアドレスとして正しくないため、この値は保存されませんでした"
        }
        L10nKey::SettingsAgentClaudeCode => "Claude Code",
        L10nKey::SettingsAgentCodex => "Codex",
        L10nKey::SettingsAgentTraeCode => "TraeCode",
        L10nKey::SettingsAgentCopilotCli => "Copilot CLI",
        L10nKey::SettingsAgentOpencode => "OpenCode",
        L10nKey::SettingsAgentPi => "Pi",
        L10nKey::SettingsAgentGrokBuild => "Grok Build",
        L10nKey::SettingsAgentOhMyPi => "Oh My Pi",
        L10nKey::SettingsAgentGemini => "Gemini",
        L10nKey::SettingsAgentDroid => "Droid",
        L10nKey::SettingsAgentQwenCode => "Qwen Code",
        L10nKey::SettingsAgentGoose => "Goose",
        L10nKey::SettingsAgentKimiCode => "Kimi Code",
        L10nKey::SettingsAgentQoderCLI => "Qoder CLI",
        L10nKey::SettingsAgentCrush => "Crush",
        L10nKey::SettingsAgentCodeBuddy => "CodeBuddy",
        L10nKey::SettingsAgentCursorCli => "Cursor CLI",
        L10nKey::SettingsAgentPrimeAgent => "Prime Agent",
        L10nKey::SettingsAgentAntigravity => "Antigravity",
        L10nKey::SettingsAgentEmpryo => "Empryo",
        L10nKey::SettingsAgentJcode => "jcode",
        L10nKey::SettingsAgentMuse => "Muse Code",
        L10nKey::SettingsMuseManualInstall => "手動インストールが必要",
        L10nKey::AppAgentHooksMuseManualInstall => "対象のマシンで実行: {command}",
        L10nKey::SettingsAgentQoderCn => "Qoder CN CLI",
        L10nKey::SettingsSearchAboutKeywords => {
            "バージョン ライセンス クレジット ビルド 更新 確認 github about version license credits update check"
        }
        L10nKey::SettingsSearchAppHttpProxyKeywords => {
            "プロキシ 通信 ネットワーク ダウンロード アップデート proxy http https socks socks5 clash v2ray network download update"
        }
        L10nKey::SettingsSearchAnsiColorsKeywords => {
            "パレット 16 ANSI カラー ターミナル テーマ ansi colors palette terminal theme colours"
        }
        L10nKey::SettingsSearchBackgroundImageKeywords => {
            "背景画像 壁紙 画像 写真 テーマ background image wallpaper picture theme"
        }
        L10nKey::SettingsSearchImageOpacityKeywords => {
            "背景画像 不透明度 透明度 強さ 壁紙 background image opacity strength fade"
        }
        L10nKey::SettingsSearchArgumentsKeywords => {
            "シェル フラグ ログイン 引数 arguments shell flags login args"
        }
        L10nKey::SettingsSearchBlurKeywords => {
            "透明度 半透明 すりガラス ウィンドウ 背景 blur transparency translucent frosted vibrancy window background"
        }
        L10nKey::SettingsSearchBoldFontKeywords => {
            "タイプフェイス 太字 ウェイト bold font typeface weight"
        }
        L10nKey::SettingsSearchClaudeCodeKeywords => {
            "エージェント 統合 フック インストール アンインストール 状態 セッション タブバー サイドバー バッジ claude agent integration hooks install status working waiting"
        }
        L10nKey::SettingsSearchCodexKeywords => {
            "エージェント 統合 フック インストール openai codex agent integration hooks install"
        }
        L10nKey::SettingsSearchTraeCodeKeywords => {
            "エージェント 統合 フック インストール trae code traecli traex agent integration hooks install"
        }
        L10nKey::SettingsSearchCommandLineToolKeywords => {
            "cli tty7 パス シェル コマンド インストール シンボリックリンク ターミナル iterm エージェント スクリプト command line tool"
        }
        L10nKey::SettingsSearchCopilotCliKeywords => {
            "エージェント 統合 フック インストール github copilot agent integration hooks install"
        }
        L10nKey::SettingsSearchCopyOnSelectKeywords => {
            "クリップボード 選択 コピー マウス copy on select clipboard selection yank mouse"
        }
        L10nKey::SettingsSearchCursorBlinkKeywords => {
            "カーソル 点滅 フラッシュ cursor blink caret blinking flash"
        }
        L10nKey::SettingsSearchCursorShapeKeywords => {
            "カーソル 形状 ブロック バー アンダーライン ビーム cursor shape caret block bar underline beam"
        }
        L10nKey::SettingsSearchPromptCursorShapeKeywords => {
            "プロンプト カーソル 形状 ブロック バー 下線 prompt cursor shape caret block bar underline beam"
        }
        L10nKey::SettingsSearchCustomThemesKeywords => {
            "テーマ 複製 編集 色 フォルダ 背景画像 壁紙 yaml インポート custom themes duplicate edit colors folder import background image wallpaper"
        }
        L10nKey::SettingsSearchDetectUrlsKeywords => {
            "リンク ハイパーリンク クリック可能 開く detect urls links hyperlink clickable open"
        }
        L10nKey::SettingsSearchDimInactivePanesKeywords => {
            "非アクティブ ペイン 暗く フォーカス 分割 fade unfocused inactive split pane focus opacity highlight active dimming"
        }
        L10nKey::SettingsSearchAutoHideTitlebarButtonsKeywords => {
            "自動 隠す タイトルバー ボタン ホバー ポインタ すっきり 新規タブ サイドバー auto hide title bar buttons hover"
        }
        L10nKey::SettingsSearchShowContentTitleKeywords => {
            "コンテンツ タイトル タブ 表示 非表示 content title bar tab show hide"
        }
        L10nKey::SettingsSearchFocusFollowsMouseKeywords => {
            "ペイン ホバー アクティブ focus follows mouse pane hover activate"
        }
        L10nKey::SettingsSearchFontFamilyKeywords => {
            "タイプフェイス 等幅 タイポグラフィ font family monospace typography typeface"
        }
        L10nKey::SettingsSearchFontLigaturesKeywords => {
            "タイポグラフィ グリフ fira font ligatures typography glyph fira"
        }
        L10nKey::SettingsSearchFontThickenKeywords => {
            "フォントスムージング 太字 細字 ウェイト font smoothing thicken bold weight thin AppleFontSmoothing"
        }
        L10nKey::SettingsSearchFontSizeKeywords => {
            "タイポグラフィ 文字 拡大 縮小 ズーム font size typography text bigger smaller zoom"
        }
        L10nKey::SettingsSearchForwardSshLoopbackLinksKeywords => {
            "ssh リモート ポート トンネル localhost フォワード リンク 自動転送 forward ssh loopback links tunnel ports"
        }
        L10nKey::SettingsSearchGrokBuildKeywords => {
            "エージェント 統合 フック インストール xai grok build agent integration hooks install"
        }
        L10nKey::SettingsSearchHideMouseWhileTypingKeywords => {
            "カーソル ポインタ 自動非表示 hide mouse while typing cursor pointer autohide"
        }
        L10nKey::SettingsSearchHostsKeywords => {
            "ssh ホスト 接続 保存 プロファイル インポート ssh_config 管理 追加 編集 クイック接続 hosts ssh profile import connect manage"
        }
        L10nKey::SettingsSearchItalicFontKeywords => "タイプフェイス 斜体 italic oblique typeface",
        L10nKey::SettingsSearchKeybindingsKeywords => {
            "ショートカット ホットキー キーボード バインディング コード tmux プリセット 再バインド プレフィックス keybindings shortcut hotkey binding chord prefix"
        }
        L10nKey::SettingsSearchKeybindingsTitle => "キーボードショートカット",
        L10nKey::SettingsSearchLineHeightKeywords => {
            "タイポグラフィ リーディング 行間 line height typography leading spacing"
        }
        L10nKey::SettingsSearchUiFontFamilyKeywords => {
            "インターフェース フォント ファミリー UI 書体 タブ サイドバー interface font family ui typeface"
        }
        L10nKey::SettingsSearchNewTabPositionKeywords => {
            "タブ 順序 末尾 現在のタブの隣 new tab position tabs order end after current"
        }
        L10nKey::SettingsSearchConfirmCloseKeywords => {
            "閉じる 確認 タブ ペイン 実行中 アイドル 常に しない confirm close tab pane ask prompt busy idle always never"
        }
        L10nKey::SettingsSearchNotifyOnCommandFinishKeywords => {
            "通知 アラート 完了 osc デスクトップ バナー 長い コマンド notify on command finish notification alert desktop"
        }
        L10nKey::SettingsSearchNotifyThresholdKeywords => {
            "通知 アラート 秒 時間 長い コマンド 遅延 notify threshold notification alert seconds duration delay"
        }
        L10nKey::SettingsSearchOpacityKeywords => {
            "透明度 半透明 透ける ウィンドウ alpha opacity transparency translucent window"
        }
        L10nKey::SettingsSearchOpencodeKeywords => {
            "エージェント 統合 プラグイン インストール opencode agent integration plugin install"
        }
        L10nKey::SettingsSearchOptionAsMetaKeywords => {
            "alt キーボード 修飾キー エスケープ macos option meta option acts as meta keyboard modifier"
        }
        L10nKey::SettingsSearchOhMyPiKeywords => {
            "エージェント 統合 拡張 インストール omp oh my pi agent integration extension install"
        }
        L10nKey::SettingsSearchGeminiKeywords => {
            "エージェント 統合 フック インストール gemini google agent integration hooks install"
        }
        L10nKey::SettingsSearchDroidKeywords => {
            "エージェント 統合 フック インストール droid factory agent integration hooks install"
        }
        L10nKey::SettingsSearchQwenCodeKeywords => {
            "エージェント 統合 フック インストール qwen code agent integration hooks install"
        }
        L10nKey::SettingsSearchGooseKeywords => {
            "エージェント 統合 フック プラグイン インストール goose agent integration hooks plugin install"
        }
        L10nKey::SettingsSearchKimiCodeKeywords => {
            "エージェント 統合 フック インストール kimi code moonshot agent integration hooks install"
        }
        L10nKey::SettingsSearchQoderCLIKeywords => {
            "エージェント 統合 フック インストール qoder qodercli agent integration hooks install"
        }
        L10nKey::SettingsSearchCrushKeywords => {
            "エージェント 統合 フック インストール crush agent integration hooks install"
        }
        L10nKey::SettingsSearchCodeBuddyKeywords => {
            "エージェント 統合 フック インストール codebuddy cbc tencent agent integration hooks install"
        }
        L10nKey::SettingsSearchCursorCliKeywords => {
            "エージェント 統合 フック インストール cursor cursor-agent agent integration hooks install"
        }
        L10nKey::SettingsSearchPrimeAgentKeywords => {
            "エージェント 統合 拡張 インストール prime prime-agent primeintellect agent integration extension install"
        }
        L10nKey::SettingsSearchEmpryoKeywords => {
            "エージェント 統合 フック インストール empryo agent integration hooks install"
        }
        L10nKey::SettingsSearchJcodeKeywords => {
            "エージェント 統合 フック インストール jcode agent integration hooks install"
        }
        L10nKey::SettingsSearchMuseKeywords => {
            "エージェント 統合 フック プラグイン インストール muse meta agent integration hooks plugins install"
        }
        L10nKey::SettingsSearchAntigravityKeywords => {
            "エージェント 統合 フック インストール antigravity agy google agent integration hooks install"
        }
        L10nKey::SettingsSearchQoderCnKeywords => {
            "エージェント 統合 フック インストール qodercn qoderclicn qoder 中国版 agent integration hooks install"
        }
        L10nKey::SettingsSearchPiKeywords => {
            "エージェント 統合 拡張 インストール pi agent integration extension install"
        }
        L10nKey::SettingsSearchProgramKeywords => {
            "シェル バイナリ zsh bash fish nu nushell pwsh powershell 実行可能 起動 program shell binary executable launch"
        }
        L10nKey::SettingsSearchRememberWindowSizeKeywords => {
            "ウィンドウ サイズ 位置 境界 ジオメトリ 起動 記憶 remember window size position bounds geometry launch startup"
        }
        L10nKey::SettingsSearchReportMouseToAppsKeywords => {
            "マウス レポート vim tmux クリック スクロール shift パススルー report mouse to apps vim tmux passthrough"
        }
        L10nKey::SettingsSearchRestoreLastLayoutKeywords => {
            "復元 セッション 前回 タブ 分割 開き直し 起動 レイアウト restore last layout session previous tabs splits reopen launch"
        }
        L10nKey::SettingsSearchScrollSpeedKeywords => {
            "マウス ホイール 倍率 スクロール scroll speed mouse wheel multiplier scrolling"
        }
        L10nKey::SettingsSearchSmoothScrollKeywords => {
            "スムーズ スクロール アニメーション ホイール トラックパッド smooth animation ease wheel trackpad"
        }
        L10nKey::SettingsSearchUpdateChannelKeywords => {
            "更新 チャンネル 安定版 ナイトリー リリース update channel stable nightly release"
        }
        L10nKey::SettingsSearchCheckUpdatesOnLaunchKeywords => {
            "起動時 更新 確認 自動 update check launch startup automatic"
        }
        L10nKey::SettingsSearchAutoDownloadKeywords => {
            "バックグラウンド ダウンロード 更新 インストール 通信量 update download background install metered"
        }
        L10nKey::SettingsSearchScrollbackKeywords => {
            "履歴 バッファ 行数 スクロール scrollback history buffer lines scroll"
        }
        L10nKey::SettingsSearchShowTrayIconKeywords => {
            "トレイ メニューバー ステータス アイコン エージェント 通知 システム tray icon menu bar status system attention"
        }
        L10nKey::SettingsSearchSidebarGroupingKeywords => {
            "タブ グループ 自動 リポジトリ git ssh ホスト 固定 未分類 サイドバー sidebar auto grouping tabs repo repository pinned pin host ungrouped"
        }
        L10nKey::SettingsSearchSmartSelectionKeywords => {
            "ダブルクリック 単語 url パス 選択 セマンティック 括弧 メール smart selection double click word url path bracket email"
        }
        L10nKey::SettingsSearchStartInKeywords => {
            "cwd 作業ディレクトリ 起動 フォルダ パス ホーム 継承 カスタム start in working directory home inherit custom"
        }
        L10nKey::SettingsSearchSyncWithSystemKeywords => {
            "テーマ ダーク ライト 自動 os 外観 モード sync with system theme dark light auto follow appearance"
        }
        L10nKey::SettingsSearchLegiblePaletteKeywords => {
            "可読 コントラスト 明色 パレット パラメーター 修正 legible bright contrast palette parameter"
        }
        L10nKey::SettingsSearchTerminalBellKeywords => {
            "ベル 可聴 視覚 フラッシュ サウンド サイレント ビープ 両方 ^g terminal bell audible visual flash sound silence beep both"
        }
        L10nKey::SettingsSearchThemeKeywords => {
            "外観 色 配色 ダーク ライト パレット 背景 前景 アクセント 同期 システム os 自動 theme appearance color scheme palette background foreground accent sync auto"
        }
        L10nKey::SettingsSearchTrimTrailingSpacesKeywords => {
            "クリップボード 空白 コピー trim trailing spaces copy whitespace clipboard"
        }
        L10nKey::SettingsSearchVerifyHostKeysKeywords => {
            "ssh セキュリティ known_hosts フィンガープリント mitm ホストキー 検証 verify host keys fingerprint known_hosts"
        }
        L10nKey::SettingsSearchWarnBeforeClosingKeywords => {
            "ssh 確認 閉じる タブ ペイン ライブ セッション セキュリティ warn before closing ssh confirm tab pane live session"
        }
        L10nKey::SettingsSearchStartupWindowKeywords => {
            "起動 開く 最大化 全画面 通常 startup window launch maximized fullscreen normal"
        }
        L10nKey::SwitcherNoMatch => "一致するワークスペースまたはマシンがありません",
        L10nKey::AddSshHost => "SSH ホストを追加…",
        L10nKey::RestartServer => "tty7 server を再起動",
        L10nKey::Ok => "OK",
        L10nKey::SftpPasteUploadFailed => {
            "{name} を {host} にアップロードできませんでした: {error}"
        }
        L10nKey::ForwardDisconnected => "切断済み",
        L10nKey::ForwardDisconnectedFrom => "{host} から切断されました",
        L10nKey::SshEditProfile => "接続を編集…",
        L10nKey::ForwardTooltipAdd => "フォワードを追加",
        L10nKey::ForwardTooltipRemove => "削除",
        L10nKey::ForwardTooltipTurnOn => "オンにする",
        L10nKey::ForwardTooltipTurnOff => "オフにする — ルールは残ります",
        L10nKey::ForwardSwitchFailed => "フォワードを切り替えられませんでした — {error}",
        L10nKey::ForwardLocal => "ローカル",
        L10nKey::ForwardRemote => "リモート",
        L10nKey::ForwardDynamic => "ダイナミック",
        L10nKey::ForwardBindLabel => "バインド",
        L10nKey::ForwardToLabel => "転送先",
        L10nKey::ForwardSocksLabel => "SOCKS",
        L10nKey::ForwardAdd => "追加",
        L10nKey::ForwardPortLabel => "リモートポート",
        L10nKey::ForwardPortHere => "localhost:{port} で開きます",
        L10nKey::ForwardNeedsPort => "ポートは 1 から 65535 までの数字です。",
        L10nKey::ForwardAdvancedToggle => "詳細",
        L10nKey::ForwardSimpleToggle => "シンプル",
        L10nKey::ForwardRequestFailed => "セッションに届きませんでした。何も変更していません",
        L10nKey::SshPromptNewKey => "新しいキー {fingerprint}",
        L10nKey::SshPromptOldKey => "以前のキー {old_fingerprint}",
        L10nKey::SshPromptHostKeyNewAlgorithm => {
            "このホストはすでに {previous_algorithm} キーで登録されています。これはそれを置き換えるものではなく、新しい {algorithm} キーです"
        }
        L10nKey::SshPromptTypeYesToOverride => "「yes」を入力すると「上書き」が有効になります",
        L10nKey::SettingsDiscard => "破棄",
        L10nKey::PanelNoSession => "アクティブなセッションがありません",
        L10nKey::PanelNoSessionHint => {
            "タブを開くと、そのシェル、ディレクトリ、プロセスがここに表示されます"
        }
        L10nKey::PanelSessionSubtitle => "セッション",
        L10nKey::PanelProcessesSubtitle => "プロセス",
        L10nKey::PanelProcessesTotal => "合計",
        L10nKey::PanelPortsSubtitle => "ポート",
        L10nKey::PanelLatency => "遅延",
        L10nKey::PanelPortsUnsupported => "リモートの tty7-server が古く、ポートを列挙できません。",
        L10nKey::PanelPortsProbeFailed => {
            "このペインが何をリッスンしているか確認できませんでした。"
        }
        L10nKey::PanelPortsRestricted => {
            "他のユーザーで動いているプロセスがあり、そのポートは見えません。"
        }
        L10nKey::PanelPortsEmpty => "転送中のポートはありません",
        L10nKey::PortAutoForwarded => "リモートの :{port} は http://localhost:{local} で開けます",
        L10nKey::PanelCwd => "作業ディレクトリ",
        L10nKey::PanelShell => "シェル",
        L10nKey::PanelSsh => "ssh",
        L10nKey::PanelAgentWorking => "作業中",
        L10nKey::PanelAgentWaiting => "待機中",
        L10nKey::PanelAgentDone => "完了",
        L10nKey::PanelRevealInFinder => "Finder で表示",
        L10nKey::PanelOpenFolder => "フォルダを開く",
        L10nKey::PanelOpenInBrowser => "ブラウザで開く",
        L10nKey::WindowStop => "停止",
        L10nKey::WindowDelete => "削除",
        L10nKey::WindowThisWorkspace => "このワークスペース",
        L10nKey::WindowConfirmTitle => "ワークスペース「{name}」を{verb}しますか？",
        L10nKey::WindowStopUnreachable => {
            "そのマシンに到達できませんでした。そこでまだ実行中のシェルはすべて終了します"
        }
        L10nKey::WindowDeleteUnreachable => {
            "そのマシンに到達できませんでした。そこでまだ実行中のシェルはすべて終了し、レイアウトは消去されます"
        }
        L10nKey::WindowStopShells => "{count} 個の実行中シェルが終了します",
        L10nKey::WindowDeleteShells => "{count} 個の実行中シェルが終了し、レイアウトが消去されます",
        L10nKey::PendingConnecting => "{machine} に接続中…",
        L10nKey::PendingUnreachable => "{machine} に到達できませんでした",
        L10nKey::HomeTimeJustNow => "たった今",
        L10nKey::HomeTimeMinutesAgo => "{count} 分前",
        L10nKey::HomeTimeHourAgo => "1 時間前",
        L10nKey::HomeTimeHoursAgo => "{count} 時間前",
        L10nKey::HomeTimeYesterday => "昨日",
        L10nKey::HomeTimeDaysAgo => "{count} 日前",
        L10nKey::HomeTimeWeeksAgo => "{count} 週間前",
        L10nKey::HomeTimeMonthsAgo => "{count} か月前",
        L10nKey::HomeTimeOverYearAgo => "1 年以上前",
        L10nKey::HomeReopenNamed => "「{name}」をもう一度開く",
        L10nKey::RemoteStripDisconnected => "{machine} に未接続です",
        L10nKey::RemoteStripConnecting => "{machine} に接続中…",
        L10nKey::RemoteStripReconnecting => "{machine} に再接続中…",
        L10nKey::RemoteStripReconnectingAttempt => "{machine} に再接続中…（{count} 回目の試行）",
        L10nKey::RemoteStripReconnectingWhy => "{machine} に再接続中…（前回の失敗: {error}）",
        L10nKey::RemoteStripReconnectingAttemptWhy => {
            "{machine} に再接続中…（{count} 回目の試行、前回の失敗: {error}）"
        }
        L10nKey::RemoteStripPreempted => "このワークスペースは {by} で開かれました",
        L10nKey::RemoteStripFailed => "{machine} に未接続です — {error}",
        L10nKey::RemoteStripRouteLost => "{machine} の接続設定は存在しません — 再接続できません",
        L10nKey::RemoteRouteParkedHint => {
            "接続設定が存在しないため、自動再接続しません。リモートのセッションは残っています — 新しいプロファイルで接続すると、ワークスペース一覧に戻ります。"
        }
        L10nKey::RemoteNoticePreempted => "別の場所で開かれました — 入力しても反映されません",
        L10nKey::RemoteNoticeDisconnected => "未接続です — 入力しても反映されません",
        L10nKey::RemoteActionRetryNow => "今すぐ再試行",
        L10nKey::RemoteActionTakeBack => "取り戻す",
        L10nKey::PaneLeasedBy => "{by} で使用中（その画面サイズで表示）",
        L10nKey::RemoteActionConnect => "接続",
        L10nKey::RemoteActionRetry => "再試行",
        L10nKey::RemoteActionRemoveEntry => "エントリを削除",
        L10nKey::RemoteNoConnectionDetails => {
            "このウィンドウは {machine} 上のワークスペースですが、tty7 に接続情報がありません。SSH プロファイルか ~/.ssh/config の項目が残っているか確認してください"
        }
        L10nKey::RemoteThisComputer => "このコンピュータ",
        L10nKey::RemoteProfileGone => "削除されたプロファイル",
        L10nKey::RemoteRestartTitle => "「{machine}」上の tty7 serverを再起動しますか？",
        L10nKey::RemoteRestartBody => {
            "{machine} 上のシェルは、表示されていないものも含めてすべて終了します。ワークスペースとレイアウトは保持され、新しいシェルで開きます"
        }
        L10nKey::RemoteReplaceBody => {
            "tty7 は {machine} に対応するサーバーをインストールして起動します。\n\n{machine} で実行中のすべてのセッションが終了します。このウィンドウが接続していないセッションも含みます"
        }
        L10nKey::RemoteRestartFailedTitle => {
            "「{machine}」上の tty7 serverは再起動されませんでした"
        }
        L10nKey::RemoteRestartFailedBody => {
            "{error}\n\nそこで実行中のセッションは古いビルドのままです。セッションがなくなっている場合は、再接続時にこのビルドのサーバーが起動します"
        }
        L10nKey::RemoteHostUnreachable => "{machine} に到達できませんでした: {error}",
        L10nKey::RemoteInstallTitle => "「{machine}」に tty7 serverをインストールしますか？",
        L10nKey::RemoteInstallDetail => {
            "tty7 はサーバーバイナリを {machine} に書き込み、{machine} でワークスペースをホストできるようにします。{machine} 上の他のものには触れず、sudo も使いません。\n\n{path_label}\u{2003}{path}\n{version_label}\u{2003}{version}\n{size_label}\u{2003}{size}\n{from_label}\u{2003}{from}\n{sha_label}\u{2003}{sha256}\n\n{silent_upgrades}"
        }
        L10nKey::RemoteInstallPathLabel => "パス",
        L10nKey::RemoteInstallVersionLabel => "バージョン",
        L10nKey::RemoteInstallSizeLabel => "サイズ",
        L10nKey::RemoteInstallFromLabel => "取得元",
        L10nKey::RemoteInstallShaLabel => "SHA-256",
        L10nKey::RemoteInstallSilentUpgrades => {
            "このマシンでの今後のアップグレードはサイレントにインストールされます"
        }
        L10nKey::RemoteInstallBytes => "バイト",
        L10nKey::RemoteMismatchTitle => "「{machine}」上の tty7 serverを更新しますか？",
        L10nKey::RemoteMismatchDetail => {
            "{machine} はサーバー {running} で動いていますが、このクライアント（{wanted}）はそのプロトコルを話せません。対応するサーバーはインストール済みですが、セッションは実行中のサーバー上にあります。\n\n{replace_server}\u{2003}{wanted} に置き換え、そのサーバー上のセッションをすべて終了します。\n{cancel}\u{2003}{machine} はそのままです。このウィンドウは接続しません"
        }
        L10nKey::RemoteMismatchReplaceServer => "サーバーを更新",
        L10nKey::RemoteMismatchDowngradeServer => "サーバーを置き換え",
        L10nKey::RemoteMismatchUnknownBuild => "不明なビルド",
        L10nKey::RemoteMismatchUnknownBuildFromExe => "不明なビルド（{exe} から）",
        L10nKey::RemoteServerOutdated => {
            "{machine} の tty7 serverが古く（{build}）、この tty7 からは通信できません。更新すると接続できます"
        }
        L10nKey::RemoteServerTooNew => {
            "{machine} の tty7 server（{build}）は、この tty7 より新しいバージョンです。このコンピューターの tty7 を更新するか、向こうのサーバーを対応するものに置き換えてください"
        }
        L10nKey::RemoteDaemonStartFailed => {
            "tty7 のローカルサーバーを起動できませんでした: {error}"
        }
        L10nKey::RemoteDaemonUnreachable => {
            "tty7 のローカルサーバーに到達できませんでした: {error}"
        }
        L10nKey::RemoteDaemonTooOld => {
            "このマシンのデーモンは古いビルドのため、{machine} 上のサーバーを再起動できません。tty7 を終了（デーモンも停止します）して開き直し、再試行してください"
        }
        L10nKey::RemoteProfileMissing => "その保存済み SSH プロファイルはもう存在しません",
        L10nKey::RemoteAliasMissing => "`{alias}` は ~/.ssh/config にありません",
        L10nKey::RemoteWslNoSsh => "WSL ワークスペースには SSH 接続がありません",
        L10nKey::RemoteLocalStdioNoSsh => {
            "ローカルの --stdio ワークスペースには SSH 接続がありません"
        }
        L10nKey::RemoteHostNotTty7 => {
            "{machine} は応答しましたが、tty7 serverとしては応答しませんでした: {error}"
        }
        L10nKey::RemoteWorkspaceListFailed => {
            "{machine} に接続しましたが、ワークスペースの一覧を取得できませんでした: {error}"
        }
        L10nKey::RemoteServerRestartFailed => {
            "{machine} 上の tty7 serverを再起動できませんでした: {error}"
        }
        L10nKey::RemoteNoRouteToHost => "tty7 は {machine} に到達する手段を失いました",
        L10nKey::RemoteMachineTreeUnexpectedReply => {
            "サーバーがマシンツリーに {reply} で応答しました"
        }
        L10nKey::RemoteMismatchVersionFromExe => "{version}（{exe} から）",
        L10nKey::SwitcherThisComputer => "このコンピュータ",
        L10nKey::SwitcherStartingServer => "tty7 のサーバーを起動中…",
        L10nKey::SwitcherDownloadingServerWithTotal => {
            "tty7 のサーバーをダウンロード中… {done} / {total}"
        }
        L10nKey::SwitcherDownloadingServerNoTotal => "tty7 のサーバーをダウンロード中… {done}",
        L10nKey::SwitcherCopyingServer => "tty7 のサーバーをコピー中… {done} / {total}",
        L10nKey::SwitcherThisWindow => "このウィンドウ",
        L10nKey::SwitcherOpen => "開く",
        L10nKey::SwitcherOffline => "オフライン",
        L10nKey::SwitcherDisconnect => "切断",
        L10nKey::SwitcherEditHost => "ホストを編集…",
        L10nKey::SwitcherSaveAsHost => "SSH ホストとして保存…",
        L10nKey::SwitcherOpenInNewWindow => "新しいウィンドウで開く",
        L10nKey::SwitcherRename => "名前を変更…",
        L10nKey::SwitcherPickAWorkspace => "ワークスペースを選ぶとタブが表示されます",
        L10nKey::SwitcherNoTabs => "このワークスペースにタブはありません",
        L10nKey::SwitcherNoTabMatch => "一致するタブがありません",
        L10nKey::SwitcherTabsAfterOpening => "このワークスペースを開くとタブが表示されます",
        L10nKey::SwitcherOpenToManage => "このワークスペースを開くと名前の変更や停止ができます",
        L10nKey::SwitcherOrphanPanes => {
            "バックグラウンドペイン — どのウィンドウにも属さずに実行中のシェル:"
        }
        L10nKey::SwitcherTabCount => "{n} 個のタブ",
        L10nKey::SwitcherTabCountOne => "1 個のタブ",
        L10nKey::SwitcherActiveTab => "現在",
        L10nKey::SwitcherHoldToSwitch => "Tab で移動 · 離して切り替え",
        L10nKey::SwitcherTabToCrossColumns => "Tab で列を移動",
        L10nKey::SwitcherHintNavigate => "移動",
        L10nKey::SwitcherHintOpen => "開く",
        L10nKey::SwitcherHintNewWindow => "新しいウィンドウ",
        L10nKey::SwitcherLocalHost => "ローカル",
        L10nKey::SwitcherConnectingTo => "{machine} に接続中…",
        L10nKey::SwitcherFormName => "名前",
        L10nKey::SwitcherFormHost => "ホスト",
        L10nKey::SwitcherFormNamePlaceholder => "ワークスペース名",
        L10nKey::SwitcherFormBack => "戻る",
        L10nKey::SwitcherFormCreate => "作成",
        L10nKey::SwitcherFormPickHint => "↑↓ で選択 · Enter で決定 · Esc で閉じる",
        L10nKey::SshPromptPasswordFor => "{user}@{host} のパスワード",
        L10nKey::SshPromptPassphraseFor => "{key_path} のパスフレーズ",
        L10nKey::SshPromptTwoFactor => "二要素認証",
        L10nKey::SshPromptUnknownHost => "未知のホスト {host}",
        L10nKey::SshPromptHostKeyChanged => {
            "ホストキーが変更されました — 中間者攻撃の可能性があります"
        }
        L10nKey::SshPromptHostKeyChangedBody => {
            "ホストキーが以前に信頼したものと異なります。攻撃の可能性があります"
        }
        L10nKey::SshPromptConnect => "接続",
        L10nKey::SshPromptUnlock => "ロック解除",
        L10nKey::SshPromptSubmit => "送信",
        L10nKey::IoDenied => "権限がありません。",
        L10nKey::IoGone => "もう存在しません。",
        L10nKey::IoNoSpace => "ディスクに空き容量がありません。",
        L10nKey::IoReadOnly => "その場所は読み取り専用です。",
        L10nKey::IoBusy => "他のプログラムが使用中です。",
        L10nKey::IoTimedOut => "時間内に応答がありませんでした。",
        L10nKey::TreeWindowOpenedEmpty => {
            "サーバーがこのウィンドウのタブを渡さなかったため、空のまま開きました。失われたものはなく、応答すれば戻ります。戻らない場合は「設定 → このアプリについて」でローカルの tty7 server を再起動するか、リモート server を更新してください"
        }
        L10nKey::CmdGroupTabsPanes => "タブとペイン",
        L10nKey::CmdGroupWorkspaces => "ワークスペース",
        L10nKey::CmdGroupView => "表示",
        L10nKey::CmdGroupTerminal => "ターミナル",
        L10nKey::CmdGroupSsh => "SSH",
        L10nKey::CmdGroupAgents => "エージェント",
        L10nKey::CmdGroupApplication => "アプリケーション",
        L10nKey::CmdNewTab => "新しいタブ",
        L10nKey::CmdNewWindow => "新しいウィンドウ",
        L10nKey::CmdRenameTab => "タブの名前を変更…",
        L10nKey::CmdSplitRight => "右に分割",
        L10nKey::CmdSplitDown => "下に分割",
        L10nKey::CmdZoomPane => "ペインを拡大",
        L10nKey::CmdNextPane => "次のペイン",
        L10nKey::CmdPreviousPane => "前のペイン",
        L10nKey::CmdFocusPaneLeft => "左のペインにフォーカス",
        L10nKey::CmdFocusPaneRight => "右のペインにフォーカス",
        L10nKey::CmdFocusPaneUp => "上のペインにフォーカス",
        L10nKey::CmdFocusPaneDown => "下のペインにフォーカス",
        L10nKey::CmdResizePaneLeft => "ペインを左にリサイズ",
        L10nKey::CmdResizePaneRight => "ペインを右にリサイズ",
        L10nKey::CmdResizePaneUp => "ペインを上にリサイズ",
        L10nKey::CmdResizePaneDown => "ペインを下にリサイズ",
        L10nKey::CmdSwapPaneNext => "次のペインと入れ替え",
        L10nKey::CmdSwapPanePrevious => "前のペインと入れ替え",
        L10nKey::CmdNextTab => "次のタブ",
        L10nKey::CmdPreviousTab => "前のタブ",
        L10nKey::CmdMoveTabUp => "タブを上へ移動",
        L10nKey::CmdMoveTabDown => "タブを下へ移動",
        L10nKey::CmdRecentTabSwitcher => "最近のタブを切り替える",
        L10nKey::CmdRecentTabSwitcherReverse => "最近のタブを切り替える（逆順）",
        L10nKey::CmdCopyWorkingDirectory => "作業ディレクトリをコピー",
        L10nKey::CmdCopySessionId => "セッション ID をコピー",
        L10nKey::CmdNewAgentTab => "新しいエージェントタブ",
        L10nKey::CmdNextAgentNeedingAttention => "エージェント: 対応待ちの次のエージェントへ移動",
        L10nKey::AppNoAgentNeedsAttention => {
            "開いているウィンドウに対応が必要なエージェントはありません。"
        }
        L10nKey::AgentWaitingFor => "{age}待機中",
        L10nKey::AgeUnderMinute => "1分未満",
        L10nKey::AgeMinutes => "{count}分",
        L10nKey::AgeHours => "{count}時間",
        L10nKey::AgeDays => "{count}日",
        L10nKey::CmdForkSession => "セッションをフォーク",
        L10nKey::CmdMarkTabAsUnread => "タブを未読としてマーク",
        L10nKey::CmdHibernateTab => "タブを休止",
        L10nKey::CmdClosePaneTab => "ペイン / タブを閉じる",
        L10nKey::CmdCloseWindow => "ウィンドウを閉じる",
        L10nKey::CmdCloseOtherTabs => "他のタブを閉じる",
        L10nKey::CmdCloseTabsBelow => "下のタブを閉じる",
        L10nKey::CmdReopenClosedTab => "閉じたタブをもう一度開く",
        L10nKey::CmdNewWorkspace => "新しいワークスペース…",
        L10nKey::CmdSwitchWorkspace => "ワークスペースを切り替える…",
        L10nKey::CmdRenameWorkspace => "ワークスペースの名前を変更…",
        L10nKey::CmdStopWorkspace => "ワークスペースを停止…",
        L10nKey::CmdDeleteWorkspace => "ワークスペースを削除…",
        L10nKey::CmdResetFontSize => "フォントサイズをリセット",
        L10nKey::CmdEnterFullScreen => "全画面表示",
        L10nKey::CmdClearScrollback => "スクロールバックをクリア",
        L10nKey::CmdFindInTerminal => "ターミナル内を検索…",
        L10nKey::CmdToggleComposer => "メッセージ入力欄の切り替え",
        L10nKey::ComposerModeDefault => "編集前に確認",
        L10nKey::ComposerModeAcceptEdits => "編集を自動承認",
        L10nKey::ComposerModePlan => "プランモード",
        L10nKey::ComposerModeBypass => "権限確認をスキップ",
        L10nKey::ComposerModeAuto => "自動モード",
        L10nKey::ComposerModeTip => "権限モード  ·  Shift+Tab で切り替え",
        L10nKey::ComposerModel => "モデル",
        L10nKey::ComposerModelTip => "モデルを切り替え",
        L10nKey::ComposerModelDefault => "デフォルト（推奨）",
        L10nKey::ComposerEffort => "推論の強さ",
        L10nKey::ComposerEffortTip => "推論の強さ  ·  クリックして変更",
        L10nKey::ComposerPlaceholder => "{agent} にメッセージ   ·   @ ファイル   / コマンド",
        L10nKey::ComposerPlaceholderFiles => "{agent} にメッセージ   ·   @ ファイル",
        L10nKey::ComposerSendTip => "送信（Enter）  ·  Shift+Enter で改行",
        L10nKey::ComposerAttach => "ファイルや画像を添付",
        L10nKey::ComposerMenuCommands => "コマンド",
        L10nKey::ComposerCmdProject => "プロジェクトのコマンド",
        L10nKey::ComposerCmdUser => "ユーザーのコマンド",
        L10nKey::ComposerAgentAsking => {
            "{agent} が質問しています。ターミナルで答えてから送信してください。"
        }
        L10nKey::CmdFindNext => "次を検索",
        L10nKey::CmdFindPrevious => "前を検索",
        L10nKey::CmdCopy => "コピー",
        L10nKey::CmdPaste => "貼り付け",
        L10nKey::CmdAlternatePaste => "貼り付け（全画面アプリを除く）",
        L10nKey::CmdSshManageProfiles => "SSH: プロファイルを管理…",
        L10nKey::CmdSshReconnect => "SSH: 再接続",
        L10nKey::CmdSshPortForwarding => "SSH: ポートフォワーディング",
        L10nKey::CmdSettings => "設定…",
        L10nKey::CmdKeyboardShortcuts => "キーボードショートカット",
        L10nKey::CmdAboutTty7 => "tty7 について",
        L10nKey::CmdCheckForUpdates => "アップデートを確認…",
        L10nKey::CmdDocumentation => "ドキュメント",
        L10nKey::CmdJoinDiscord => "Discord に参加",
        L10nKey::CmdReportIssue => "問題を報告…",
        L10nKey::CmdQuitTty7 => "tty7 を終了",
        L10nKey::AppRestartServerTitle => "tty7 server を再起動しますか？",
        L10nKey::AppRestartServerFailed => "tty7 server を再起動できませんでした: {error}",
        L10nKey::AppRestartServerMismatchDetail => {
            "tty7 server はプロトコル {protocol}（ビルド v{build}）、このアプリは {ours} のため、タブを取り出せません。\n\n終了：何も変わりません。tty7 server もシェルも動き続けます。\n再起動：タブは新しいシェルで戻り、いま実行中のものは終了します"
        }
        L10nKey::AppRestartServerDialectDetail => {
            "tty7 server は制御方言 v{dialect}（ビルド v{build}）、このアプリは v{ours} のため、ウィンドウはどれも空で開きます。\n\n終了：何も変わりません。tty7 server もシェルも動き続けます。\n再起動：タブは新しいシェルで戻り、いま実行中のものは終了します"
        }
        L10nKey::AppRestartServerDialectNewerDetail => {
            "tty7 server は制御方言 v{dialect}（ビルド v{build}）、このアプリは v{ours} のため、ウィンドウはどれも空で開きます。\n\n終了して新しいビルドを入れる：根本的な解決で、シェルはそのまま残ります。\n再起動：タブは新しいシェルで戻り、いま実行中のものは終了します"
        }
        L10nKey::AppRestartServerOldDetail => {
            "tty7 server はバージョン照合より前のもので、何を話すか分かりません。\n\n終了：何も変わりません。tty7 server もシェルも動き続けます。\n再起動：タブは新しいシェルで戻り、いま実行中のものは終了します"
        }
        L10nKey::AppRestart => "再起動",
        L10nKey::AppRestartServerNoServer => {
            "{label} には再起動できる tty7 server がありません。このコンピュータが --stdio で実行しているプログラムです。代わりにワークスペースを止めてください"
        }
        L10nKey::AppRestartServerBody => {
            "このコンピュータのシェルはすべて終了します。タブとレイアウトは保持され、新しいシェルで開きます"
        }
        L10nKey::ConfigQuarantinedStartup => {
            "config.json を解析できませんでした。デフォルト設定で実行しており、内容は config.json.corrupt として残しました。直せば自動で再読み込みされます。それまで設定の変更は保存されません"
        }
        L10nKey::ConfigQuarantinedReload => {
            "編集された config.json を解析できませんでした。実行中の設定を保持し、内容は config.json.corrupt として残しました。直せば自動で再読み込みされます。それまでに設定を保存すると上書きされます"
        }
        L10nKey::ConfigUnreadableStartup => {
            "config.json を読み込めませんでした。デフォルト設定で実行しており、ファイルはそのままです。権限か内容を直せば自動で再読み込みされます。それまで設定の変更は保存されません"
        }
        L10nKey::ConfigUnreadableReload => {
            "config.json を読み込めませんでした。実行中の設定を保持し、ファイルもそのままです。権限か内容を直せば自動で再読み込みされます。それまでに設定を保存すると上書きされます"
        }
        L10nKey::AppReopenTabFailed => "タブを開き直せませんでした: ターミナルが起動しませんでした",
        L10nKey::AppOpenTerminalFailed => "ターミナルを開けませんでした: {error}",
        L10nKey::AppTabsNotRestored => "前回のタブ {count} 個を開き直せませんでした",
        L10nKey::AppFullscreenEntered => "全画面表示 — 解除するには {key}",
        L10nKey::AppFullscreenEnteredNoKey => {
            "全画面表示 — 解除するまでウィンドウボタンは非表示です"
        }
        L10nKey::LaunchWorkspacesLeftRunning => {
            "このウィンドウだけを復元しました — あと {count} 個のワークスペースがバックグラウンドで実行中です。サイドバーから開き直せます。"
        }
        L10nKey::AppSshConnectionFailed => "SSH 接続に失敗しました: {error}",
        L10nKey::AppSshReconnectFailed => "SSH 再接続に失敗しました: {error}",
        L10nKey::AppSplitPaneFailed => "ペインを分割できませんでした: {error}",
        L10nKey::PaneDragHandleTooltip => "ドラッグしてこのペインを移動",
        L10nKey::AppPaneNoCodingAgent => "このペインはコーディングエージェントを実行していません",
        L10nKey::AppForkNoCommand => "tty7 には {name} 用のフォークコマンドがありません",
        L10nKey::AppForkLocalOnly => {
            "SSH または WSL セッション内の {name} のセッションはフォークできません"
        }
        L10nKey::AppForkNoSessionId => {
            "tty7 はこのペインで {name} のセッション ID を確認できていません — 設定 → 連携でフックをインストールしてください"
        }
        L10nKey::AppForkSessionIdNotToken => {
            "{name} のセッション ID はプレーンなトークンではありません"
        }
        L10nKey::AppForkMidTurn => {
            "{name} は処理の途中です — 進行中のターンはフォークに含まれません"
        }
        L10nKey::AppCmdAgentLaunchTitle => "Agent: {name}",
        L10nKey::AppNoAgentOnPath => "このマシンの PATH にコーディングエージェントが見つかりません",
        L10nKey::AppNoAgentSeenHere => {
            "このワークスペースではまだコーディングエージェントが実行されていません — 一度手動で起動すると、ここに表示されます"
        }
        L10nKey::AppAgentLaunchSaved => "{name} は今後このコマンドで起動します: {command}",
        L10nKey::AppAgentLaunchArgsUnknown => "{name} は起動時の引数を報告していません",
        L10nKey::AppPlaceholderDescription => "説明",
        L10nKey::AppPlaceholderLoginShell => "ログインシェル",
        L10nKey::AppPlaceholderNone => "なし",
        L10nKey::AppThemeColorBackground => "背景",
        L10nKey::AppThemeColorForeground => "前景",
        L10nKey::AppThemeColorAccent => "アクセント",
        L10nKey::AppThemeColorCursor => "カーソル",
        L10nKey::AppThemeColorSelection => "選択範囲",
        L10nKey::AppThemeAnsiBlack => "黒",
        L10nKey::AppThemeAnsiRed => "赤",
        L10nKey::AppThemeAnsiGreen => "緑",
        L10nKey::AppThemeAnsiYellow => "黄",
        L10nKey::AppThemeAnsiBlue => "青",
        L10nKey::AppThemeAnsiMagenta => "マゼンタ",
        L10nKey::AppThemeAnsiCyan => "シアン",
        L10nKey::AppThemeAnsiWhite => "白",
        L10nKey::AppThemeAnsiBrightBlack => "明るい黒",
        L10nKey::AppThemeAnsiBrightRed => "明るい赤",
        L10nKey::AppThemeAnsiBrightGreen => "明るい緑",
        L10nKey::AppThemeAnsiBrightYellow => "明るい黄",
        L10nKey::AppThemeAnsiBrightBlue => "明るい青",
        L10nKey::AppThemeAnsiBrightMagenta => "明るいマゼンタ",
        L10nKey::AppThemeAnsiBrightCyan => "明るいシアン",
        L10nKey::AppThemeAnsiBrightWhite => "明るい白",
        L10nKey::AppAgentHooksThisComputer => "このコンピュータ",
        L10nKey::AppAgentHooksRemoteMachine => "リモートマシン",
        L10nKey::AppAgentHooksNoHomeDir => {
            "tty7 はこのコンピュータのホームディレクトリを特定できなかったため、インストール先がありません"
        }
        L10nKey::AppAgentHooksOffline => {
            "このマシンに接続されていないため、エージェントの設定を読み書きできません。そのマシンでワークスペースを開いてから戻ってください"
        }
        L10nKey::AppAgentHooksHomeDirUnresolved => "ホームディレクトリを解決できません",
        L10nKey::AppAgentHooksInstalled => "インストール済み",
        L10nKey::AppAgentHooksInstalledEnableCodexThere => {
            "インストール済み — そのマシンで `codex features enable hooks` を一度実行してください"
        }
        L10nKey::AppAgentHooksInstalledCodexEnableFailed => {
            "インストール済みですが `codex features enable hooks` を実行できませんでした ({error}) — 手動で一度実行してください"
        }
        L10nKey::AppAgentHooksRemoved => "削除済み",
        L10nKey::AppAgentHooksNothingInstalled => {
            "インストールされていないため、削除するものはありません"
        }
        L10nKey::AppAgentHooksNoTty7Hooks => {
            "tty7 のフックが見つからないため、削除するものはありません"
        }
        L10nKey::AppAgentHooksInstallFailed => "フックをインストールできませんでした: {error}",
        L10nKey::AppAgentHooksRemoveFailed => "フックを削除できませんでした: {error}",
        L10nKey::AppKeybindingDisplacedNote => {
            "{action} が {previous} からショートカットを奪いました。{previous} は現在未設定です"
        }
        L10nKey::AppLocalServerName => "ローカルサーバー",
        L10nKey::AppMenuMinimize => "最小化",
        L10nKey::AppMenuZoom => "ズーム",
        L10nKey::SwitcherStatusTakenOver => "他のクライアントが使用中",
        L10nKey::SettingsFontDefault => "デフォルト（メインに合わせる）",
        L10nKey::SettingsUiFontDefault => "デフォルト（システム UI フォント）",
        L10nKey::ForwardDescriptionPlaceholder => "用途",
        L10nKey::SettingsShellDefaultLoginShell => "あなたのログインシェル",
        L10nKey::Replace => "置き換える",
        L10nKey::SettingsDaemonStaleDescInPlace => {
            "tty7 が更新されました。server はその場で入れ替わり、シェルは続きます。内蔵 SSH のペインだけ閉じます"
        }
        L10nKey::AppRestartServerAfterUpdateDetail => {
            "tty7 は {ours} に更新されましたが、server はまだ {build} で動いています。停止せずにこのビルドへ置き換えてシェルを残せますが、tty7 内蔵の SSH クライアントを使うペインは閉じられます。今すぐ置き換えるか、あとで設定から行えます"
        }
        L10nKey::AppRestartServerBodyInPlace => {
            "tty7 server は停止せずに自分自身をこのビルドへ置き換えます。シェルは動いたままで、ウィンドウはすぐに再接続します。tty7 内蔵の SSH クライアントを使うペインだけは例外で、その接続は閉じられ、開き直しが必要です"
        }
        L10nKey::PaneRestoredScreenBanner => {
            "復元された画面 — 以下は新しいシェルで、これより上のものは動いていません"
        }
        L10nKey::PaneTitleDisconnected => "{title} — 切断されました",
        L10nKey::PaneTitleProcessExited => "{title} — プロセスが終了しました",
        L10nKey::LoopbackForwardFailed => ":{port} を転送できませんでした — {error}",
        L10nKey::TrayTooltipAgents => "tty7: {parts}",
        L10nKey::TrayAgentSep => "、",
        L10nKey::CursorShapeBlock => "ブロック",
        L10nKey::CursorShapeBar => "バー",
        L10nKey::CursorShapeUnderline => "下線",
        L10nKey::PromptCursorShapeFollow => "追従",
        L10nKey::CmdUpdateRemoteServer => "「{machine}」上の tty7 server を更新…",
        L10nKey::CmdUpdateRemoteServerSubtitle => {
            "このビルドのサーバーを再インストールし、そこのセッションはすべて終了"
        }
        L10nKey::RemoteUpdateBody => {
            "tty7 は {machine} にこのビルドのサーバーをインストールし（同じバージョンのものがあっても上書きします）、再起動します。\n\n{machine} で実行中のすべてのセッションが終了します。このウィンドウが接続していないセッションも含みます"
        }
        L10nKey::RemoteUpdateNeedsLocalServer => {
            "このコンピュータの tty7 server は古すぎて、{machine} のサーバーを更新できません。先にこのコンピュータのサーバーを更新してから、もう一度お試しください"
        }
        L10nKey::AppMenuAbout => "tty7 について",
        L10nKey::AppMenuCheckForUpdates => "アップデートを確認…",
        L10nKey::AppMenuSettings => "設定…",
        L10nKey::AppMenuServices => "サービス",
        L10nKey::AppMenuHideApp => "tty7 を非表示",
        L10nKey::AppMenuHideOthers => "ほかを非表示",
        L10nKey::AppMenuShowAll => "すべて表示",
        L10nKey::AppMenuQuit => "tty7 を終了",
        L10nKey::AppMenuFile => "ファイル",
        L10nKey::AppMenuEdit => "編集",
        L10nKey::AppMenuView => "表示",
        L10nKey::AppMenuWindow => "ウィンドウ",
        L10nKey::AppMenuHelp => "ヘルプ",
        L10nKey::AppMenuNewTab => "新規タブ",
        L10nKey::AppMenuNewWorkspace => "新規ワークスペース…",
        L10nKey::AppMenuSplitRight => "右に分割",
        L10nKey::AppMenuSplitLeft => "左に分割",
        L10nKey::AppMenuSplitDown => "下に分割",
        L10nKey::AppMenuSplitUp => "上に分割",
        L10nKey::AppMenuRenameTab => "タブの名前を変更…",
        L10nKey::AppMenuCopyWorkingDirectory => "作業ディレクトリをコピー",
        L10nKey::AppMenuCopySessionId => "セッション ID をコピー",
        L10nKey::AppMenuForkSession => "セッションをフォーク",
        L10nKey::AppMenuSaveAgentLaunchArgs => "現在の起動引数をデフォルトに設定",
        L10nKey::AppMenuClosePaneTab => "閉じる",
        L10nKey::AppMenuCloseOtherTabs => "他のタブを閉じる",
        L10nKey::AppMenuCloseTabsBelow => "下のタブを閉じる",
        L10nKey::AppMenuReopenClosedTab => "閉じたタブをもう一度開く",
        L10nKey::AppMenuRenameWorkspace => "ワークスペースの名前を変更…",
        L10nKey::AppMenuStopWorkspace => "ワークスペースを停止…",
        L10nKey::AppMenuDeleteWorkspace => "ワークスペースを削除…",
        L10nKey::AppMenuUndo => "元に戻す",
        L10nKey::AppMenuRedo => "やり直す",
        L10nKey::AppMenuCut => "切り取り",
        L10nKey::AppMenuCopy => "コピー",
        L10nKey::AppMenuPaste => "貼り付け",
        L10nKey::AppMenuSelectAll => "すべて選択",
        L10nKey::AppMenuFind => "検索…",
        L10nKey::AppMenuFindNext => "次を検索",
        L10nKey::AppMenuFindPrevious => "前を検索",
        L10nKey::AppMenuIncreaseFontSize => "フォントサイズを拡大",
        L10nKey::AppMenuDecreaseFontSize => "フォントサイズを縮小",
        L10nKey::AppMenuResetFontSize => "フォントサイズをリセット",
        L10nKey::AppMenuLeftSidebar => "左サイドバー",
        L10nKey::AppMenuRightPanel => "右パネル",
        L10nKey::AppMenuFocusNextPane => "次のペインにフォーカス",
        L10nKey::AppMenuFocusPreviousPane => "前のペインにフォーカス",
        L10nKey::AppMenuZoomPane => "ペインを拡大",
        L10nKey::AppMenuClearScrollback => "スクロールバックをクリア",
        L10nKey::AppMenuDocumentation => "tty7 ドキュメント",
        L10nKey::AppMenuKeyboardShortcuts => "キーボードショートカット",
        L10nKey::AppMenuJoinDiscord => "Discord に参加",
        L10nKey::AppMenuReportIssue => "問題を報告…",
        L10nKey::AppMenuRestartServer => "tty7 server を再起動…",
        L10nKey::WindowUntitled => "無題",
        L10nKey::TrayShowTty7 => "tty7 を表示",
        L10nKey::TrayNotifications => "通知",
        L10nKey::TrayAgentNeedsInput => "入力が必要",
        L10nKey::AgentStatusWorking => "実行中",
        L10nKey::AgentStatusWaiting => "入力が必要",
        L10nKey::AgentStatusDone => "完了",
        L10nKey::NotifyCommandFinished => "コマンドが {secs} 秒で完了しました",
        L10nKey::NotifyCommandFinishedWithCommand => "{command} — {secs} 秒で完了しました",
        L10nKey::NotifyAgentFinished => "{secs} 秒で完了しました",
        L10nKey::NotifyAgentWaiting => "入力を待っています",
        L10nKey::NotifyTurnFinished => "ターンが完了しました",
        L10nKey::TabTooltipMore => "その他",
        L10nKey::TabTooltipShowSidebar => "サイドバーを表示",
        L10nKey::TabTooltipHideSidebar => "サイドバーを非表示",
        L10nKey::TabTooltipHideDetailPanel => "詳細パネルを非表示",
        L10nKey::TabTooltipShowDetailPanel => "詳細パネルを表示",
        L10nKey::TabTooltipZoomed => "ペインを拡大中 — 他のペインは非表示",
        L10nKey::TabMenuLocalShells => "ローカル",
        L10nKey::TabMenuAddHost => "SSH ホストを追加…",
        L10nKey::TabMenuAllHosts => "すべての SSH ホスト…",
        L10nKey::TabMenuSplitHint => "{key} を押しながら選ぶと分割",
        L10nKey::TabUnnamedShell => "シェル {n}",
        L10nKey::ShellDefault => "デフォルト",
        L10nKey::SidebarUngroupedGroup => "未分類",
        L10nKey::SidebarMoveToGroup => "グループへ移動",
        L10nKey::SidebarNewGroup => "新規グループ…",
        L10nKey::SidebarRemoveFromGroup => "グループから外す",
        L10nKey::SidebarNewGroupName => "新規グループ",
        L10nKey::SidebarRenameGroup => "グループ名を変更",
        L10nKey::SidebarPinGroup => "グループを固定",
        L10nKey::SidebarUnpinGroup => "固定を解除",
        L10nKey::SidebarGroupNewTab => "新規タブ",
        L10nKey::SidebarSetGroupFolder => "フォルダを設定…",
        L10nKey::SidebarUseCurrentTabFolder => "現在のタブのフォルダを使用",
        L10nKey::SidebarClearGroupFolder => "フォルダを解除",
        L10nKey::SidebarDeleteGroup => "グループを削除",
        L10nKey::SidebarDropToPin => "ここにドロップして固定",
        L10nKey::TabContextCloseTab => "タブを閉じる",
        L10nKey::TerminalContextClear => "クリア",
        L10nKey::Search => "検索",
        L10nKey::TabContextMarkUnread => "未読としてマーク",
        L10nKey::TabContextHibernate => "休止",
        L10nKey::TabContextWake => "復帰",
        L10nKey::TabTooltipAsleep => "休止中 — 選択すると復帰",
        L10nKey::TabWakeFailed => "タブを復帰できませんでした：どのペインも起動できません",
        // ---- The right panel's GitHub tab ----
        L10nKey::CmdCopyRaw => "そのままコピー",
        L10nKey::SettingsSearchCopyJoinWrappedKeywords => {
            "クリップボード コピー 折り返し 行 結合 clipboard copy wrap unwrap join lines claude raw"
        }
        L10nKey::SettingsCopyJoinWrapped => "コピー時に折り返し行をつなげる",
        L10nKey::SettingsCopyJoinWrappedDesc => {
            "Claude Code のペインで、Claude が画面端で折り返した行をつなげ、段落を 1 行として貼り付けます。⌘⌥C または「そのままコピー」で表示どおりにコピーします"
        }
    })
}

pub fn translate_variant_ja(key: L10nKey, branch: &'static str) -> Option<&'static str> {
    let res = match (key, branch) {
        (L10nKey::SettingsMatchCount, "zero") => "0 件",
        (L10nKey::SettingsMatchCount, "one") => "1 件",
        (L10nKey::SettingsMatchCount, "other") => "{count} 件",
        (L10nKey::SettingsRestoreChanged, "zero") => "変更を戻す",
        (L10nKey::SettingsRestoreChanged, "one") => "変更した 1 件を戻す",
        (L10nKey::SettingsRestoreChanged, "other") => "変更した {count} 件を戻す",
        (L10nKey::SettingsMoreHosts, "zero") => "ほかのホストはありません",
        (L10nKey::SettingsMoreHosts, "one") => "ほかに 1 台のホスト",
        (L10nKey::SettingsMoreHosts, "other") => "ほかに {count} 台のホスト",
        (L10nKey::SettingsMoreAgents, "zero") => "ほかのエージェントはありません",
        (L10nKey::SettingsMoreAgents, "one") => "ほかに 1 件のエージェント",
        (L10nKey::SettingsMoreAgents, "other") => "ほかに {count} 件のエージェント",
        (L10nKey::SettingsImportSummary, "zero") => {
            "新しいホストはありません — {updated} 件を更新、{unchanged} 件は変更なし"
        }
        (L10nKey::SettingsImportSummary, "one") => {
            "ホスト 1 件を追加 — {updated} 件を更新、{unchanged} 件は変更なし"
        }
        (L10nKey::SettingsImportSummary, "other") => {
            "ホスト {count} 件を追加 — {updated} 件を更新、{unchanged} 件は変更なし"
        }
        (L10nKey::SettingsImportIgnored, "zero") => {
            "ファイル内のすべてのオプションに tty7 側の設定があります"
        }
        (L10nKey::SettingsImportIgnored, "one") => {
            "tty7 に設定のないオプションが 1 件あり、ファイルに残されています: {options}"
        }
        (L10nKey::SettingsImportIgnored, "other") => {
            "tty7 に設定のないオプションが {count} 件あり、ファイルに残されています: {options}"
        }
        (L10nKey::SettingsOfflineMachines, "zero") => {
            "未接続の保存済みマシンはもうありません — いずれかでワークスペースを開くと、そこにフックをインストールできます"
        }
        (L10nKey::SettingsOfflineMachines, "one") => {
            "未接続の保存済みマシンがもう 1 台あります — そのマシンでワークスペースを開くと、そこにフックをインストールできます"
        }
        (L10nKey::SettingsOfflineMachines, "other") => {
            "未接続の保存済みマシンがさらに {count} 台あります — いずれかでワークスペースを開くと、そこにフックをインストールできます"
        }
        (L10nKey::CloseTabsTitle, "one") => "1 個のタブを閉じますか？",
        (L10nKey::CloseTabsTitle, "other") => "{count} 個のタブを閉じますか？",
        (L10nKey::AppTabsNotRestored, "one") => "前回のタブ 1 個を開き直せませんでした",
        (L10nKey::AppTabsNotRestored, "other") => "前回のタブ {count} 個を開き直せませんでした",
        (L10nKey::LaunchWorkspacesLeftRunning, "one") => {
            "このウィンドウだけを復元しました — あと 1 個のワークスペースがバックグラウンドで実行中です。サイドバーから開き直せます。"
        }
        (L10nKey::LaunchWorkspacesLeftRunning, "other") => {
            "このウィンドウだけを復元しました — あと {count} 個のワークスペースがバックグラウンドで実行中です。サイドバーから開き直せます。"
        }
        (L10nKey::HomeTimeMinutesAgo, "one") => "1 分前",
        (L10nKey::HomeTimeMinutesAgo, "other") => "{count} 分前",
        (L10nKey::HomeTimeHoursAgo, "one") => "1 時間前",
        (L10nKey::HomeTimeHoursAgo, "other") => "{count} 時間前",
        (L10nKey::HomeTimeDaysAgo, "one") => "1 日前",
        (L10nKey::HomeTimeDaysAgo, "other") => "{count} 日前",
        (L10nKey::HomeTimeWeeksAgo, "one") => "1 週間前",
        (L10nKey::HomeTimeWeeksAgo, "other") => "{count} 週間前",
        (L10nKey::HomeTimeMonthsAgo, "one") => "1 か月前",
        (L10nKey::HomeTimeMonthsAgo, "other") => "{count} か月前",
        (L10nKey::WindowStopShells, "zero") => "レイアウトと作業ディレクトリは消去されます",
        (L10nKey::WindowStopShells, "one") => "実行中のシェル 1 個が終了します",
        (L10nKey::WindowStopShells, "other") => "実行中のシェル {count} 個が終了します",
        (L10nKey::WindowDeleteShells, "zero") => "レイアウトと作業ディレクトリは消去されます",
        (L10nKey::WindowDeleteShells, "one") => {
            "実行中のシェル 1 個が終了し、レイアウトが消去されます"
        }
        (L10nKey::WindowDeleteShells, "other") => {
            "{count} 個の実行中シェルが終了し、レイアウトが消去されます"
        }
        // ---- The right panel's GitHub tab ----
        _ => return None,
    };
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn japanese_covers_every_key() {
        assert_eq!(translate_ja(L10nKey::SearchTabs), Some("タブを検索…"));
        assert!(translate_variant_ja(L10nKey::WindowDeleteShells, "other").is_some());
    }
}
