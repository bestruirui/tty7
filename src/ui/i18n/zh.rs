use super::L10nKey;

pub fn translate_zh(key: L10nKey) -> Option<&'static str> {
    Some(match key {
        L10nKey::SettingsNoMatchesShort => "没有匹配项",
        L10nKey::SettingsModifiedTitle => "已修改",
        L10nKey::SettingsMatchCount => "{count} 个结果",
        L10nKey::SettingsNothingModified => "所有设置均为默认值",
        L10nKey::SettingsKeyThen => "再按",
        L10nKey::SettingsStartupRestore => "启动与恢复",
        L10nKey::SettingsSettingsFile => "设置文件",
        L10nKey::SettingsReveal => "显示",
        L10nKey::SettingsThemeModeSystem => "跟随系统",
        L10nKey::SettingsThemeModeSystemDesc => "跟随系统外观，在浅色与深色主题之间切换。",
        L10nKey::SettingsThemeModeLightDesc => "始终使用浅色主题。",
        L10nKey::SettingsThemeModeDarkDesc => "始终使用深色主题。",
        L10nKey::SettingsThemeSlotLightDesc => "系统处于浅色模式时使用。",
        L10nKey::SettingsThemeSlotDarkDesc => "系统处于深色模式时使用。",
        L10nKey::SettingsThemeSlotDesc => "终端和界面使用的主题。",
        L10nKey::SettingsLightThemeLabel => "浅色主题",
        L10nKey::SettingsDarkThemeLabel => "深色主题",
        L10nKey::SettingsNoThemesMatch => "没有匹配的主题",
        L10nKey::SettingsRestoreChanged => "恢复 {count} 项修改",
        L10nKey::SettingsShortcutsHint => {
            "点击快捷键后按下新组合键；紧接着再按一组即可组成序列。Esc 取消，⌫ 清除。"
        }
        L10nKey::SettingsShortcutsHintTmux => {
            "tmux 预设：窗格与标签页操作使用前缀键。点击快捷键即可修改；Esc 取消，⌫ 清除。"
        }
        L10nKey::SettingsShortcutConflict => "{keys} 已被“{action}”使用，替换后将从那里移除。",
        L10nKey::SettingsReplace => "替换",
        L10nKey::SettingsNoActionsMatch => "没有与“{query}”匹配的操作",
        L10nKey::SettingsSearchShortcuts => "搜索操作或按键",
        L10nKey::SettingsHostsDesc => "最近使用的主机。搜索可找到任意已保存的主机。",
        L10nKey::SettingsAddHost => "添加主机",
        L10nKey::SettingsNoHostsMatch => "没有与“{query}”匹配的主机",
        L10nKey::SettingsHostsFromFiles => "{count} 台主机，来自 {files} 个来源",
        L10nKey::SettingsMoreHosts => "还有 {count} 台主机",
        L10nKey::SettingsShowRecentOnly => "只显示最近",
        L10nKey::SettingsShowAll => "显示全部",
        L10nKey::SettingsUnsaved => "未保存",
        L10nKey::SettingsNever => "从未",
        L10nKey::SettingsDefinedIn => "定义于",
        L10nKey::SettingsConnectInNewTab => "在新标签页中连接",
        L10nKey::SettingsCopied => "已复制",
        L10nKey::SettingsNavMobile => "手机",
        L10nKey::SettingsMobileAccess => "允许手机访问",
        L10nKey::SettingsMobileAccessDesc => {
            "配对过的手机可以查看并操作这台电脑上的窗格，关掉所有窗口也可以。连接端到端加密。"
        }
        L10nKey::SettingsMobileStatusStarting => "正在启动…",
        L10nKey::SettingsMobileStatusFailed => "未运行：{error}",
        L10nKey::SettingsMobileStartFailed => "手机访问没能启动：{error}",
        L10nKey::SettingsMobileNoAnswer => {
            "tty7 server 没有启动它。请在 设置 → 关于 中重启 server 后再试。"
        }
        L10nKey::SettingsMobilePair => "配对手机",
        L10nKey::SettingsMobileShowCode => "显示配对码",
        L10nKey::SettingsMobilePairDesc => "生成一个一次性配对码，给手机上的 tty7 app 用。",
        L10nKey::SettingsMobilePairNeedsAccess => "请先打开手机访问。",
        L10nKey::SettingsMobilePairScan => {
            "用手机相机对准它，或在 tty7 app 里扫描，或者复制配对码粘贴过去。"
        }
        L10nKey::SettingsMobilePairValid => "{time} 后失效，只能配对一台手机。",
        L10nKey::SettingsMobileNewCode => "换一个配对码",
        L10nKey::SettingsMobilePairExpired => "这个配对码已过期。生成一个新的再配对。",
        L10nKey::SettingsMobilePairTried => {
            "这个配对码已被尝试过，不能再用了 —— 可能是输错了，或者连接中途断了。生成一个新的再配对。"
        }
        L10nKey::SettingsMobilePairReplaced => "已有更新的配对码替换了它。生成一个新的再配对。",
        L10nKey::SettingsMobileCopyCode => "复制配对码",
        L10nKey::SettingsMobilePaired => "已与 {name} 配对。",
        L10nKey::SettingsMobilePhones => "已配对的手机",
        L10nKey::SettingsMobileNoPhones => "还没有配对过手机。",
        L10nKey::SettingsMobileUnpair => "取消配对",
        L10nKey::SettingsSearchMobileKeywords => {
            "手机 移动 iphone android ipad 配对 二维码 远程 访问"
        }
        L10nKey::SettingsCopySshCommand => "复制 ssh 命令",
        L10nKey::SettingsStoredInTty7 => "保存在 tty7 设置中",
        L10nKey::SettingsClickAgainToRemove => "再次点击以删除",
        L10nKey::SettingsRemoveHost => "删除主机",
        L10nKey::SettingsEditHost => "编辑",
        L10nKey::SettingsLive => "已连接",
        L10nKey::SettingsPressKeysShort => "按下按键…",
        L10nKey::SettingsSearchAgents => "搜索 Agent",
        L10nKey::SettingsConnectedMachines => "已连接",
        L10nKey::SettingsInstalledCount => "已安装 {count} 个",
        L10nKey::SettingsMachine => "机器",
        L10nKey::SettingsMachineLocalDesc => "钩子按机器安装。远程机器在连接期间会出现在这里。",
        L10nKey::SettingsMachineRemoteDesc => "通过当前与 {name} 的连接安装。",
        L10nKey::SettingsAgentsInstalledSummary => "已安装 {count} / {total}",
        L10nKey::SettingsNoAgentsInstalled => "这台机器上还没有安装 Agent 钩子。",
        L10nKey::SettingsNoAgentsMatch => "没有与“{query}”匹配的 Agent",
        L10nKey::SettingsMoreAgents => "还有 {count} 个可用 Agent",
        L10nKey::SettingsShowInstalledOnly => "只显示已安装",
        L10nKey::SettingsWorking => "处理中…",
        L10nKey::SettingsUpdateAvailable => "有可用更新",
        L10nKey::SettingsRevealHookFile => "显示钩子文件",
        L10nKey::SettingsNavGeneral => "常规",
        L10nKey::SettingsEditShortcuts => "编辑快捷键…",
        L10nKey::SettingsModifiedOnly => "仅显示已修改",
        L10nKey::SettingsModified => "已修改",
        L10nKey::SettingsSearchResults => "搜索结果",
        L10nKey::SettingsOpenSetting => "打开设置",
        L10nKey::SettingsNoModified => "没有符合筛选条件的已修改设置。",
        L10nKey::SettingsTerminalFontGroup => "终端文字",
        L10nKey::SettingsInterfaceFontGroup => "界面文字",
        L10nKey::SettingsUnsavedTitle => "离开前保存更改？",
        L10nKey::SettingsUnsavedBody => "可以保存更改、放弃更改，或继续编辑。",
        L10nKey::SettingsSaveChanges => "保存更改",
        L10nKey::SettingsThemeDraft => "正在预览主题更改。保存以保留，取消以还原。",
        L10nKey::SettingsSaveError => "无法保存更改：{error}",
        L10nKey::SettingsRetrySave => "重新保存",

        L10nKey::SearchTabs => "搜索标签页…",
        L10nKey::SearchSettings => "搜索设置…",
        L10nKey::FilterHosts => "筛选主机…",
        L10nKey::SearchTheme => "搜索…",
        L10nKey::SearchWorkspacesAndMachines => "搜索工作区、标签页和机器…",
        L10nKey::SearchFind => "查找…",
        L10nKey::SearchMatchCase => "区分大小写",
        L10nKey::SearchUseRegex => "使用正则表达式",
        L10nKey::HomeNewTab => "新标签页",
        L10nKey::HomeReopenClosedTab => "重新打开已关闭的标签页",
        L10nKey::HomeSwitchWorkspace => "切换工作区…",
        L10nKey::HomeSplitRight => "向右分屏",
        L10nKey::HomeSplitDown => "向下分屏",
        L10nKey::HomeSettings => "设置…",
        L10nKey::Reconnect => "重新连接",
        L10nKey::None => "无。",
        L10nKey::TryAgain => "重试",
        L10nKey::Delete => "删除",
        L10nKey::Dismiss => "关闭",
        L10nKey::StoredPasswordRejected => "已存储的密码被拒绝，请输入新密码。",
        L10nKey::StoredPassphraseRejected => "已保存的口令无法解锁该密钥，请输入正确的口令。",
        L10nKey::Trust => "信任",
        L10nKey::Abort => "中止",
        L10nKey::HostKeyOverrideMessage => "输入 yes 覆盖并信任新密钥，或按 Esc 中止。",
        L10nKey::Override => "覆盖",
        L10nKey::RememberKeychain => "记住（钥匙串）",
        L10nKey::Cancel => "取消",
        L10nKey::Close => "关闭",
        L10nKey::QuitStopServerTitle => "退出并停止 tty7 server？",
        L10nKey::QuitStopServerBody => {
            "这会退出 tty7 并停止 tty7 server，shell 里正在跑的东西都会被终止。标签页和布局会保留，下次启动时以全新的 shell 打开。（只关窗口的话应用收进托盘，shell 继续跑。）"
        }
        L10nKey::QuitAndStop => "退出并停止",
        L10nKey::CloseSshConnectionTitle => "关闭这个 SSH 连接？",
        L10nKey::CloseSshConnectionBody => "连接仍处于活动状态，关闭会断开它。",
        L10nKey::ClosePaneBusyTitle => "关闭这个窗格？",
        L10nKey::CloseTabBusyTitle => "关闭这个标签页？",
        L10nKey::CloseBusyCommandBody => "{what} 还在运行，关闭会终止它。",
        L10nKey::CloseBusyAgentBody => "{agent} 还在工作，关闭会中断这一轮。",
        L10nKey::CloseIdleBody => "里面的 shell 也会随之结束。",
        L10nKey::CloseTabsTitle => "关闭 {count} 个标签页？",
        L10nKey::CloseTabsBody => "里面的 shell 也会一并结束。",
        L10nKey::Keep => "保留",
        L10nKey::SettingsNavAppearance => "外观",
        L10nKey::SettingsNavTerminal => "终端",
        L10nKey::SettingsNavInput => "键盘与鼠标",
        L10nKey::SettingsNavSsh => "SSH",
        L10nKey::SettingsNavAgents => "集成",
        L10nKey::SettingsNavKeybindings => "快捷键",
        L10nKey::SettingsNavAbout => "关于",
        L10nKey::SettingsWindowTitle => "设置",
        L10nKey::Reset => "重置",
        L10nKey::Save => "保存",
        L10nKey::Connect => "连接",
        L10nKey::Download => "下载",
        L10nKey::Link => "关联",
        L10nKey::SettingsThemeIntroTitle => "主题",
        L10nKey::SettingsFontSize => "终端字号",
        L10nKey::SettingsFontSizeDesc => "终端文字大小（磅）。",
        L10nKey::SettingsUiFontSize => "界面字号",
        L10nKey::SettingsUiFontSizeDesc => "标签页、面板、设置等非终端文字的大小。",
        L10nKey::SettingsUiFontFamily => "界面字体",
        L10nKey::SettingsUiFontFamilyDesc => "用于标签页、侧栏、弹窗和设置。",
        L10nKey::SettingsLineHeight => "行高",
        L10nKey::SettingsLineHeightDesc => "行间距为字号的倍数。",
        L10nKey::SettingsFontFamily => "终端字体",
        L10nKey::SettingsFontFamilyDesc => "从系统已安装的字体中选择。",
        L10nKey::SettingsBoldFont => "粗体字体",
        L10nKey::SettingsBoldFontDesc => "粗体字形；默认由主字体合成。",
        L10nKey::SettingsItalicFont => "斜体字体",
        L10nKey::SettingsItalicFontDesc => "斜体字形；默认由主字体合成。",
        L10nKey::SettingsFontLigatures => "字体连字",
        L10nKey::SettingsFontLigaturesDesc => "终端文字启用编程连字。",
        L10nKey::SettingsFontThicken => "笔画加粗",
        L10nKey::SettingsFontThickenDesc => "文字画得稍粗一些，重启后生效。",
        L10nKey::SettingsCursor => "光标",
        L10nKey::SettingsCursorShape => "光标形状",
        L10nKey::SettingsCursorShapeDesc => "终端光标的绘制方式。",
        L10nKey::SettingsPromptCursorShape => "提示符光标形状",
        L10nKey::SettingsPromptCursorShapeDesc => "shell 提示符处的光标。",
        L10nKey::SettingsCursorBlink => "光标闪烁",
        L10nKey::SettingsCursorBlinkDesc => "终端获得焦点时让光标闪烁。",
        L10nKey::SettingsLanguage => "语言",
        L10nKey::SettingsLanguageDesc => "选择 tty7 界面使用的语言。",
        L10nKey::SettingsLanguageEnglish => "English",
        L10nKey::SettingsLanguageChinese => "简体中文",
        L10nKey::SettingsLanguageJapanese => "日本語",
        L10nKey::SettingsLanguageRussian => "Русский",
        L10nKey::SettingsSearchLanguageKeywords => {
            "语言 区域设置 英文 中文 language locale english chinese"
        }
        L10nKey::SettingsTransparency => "透明度",
        L10nKey::SettingsOpacity => "不透明度",
        L10nKey::SettingsOpacityDesc => "低于 100% 时可透出桌面。",
        L10nKey::SettingsBlur => "模糊",
        L10nKey::SettingsBlurDesc => {
            if cfg!(target_os = "macos") {
                "模糊半透明窗口背后的内容。"
            } else {
                "模糊半透明窗口背后的内容。需要合成器支持——KDE Plasma 可以；GNOME 和裸 X11 下窗口只会变透明。"
            }
        }
        L10nKey::SettingsBlurAutoDesc => "模糊窗口背后的内容，仅限“自动”材质。",
        L10nKey::SettingsBackdrop => "背景材质",
        L10nKey::SettingsBackdropDesc => "半透明窗口背后的原生背景材质。",
        L10nKey::SettingsSearchBackdropKeywords => {
            "材质 背景 云母 亚克力 模糊 毛玻璃 窗口 material backdrop mica acrylic blur frosted window background"
        }
        L10nKey::SettingsBackdropAuto => "自动",
        L10nKey::SettingsBackdropBlur => "模糊",
        L10nKey::SettingsBackdropMica => "云母",
        L10nKey::SettingsBackdropMicaAlt => "云母 Alt",
        L10nKey::SettingsBackdropAcrylic => "亚克力",
        L10nKey::SettingsBackdropOff => "关闭",
        L10nKey::FollowTheme => "跟随主题",
        L10nKey::SettingsDimInactivePanes => "调暗非活动窗格",
        L10nKey::SettingsDimInactivePanesDesc => "在分屏中淡化未聚焦的窗格，让活动窗格更突出。",
        L10nKey::SettingsAutoHideTitlebarButtons => "悬停时显示标题栏按钮",
        L10nKey::SettingsAutoHideTitlebarButtonsDesc => {
            "新标签页、侧栏开关等按钮平时隐藏，鼠标移到标题栏上才出现。"
        }
        L10nKey::SettingsShowContentTitle => "显示内容区标题",
        L10nKey::SettingsShowContentTitleDesc => "在内容区顶部显示当前标签页的标题。",
        L10nKey::SettingsOpenThemesFolder => "打开主题文件夹",
        L10nKey::SettingsChangeThemeImage => "更改…",
        L10nKey::SettingsChooseThemeImage => "选择…",
        L10nKey::SettingsRemoveThemeImage => "移除",
        L10nKey::SettingsImageOpacity => "图片不透明度",
        L10nKey::SettingsImageOpacityDesc => "图片叠加在背景色上的显示强度。",
        L10nKey::SettingsEditTheme => "编辑主题",
        L10nKey::SettingsEditThemeIntro => "正在编辑副本，更改实时生效并保存到主题文件夹。",
        L10nKey::SettingsBackgroundImage => "背景图片",
        L10nKey::SettingsBackgroundImageDesc => "叠加在背景色之上、文字之下。",
        L10nKey::SettingsAnsiColors => "ANSI 颜色",
        L10nKey::SettingsCustomThemes => "自定义主题",
        L10nKey::SettingsThemesRejected => "主题文件夹里这些没能加载",
        L10nKey::ThemeDuplicateFailed => "无法复制主题",
        L10nKey::ThemeSaveFailed => "无法保存主题",
        L10nKey::OpenInFileManagerFailed => "无法打开 {path}",
        L10nKey::ExplorerMenuOpenIn => "在 tty7 中打开",
        L10nKey::ExplorerMenuOpenHere => "在此处打开 tty7",
        L10nKey::SettingsDuplicateToEdit => "复制以编辑",
        L10nKey::SettingsHosts => "主机",
        L10nKey::SettingsNothingMatches => "没有匹配 {query} 的内容。",
        L10nKey::SettingsInTty7 => "tty7 设置",
        L10nKey::SettingsImportFromSshConfig => "从 ~/.ssh/config 导入",
        L10nKey::SettingsNoHostsYet => "还没有主机",
        L10nKey::SettingsImportUnreadable => "无法读取 {path}——没有导入任何内容。",
        L10nKey::SettingsImportNoHosts => "{path} 中没有可导入的主机——只有通配符或 Match 规则。",
        L10nKey::SettingsImportSummary => {
            "新增 {count} 个主机——更新 {updated} 个，{unchanged} 个已是最新"
        }
        L10nKey::SettingsImportIgnored => {
            "有 {count} 个选项在 tty7 中没有对应设置，仍留在文件里：{options}"
        }
        L10nKey::SettingsImportMoreOptions => "还有 {count} 个",
        L10nKey::SettingsCouldntForgetPassword => "无法清除 {endpoint} 的已保存密码：{error}",
        L10nKey::SettingsSecurity => "安全",
        L10nKey::SettingsSecurityIntro => "主机可以在自己的高级选项中覆盖这些设置。",
        L10nKey::SettingsVerifyHostKeys => "校验主机密钥",
        L10nKey::SettingsVerifyHostKeysDesc => {
            "连接前对照 known_hosts 校验服务器密钥。关闭后无法察觉仿冒的服务器。"
        }
        L10nKey::WarnBeforeClosing => "关闭前警告",
        L10nKey::SettingsWarnBeforeClosingDesc => "关闭有活动 SSH 会话的标签页前确认。",
        L10nKey::SettingsNewHost => "新主机",
        L10nKey::SettingsKeepEditing => "继续编辑",
        L10nKey::SettingsHostRequired => "需要填写主机——不会被保存。",
        L10nKey::SettingsPortInvalid => "端口必须在 1-65535 之间——留空表示 22。",
        L10nKey::SettingsPassword => "密码",
        L10nKey::SettingsNameHint => "可不填",
        L10nKey::SettingsHostHint => "主机名或 IP",
        L10nKey::SettingsUserHint => "连接时再定",
        L10nKey::SettingsPasswordHint => "连接时再问",
        L10nKey::SettingsKeyPassphrase => "密钥口令",
        L10nKey::SettingsPassphraseNeedsKey => "先填一个密钥文件——口令是跟着它解锁的那个密钥存的。",
        L10nKey::SettingsCouldntSavePassword => "无法保存 {endpoint} 的密码：{error}",
        L10nKey::SettingsCouldntSavePassphrase => "无法保存 {key} 的口令：{error}",
        L10nKey::SettingsJumpHostUnknown => "没有名为 {jump_name} 的主机配置——不会被保存。",
        L10nKey::SettingsJumpHostSelf => "主机不能把自己当作跳板——不会被保存。",
        L10nKey::SettingsFwdNeedsBoth => "需要监听端口和目标 host:port——不会被保存。",
        L10nKey::SettingsFwdNeedsListen => "需要监听端口——不会被保存。",
        L10nKey::SettingsProxyPortInvalid => "端口必须在 1-65535 之间——只写主机则使用默认端口。",
        L10nKey::SettingsDefault => "默认",
        L10nKey::SettingsShell => "Shell",
        L10nKey::SettingsShellIntro => "新终端启动的程序，留空则使用{default}。",
        L10nKey::SettingsProgram => "Shell 程序",
        L10nKey::SettingsProgramDesc => "PATH 中的命令名或绝对路径，如 zsh、fish。",
        L10nKey::SettingsArguments => "Shell 参数",
        L10nKey::SettingsArgumentsDesc => "按命令行规则切分，例如 -l 或 -c \"echo hi\"。",
        L10nKey::SettingsArgumentsInvalid => "引号不配对，该值未保存。",
        L10nKey::SettingsStartIn => "启动目录",
        L10nKey::SettingsStartInDesc => "tty7 的启动目录、主目录或固定路径。",
        L10nKey::SettingsCustomPath => "自定义路径",
        L10nKey::SettingsCustomPathDesc => "新 shell 启动的目录。",
        L10nKey::SettingsWdInherit => "继承",
        L10nKey::SettingsWdHome => "主目录",
        L10nKey::SettingsWdCustom => "自定义",
        L10nKey::SettingsWdPathInvalid => "这个目录不存在，该值未保存。",
        L10nKey::SettingsShellFooter => {
            "仅用于没有目录可继承的 shell，例如窗口的第一个标签页；新标签页和分屏沿用当前窗格的目录。"
        }
        L10nKey::SettingsScrolling => "滚动",
        L10nKey::SettingsScrollback => "终端输出历史行数",
        L10nKey::SettingsScrollbackDesc => "每个窗格保留的终端输出行数。仅适用于新窗格。",
        L10nKey::SettingsScrollSpeed => "滚动速度",
        L10nKey::SettingsScrollSpeedDesc => "应用于鼠标滚轮滚动的倍率。",
        L10nKey::SettingsSmoothScroll => "平滑滚动",
        L10nKey::SettingsSmoothScrollDesc => "滚轮每格分几帧滚动，触控板不受影响。",
        L10nKey::SettingsMouse => "鼠标",
        L10nKey::SettingsFocusFollowsMouse => "焦点跟随鼠标",
        L10nKey::SettingsFocusFollowsMouseDesc => "悬停窗格即聚焦，无需点击。",
        L10nKey::SettingsHideMouseWhileTyping => "输入时隐藏鼠标",
        L10nKey::SettingsHideMouseWhileTypingDesc => "输入时隐藏指针；下次移动鼠标时恢复。",
        L10nKey::SettingsMouseZoom => "滚轮缩放",
        L10nKey::SettingsMouseZoomDesc => "按住它滚动滚轮可缩放终端字号。",
        L10nKey::SettingsMouseZoomOff => "关闭",
        L10nKey::SettingsReportMouseToApps => "向应用报告鼠标",
        L10nKey::SettingsMouseRightClick => "鼠标右键行为",
        L10nKey::SettingsMouseRightClickDesc => {
            "菜单：打开右键菜单；复制粘贴：有选区时复制并取消选区，无选区时粘贴。应用接管鼠标时按住 Shift 使用。"
        }
        L10nKey::SettingsMouseRightClickMenu => "菜单",
        L10nKey::SettingsMouseRightClickCopyPaste => "复制粘贴",
        L10nKey::SettingsReportMouseToAppsDesc => {
            "让 vim、tmux 等应用处理点击，按住 Shift 则留在本地。"
        }
        L10nKey::SettingsBell => "铃声",
        L10nKey::SettingsTerminalBell => "终端铃声",
        L10nKey::SettingsTerminalBellDesc => "铃声（^G）的提示方式。",
        L10nKey::SettingsLinks => "链接",
        L10nKey::DetectUrls => "检测 URL",
        L10nKey::SettingsDetectUrlsDesc => "悬停显示下划线，{modifier}+点击 打开。",
        L10nKey::ForwardSshLoopbackLinks => "转发远程端口",
        L10nKey::SettingsForwardSshLoopbackLinksDesc => "在本机打开远程窗格的 localhost 链接。",
        L10nKey::SettingsBellModeOff => "关",
        L10nKey::SettingsBellModeVisual => "闪烁",
        L10nKey::SettingsBellModeAudible => "声音",
        L10nKey::SettingsBellModeBoth => "闪烁 + 声音",
        L10nKey::SettingsSelectionClipboard => "选择与剪贴板",
        L10nKey::SettingsSmartSelection => "智能选择",
        L10nKey::SettingsSmartSelectionDesc => "双击选中完整的 URL、路径、邮箱或括号对。",
        L10nKey::SettingsCopyOnSelect => "选中即复制",
        L10nKey::SettingsCopyOnSelectDesc => {
            if cfg!(target_os = "macos") {
                "用鼠标选中文本时立即复制到剪贴板，无需按 ⌘C。"
            } else {
                "用鼠标选中文本时立即复制到剪贴板，无需按 Ctrl+Shift+C。"
            }
        }
        L10nKey::SettingsTrimTrailingSpaces => "复制时去除末尾空格",
        L10nKey::SettingsTrimTrailingSpacesDesc => "去除每行复制文本末尾的空白。",
        L10nKey::SettingsKeyboard => "键盘",
        L10nKey::SettingsOptionAsMeta => "Option (⌥) 作为 Meta",
        L10nKey::SettingsOptionAsMetaDesc => "⌥+按键 作为 Meta，如 ⌥B 后退一词。",
        L10nKey::SettingsAgentsIntro => "Agent 钩子",
        L10nKey::SettingsAgentsIntroDesc => {
            "安装 hook 后，agent 的状态（进行中 / 等待中 / 已完成）会显示在标签栏。"
        }
        L10nKey::SettingsReadingAgentConfig => "正在读取这台机器的 AI agent 配置…",
        L10nKey::SettingsStatusInstalled => "已安装",
        L10nKey::SettingsInstall => "安装",
        L10nKey::SettingsReinstall => "重新安装",
        L10nKey::SettingsUpdate => "更新",
        L10nKey::SettingsUninstall => "卸载",
        L10nKey::SettingsOfflineMachines => {
            "还有 {count} 台已保存的机器未连接——在其中一台上打开工作区，即可在那台机器上安装 hook。"
        }
        L10nKey::SettingsSyncWithSystem => "外观",
        L10nKey::SettingsSyncWithSystemDesc => "跟随操作系统外观，并分别使用浅色与深色主题。",
        L10nKey::SettingsLegiblePalette => "低对比度颜色纠偏",
        L10nKey::SettingsLegiblePaletteDesc => "自动调整在背景上看不清的亮色。",
        L10nKey::SettingsCustom => "自定义",
        L10nKey::SettingsCustomValue => "自定义 ({value})",
        L10nKey::SettingsDark => "深色",
        L10nKey::SettingsLight => "浅色",
        L10nKey::SettingsStartupWindow => "启动窗口",
        L10nKey::SettingsStartupWindowDesc => "tty7 启动时的窗口状态。",
        L10nKey::SettingsRememberWindowSize => "记住窗口大小与位置",
        L10nKey::SettingsRememberWindowSizeDesc => "按上次退出时的大小和位置打开窗口。",
        L10nKey::SettingsRestoreLastLayout => "恢复上次布局",
        L10nKey::SettingsRestoreLastLayoutDesc => "启动时恢复上次的标签页、分屏和目录。",
        L10nKey::SettingsShowTrayIcon => "显示托盘图标",
        L10nKey::SettingsShowTrayIconDesc => "agent 等待输入时提醒，可跳到对应窗格。",
        L10nKey::SettingsTabs => "标签页",
        L10nKey::SettingsNewTabPosition => "新标签页位置",
        L10nKey::SettingsNewTabPositionDesc => "新打开的标签页插入的位置。",
        L10nKey::SettingsConfirmClose => "关闭前确认",
        L10nKey::SettingsConfirmCloseDesc => {
            "关闭标签页或窗格前是否先问一下。设了“关闭前警告”的 SSH 主机无论如何都会询问。"
        }
        L10nKey::ConfirmCloseNever => "从不",
        L10nKey::ConfirmCloseWhenBusy => "有程序在运行时",
        L10nKey::ConfirmCloseAlways => "总是",
        L10nKey::SettingsSidebarGrouping => "自动分组",
        L10nKey::SettingsSidebarGroupingDesc => {
            "未固定的标签页按 git 仓库分组，SSH 标签页按主机分组。"
        }
        L10nKey::SettingsNotifications => "通知",
        L10nKey::SettingsNotifyOnCommandFinish => "命令完成时通知",
        L10nKey::SettingsNotifyOnCommandFinishDesc => "较长的前台命令完成后发出桌面提醒。",
        L10nKey::SettingsNotifyThreshold => "最短命令运行时间",
        L10nKey::SettingsNotifyThresholdDesc => "仅在命令运行达到此时长后发送完成通知。",
        L10nKey::SettingsWindow => "启动与恢复",
        L10nKey::NotifyModeNever => "从不",
        L10nKey::NotifyModeUnfocused => "窗口未聚焦时",
        L10nKey::NotifyModeAlways => "总是",
        L10nKey::ProgramNotesDropped => "此窗格的其他通知未显示",
        L10nKey::SettingsStartupNormal => "普通",
        L10nKey::SettingsStartupMaximized => "最大化",
        L10nKey::SettingsStartupFullscreen => "全屏",
        L10nKey::SettingsAfterCurrent => "当前之后",
        L10nKey::SettingsAtEnd => "末尾",
        L10nKey::SettingsPrefix => "前缀",
        L10nKey::SettingsKeybindingsIntroDesc => {
            "点击快捷键后按新按键，连按可组成序列（如 Ctrl-B X）。Esc 取消，Backspace 清除。"
        }
        L10nKey::KeybindGoToTab => "跳到第 {n} 个标签页",
        L10nKey::KeybindGoToWorkspace => "跳到第 {n} 个工作区",
        L10nKey::KeybindForkSessionRight => "向右 Fork 会话",
        L10nKey::KeybindForkSessionLeft => "向左 Fork 会话",
        L10nKey::KeybindForkSessionDown => "向下 Fork 会话",
        L10nKey::KeybindForkSessionUp => "向上 Fork 会话",
        L10nKey::SettingsAboutDesc1 => {
            "为长时间工作打造的终端工作台。会话不随窗口关闭而结束，远程主机用起来和本地一样，编程 agent 的状态直接显示在侧栏里。"
        }
        L10nKey::SettingsDefaultTerminal => "默认终端",
        L10nKey::SettingsDefaultTerminalDesc => {
            "用 tty7 打开 Unix 可执行文件、SSH 链接和 man 页面。"
        }
        L10nKey::SettingsDefaultTerminalSet => "设为默认终端",
        L10nKey::SettingsDefaultTerminalSetSuccess => {
            "tty7 已成为受支持终端文件和链接的默认处理程序。"
        }
        L10nKey::SettingsDefaultTerminalSetFailed => "无法将 tty7 设为默认终端：{error}",
        L10nKey::SettingsVersion => "版本",
        L10nKey::SettingsUpdates => "更新",
        L10nKey::SettingsUpdateAndRelaunch => "更新并重新启动",
        L10nKey::SettingsUpdateViewRelease => "查看发布页面",
        L10nKey::SettingsUpdateChecking => "正在检查更新…",
        L10nKey::SettingsUpdateUpToDate => "当前已是最新版本。",
        L10nKey::SettingsUpdateDownloadingPercent => "正在下载更新… {percent}%，共 {size}",
        L10nKey::SettingsUpdateDownloadingBytes => "正在下载更新… 已下载 {received}",
        L10nKey::SettingsUpdateVerifying => "正在校验下载的更新…",
        L10nKey::SettingsUpdateInstalling => "正在重启以应用更新…",
        L10nKey::SettingsUpdateCheckNow => "立即检查",
        L10nKey::SettingsUpdateCancel => "取消下载",
        L10nKey::SettingsUpdateRetry => "重试",
        L10nKey::SettingsUpdateDismiss => "知道了",
        L10nKey::SettingsUpdateDownloadManually => "手动下载",
        L10nKey::SettingsUpdateFailedTitle => "更新到 {version} 失败。",
        L10nKey::SettingsUpdateReady => "{version} 已下载并校验完成，可以安装。",
        L10nKey::SettingsUpdateReadyNextLaunch => "下次启动 tty7 时会自动装上。",
        L10nKey::SettingsUpdateInstallNow => "安装并重启",
        L10nKey::SettingsUpdateDiscard => "丢弃",
        L10nKey::SettingsAutoDownload => "后台下载更新",
        L10nKey::SettingsAutoDownloadDesc => "后台下载新版本，安装时只需重启；不会未经确认就安装。",
        L10nKey::SettingsUpdateChannel => "更新通道",
        L10nKey::SettingsUpdateChannelDesc => "Nightly 每晚从最新代码构建，未经发布测试。",
        L10nKey::SettingsUpdateChannelStable => "稳定版",
        L10nKey::SettingsUpdateChannelNightly => "每夜构建",
        L10nKey::SettingsDaemonStale => "tty7 server 仍运行在 {build}。",
        L10nKey::SettingsDaemonStaleDesc => {
            "tty7 已更新，但窗格仍跑在旧版 server 上。重启后换用新版，窗格中运行的进程会全部结束。"
        }
        L10nKey::UpdateDialogTitle => "有可用更新",
        L10nKey::UpdateDialogDetail => {
            "tty7 {version} 已发布，你现在是 {current}。安装会重启应用；tty7 server 不动，窗格里的东西都还在。"
        }
        L10nKey::UpdateDialogDetailWindows => {
            "tty7 {version} 已发布，你现在是 {current}。安装会重启应用和 tty7 server：窗格里的进程会被结束，标签页和布局以全新的 shell 恢复。"
        }
        L10nKey::UpdateDialogDetailManual => "tty7 {version} 已发布，你现在是 {current}。{hint}",
        L10nKey::UpdateDialogCannotSelfUpdate => "当前安装方式不支持自动更新。",
        L10nKey::UpdateDialogLater => "以后再说",
        L10nKey::UpdateDialogNextLaunch => "下次启动时安装",
        L10nKey::UpdateDialogNeedsElevation => {
            "tty7 是为所有用户安装的，安装前 Windows 会请求一次管理员批准。tty7 本身不会以管理员身份运行。"
        }
        L10nKey::SettingsUpdateCheckFailed => "无法检查更新：{error}",
        L10nKey::SettingsUpdatePrepareFailed => "更新失败：{error}",
        L10nKey::SettingsUpdateLaunchFailed => "无法启动安装程序：{error}",
        L10nKey::SettingsUpdateUnsupportedMacos => {
            "当前应用不在可写的 tty7.app 包内，无法自动更新。请把 tty7 移到“应用程序”，或打开发布页面更新。"
        }
        L10nKey::SettingsUpdateUnsupportedLinux => {
            "发布版本中没有适用于该架构的 Linux 包。请自行从源码构建，或使用包管理器。"
        }
        L10nKey::SettingsUpdateLinuxPackage => {
            "Linux 需要手动更新。请到发布页面下载 {name}，或使用包管理器。"
        }
        L10nKey::SettingsUpdateUnsupportedWindows => {
            "当前副本不是可识别的 Inno Setup 安装版或便携 ZIP 版，无法自我更新。请打开发布页面手动更新。"
        }
        L10nKey::SettingsUpdateWindowsAllUsers => {
            "tty7 是为所有用户安装的，替换需要管理员权限，而 tty7 不会自行提权。请打开发布页面，自行运行安装程序更新。"
        }
        L10nKey::SettingsUpdateUnsupportedPlatform => "此平台不支持自动安装，请打开发布页面。",
        L10nKey::SettingsUpdateMissingPackage => {
            "该版本没有适用于当前安装的 {name} 包。请打开发布页面选择其他包。"
        }
        L10nKey::SettingsUpdateMissingChecksums => {
            "该版本缺少 checksums.txt，因此 tty7 拒绝自动安装。"
        }
        L10nKey::SettingsVersionAvailable => "新版本 {version} 可用。",
        L10nKey::SettingsCheckUpdatesDesc => "无法就地更新的安装方式会改为打开发布页面。",
        L10nKey::SettingsCheckUpdatesOnLaunch => "启动时检查更新",
        L10nKey::SettingsCommandLine => "命令行",
        L10nKey::SettingsCommandLineDesc => "把 tty7 命令加入 PATH，下次启动生效。",
        L10nKey::SettingsInstallCliOnPath => "将 tty7 命令安装到 PATH",
        L10nKey::SettingsServer => "tty7 server",
        L10nKey::SettingsServerDesc => "让终端会话在后台持续运行。",
        L10nKey::SettingsRestartServer => "重启 tty7 server…",
        L10nKey::SettingsAppHttpProxy => "更新代理",
        L10nKey::SettingsAppHttpProxyDesc => "仅用于 tty7 的更新检查，留空跟随系统。",
        L10nKey::SettingsAppHttpProxyInvalid => "不是有效的代理地址，该值未保存。",
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
        L10nKey::SettingsMuseManualInstall => "需要手动安装",
        L10nKey::AppAgentHooksMuseManualInstall => "请在目标机器上运行: {command}",
        L10nKey::SettingsAgentQoderCn => "Qoder CN CLI",
        L10nKey::SettingsSearchAboutKeywords => {
            "关于 版本 许可证 致谢 构建 更新 检查 github about version license credits update"
        }
        L10nKey::SettingsSearchAppHttpProxyKeywords => {
            "代理 proxy http https socks socks5 clash v2ray 网络 下载 更新"
        }
        L10nKey::SettingsSearchAnsiColorsKeywords => {
            "ANSI颜色 调色板 终端颜色 主题 ansi colors palette terminal theme"
        }
        L10nKey::SettingsSearchBackgroundImageKeywords => {
            "背景图片 背景图 壁纸 图片 照片 主题 background image wallpaper picture theme"
        }
        L10nKey::SettingsSearchImageOpacityKeywords => {
            "背景图片 不透明度 透明度 强度 壁纸 background image opacity strength fade"
        }
        L10nKey::SettingsSearchArgumentsKeywords => {
            "参数 shell 启动参数 登录参数 arguments shell flags login args"
        }
        L10nKey::SettingsSearchBlurKeywords => {
            "模糊 毛玻璃 半透明 窗口 背景 blur frosted vibrancy window background"
        }
        L10nKey::SettingsSearchBoldFontKeywords => "粗体 字体粗细 字重 bold font weight typeface",
        L10nKey::SettingsSearchClaudeCodeKeywords => {
            "Claude Code agent 集成 hook 安装 卸载 状态 会话 claude agent integration hooks install"
        }
        L10nKey::SettingsSearchCodexKeywords => {
            "Codex agent 集成 hook 安装 OpenAI codex agent integration hooks install"
        }
        L10nKey::SettingsSearchTraeCodeKeywords => {
            "TraeCode traecli traex agent 集成 hook 安装 agent integration hooks install"
        }
        L10nKey::SettingsSearchCommandLineToolKeywords => {
            "命令行工具 cli tty7 路径 shell 命令 安装 符号链接 terminal command line tool"
        }
        L10nKey::SettingsSearchCopilotCliKeywords => {
            "Copilot CLI agent 集成 hook 安装 GitHub copilot agent integration hooks install"
        }
        L10nKey::SettingsSearchCopyOnSelectKeywords => {
            "选中即复制 复制 剪贴板 选择 鼠标 copy on select clipboard yank"
        }
        L10nKey::SettingsSearchCursorBlinkKeywords => {
            "光标闪烁 闪烁 光标 blink cursor blinking flash"
        }
        L10nKey::SettingsSearchCursorShapeKeywords => {
            "光标形状 光标 块 竖线 下划线 cursor shape caret block bar underline beam"
        }
        L10nKey::SettingsSearchPromptCursorShapeKeywords => {
            "提示符 光标形状 光标 块 竖线 下划线 prompt cursor shape caret block bar underline beam"
        }
        L10nKey::SettingsSearchCustomThemesKeywords => {
            "自定义主题 复制 编辑 颜色 文件夹 背景图片 壁纸 yaml 导入 theme custom edit duplicate colors import background image wallpaper"
        }
        L10nKey::SettingsSearchDetectUrlsKeywords => {
            "检测URL 链接 超链接 可点击 打开 detect urls links hyperlink open"
        }
        L10nKey::SettingsSearchDimInactivePanesKeywords => {
            "调暗 非活动窗格 淡化 未聚焦 分屏 高亮 active dimming pane focus"
        }
        L10nKey::SettingsSearchAutoHideTitlebarButtonsKeywords => {
            "自动隐藏 标题栏 按钮 悬停 鼠标 简洁 新标签页 侧栏 auto hide title bar buttons hover"
        }
        L10nKey::SettingsSearchShowContentTitleKeywords => {
            "内容区 标题 顶栏 标签页 显示 隐藏 content title bar tab show hide"
        }
        L10nKey::SettingsSearchFocusFollowsMouseKeywords => {
            "焦点跟随鼠标 悬停 激活 窗格 focus follows mouse hover activate pane"
        }
        L10nKey::SettingsSearchFontFamilyKeywords => {
            "字体 字体族 等宽 排版 font family monospace typography typeface"
        }
        L10nKey::SettingsSearchFontLigaturesKeywords => {
            "字体连字 连字 字形 typography ligatures glyph fira"
        }
        L10nKey::SettingsSearchFontThickenKeywords => {
            "字体平滑 加粗 变细 字重 笔画 font smoothing thicken bold weight thin AppleFontSmoothing"
        }
        L10nKey::SettingsSearchFontSizeKeywords => {
            "字号 字体大小 文字 放大 缩小 typography font size bigger smaller zoom"
        }
        L10nKey::SettingsSearchForwardSshLoopbackLinksKeywords => {
            "SSH回环链接 端口转发 隧道 localhost 转发 自动转发 端口检测 forward ssh loopback links tunnel ports"
        }
        L10nKey::SettingsSearchGrokBuildKeywords => {
            "Grok Build agent 集成 hook 安装 xai grok build agent integration hooks install"
        }
        L10nKey::SettingsSearchHideMouseWhileTypingKeywords => {
            "输入时隐藏鼠标 隐藏鼠标 指针 自动隐藏 hide mouse typing cursor pointer autohide"
        }
        L10nKey::SettingsSearchHostsKeywords => {
            "主机 SSH 连接 保存 主机配置 配置文件 导入 ssh_config 管理 添加 编辑 快速连接 hosts ssh profile import connect"
        }
        L10nKey::SettingsSearchItalicFontKeywords => "斜体 字体样式 italic oblique typeface",
        L10nKey::SettingsSearchKeybindingsKeywords => {
            "按键绑定 快捷键 热键 键盘 绑定 前缀 tmux keybindings shortcut hotkey binding prefix"
        }
        L10nKey::SettingsSearchKeybindingsTitle => "快捷键",
        L10nKey::SettingsSearchLineHeightKeywords => {
            "行高 行间距 行距 typography line height spacing leading"
        }
        L10nKey::SettingsSearchUiFontFamilyKeywords => {
            "界面 字体族 UI 字体 标签页 侧栏 interface font family ui typeface"
        }
        L10nKey::SettingsSearchNewTabPositionKeywords => {
            "新标签页位置 标签页 顺序 末尾 当前之后 new tab position tabs order end after current"
        }
        L10nKey::SettingsSearchConfirmCloseKeywords => {
            "关闭前确认 关闭 标签页 窗格 询问 提示 运行中 空闲 总是 从不 confirm close tab pane ask prompt busy idle always never"
        }
        L10nKey::SettingsSearchNotifyOnCommandFinishKeywords => {
            "命令完成时通知 通知 提醒 命令 notify command finish notification alert desktop"
        }
        L10nKey::SettingsSearchNotifyThresholdKeywords => {
            "通知阈值 通知 秒数 时长 命令 notify threshold notification duration seconds"
        }
        L10nKey::SettingsSearchOpacityKeywords => {
            "不透明度 透明度 窗口 半透明 alpha opacity transparency translucent window"
        }
        L10nKey::SettingsSearchOpencodeKeywords => {
            "OpenCode agent 集成 插件 安装 opencode agent integration plugin install"
        }
        L10nKey::SettingsSearchOptionAsMetaKeywords => {
            "Option作为Meta 修饰键 alt option meta 转义 escape macos keyboard modifier"
        }
        L10nKey::SettingsSearchOhMyPiKeywords => {
            "Oh My Pi agent 集成 扩展 安装 omp oh my pi agent integration extension install"
        }
        L10nKey::SettingsSearchGeminiKeywords => {
            "Gemini agent 集成 钩子 安装 gemini google agent integration hooks install"
        }
        L10nKey::SettingsSearchDroidKeywords => {
            "Droid agent 集成 钩子 安装 droid factory agent integration hooks install"
        }
        L10nKey::SettingsSearchQwenCodeKeywords => {
            "Qwen Code 通义千问 agent 集成 钩子 安装 qwen code agent integration hooks install"
        }
        L10nKey::SettingsSearchGooseKeywords => {
            "Goose agent 集成 钩子 插件 安装 goose agent integration hooks plugin install"
        }
        L10nKey::SettingsSearchKimiCodeKeywords => {
            "Kimi Code 月之暗面 agent 集成 钩子 安装 kimi code moonshot agent integration hooks install"
        }
        L10nKey::SettingsSearchQoderCLIKeywords => "Qoder CLI agent 集成 钩子 安装 qoder qodercli",
        L10nKey::SettingsSearchCrushKeywords => "Crush agent 集成 钩子 安装 crush",
        L10nKey::SettingsSearchCodeBuddyKeywords => {
            "CodeBuddy 腾讯云代码助手 agent 集成 钩子 安装 codebuddy cbc tencent"
        }
        L10nKey::SettingsSearchCursorCliKeywords => {
            "Cursor CLI agent 集成 钩子 安装 cursor cursor-agent"
        }
        L10nKey::SettingsSearchPrimeAgentKeywords => {
            "Prime Agent agent 集成 扩展 安装 prime prime-agent primeintellect"
        }
        L10nKey::SettingsSearchEmpryoKeywords => "Empryo agent 集成 钩子 安装 empryo",
        L10nKey::SettingsSearchJcodeKeywords => "jcode agent 集成 钩子 安装 jcode",
        L10nKey::SettingsSearchMuseKeywords => "Muse Code agent 集成 钩子 插件 安装 muse meta",
        L10nKey::SettingsSearchAntigravityKeywords => {
            "Antigravity agent 集成 钩子 安装 antigravity agy google"
        }
        L10nKey::SettingsSearchQoderCnKeywords => {
            "Qoder CN CLI 国内版 agent 集成 钩子 安装 qodercn qoderclicn qoder-cn qoder"
        }
        L10nKey::SettingsSearchPiKeywords => {
            "Pi agent 集成 扩展 安装 pi agent integration extension install"
        }
        L10nKey::SettingsSearchProgramKeywords => {
            "程序 shell 二进制 zsh bash fish nu nushell pwsh powershell 可执行文件 启动 program shell binary launch"
        }
        L10nKey::SettingsSearchRememberWindowSizeKeywords => {
            "记住窗口大小位置 窗口 大小 位置 启动 记住 remember window size position geometry"
        }
        L10nKey::SettingsSearchReportMouseToAppsKeywords => {
            "鼠标报告 鼠标 vim tmux 点击 滚动 shift report mouse apps"
        }
        L10nKey::SettingsSearchRestoreLastLayoutKeywords => {
            "恢复上次布局 恢复 会话 标签页 分屏 布局 restore last layout tabs splits"
        }
        L10nKey::SettingsSearchScrollSpeedKeywords => {
            "滚动速度 鼠标滚轮 滚动倍率 scroll speed mouse wheel multiplier scrolling"
        }
        L10nKey::SettingsSearchSmoothScrollKeywords => {
            "平滑滚动 动画 缓动 滚轮 触控板 smooth animation ease wheel trackpad scrolling"
        }
        L10nKey::SettingsSearchUpdateChannelKeywords => {
            "更新通道 稳定版 每夜构建 预发布 版本 update channel stable nightly release"
        }
        L10nKey::SettingsSearchCheckUpdatesOnLaunchKeywords => {
            "启动时检查更新 自动 检查 启动 update check launch startup automatic"
        }
        L10nKey::SettingsSearchAutoDownloadKeywords => {
            "后台下载更新 下载 后台 安装 流量 update download background install metered"
        }
        L10nKey::SettingsSearchScrollbackKeywords => {
            "scrollback 回看 向上滚动 历史 缓冲区 行数 scrollback history buffer lines"
        }
        L10nKey::SettingsSearchShowTrayIconKeywords => {
            "显示托盘图标 托盘 菜单栏 状态 图标 show tray icon menu bar status"
        }
        L10nKey::SettingsSearchSidebarGroupingKeywords => {
            "自动分组 标签页 分组 仓库 git ssh 主机 固定 未分组 侧栏 sidebar auto grouping tabs repo repository pinned pin host ungrouped"
        }
        L10nKey::SettingsSearchSmartSelectionKeywords => {
            "智能选择 双击 选择 单词 URL 路径 邮箱 括号 smart selection double click"
        }
        L10nKey::SettingsSearchStartInKeywords => {
            "起始目录 工作目录 启动目录 主目录 继承 自定义 cwd working directory start home inherit custom"
        }
        L10nKey::SettingsSearchSyncWithSystemKeywords => {
            "主题 跟随系统 自动 深色 浅色 外观 模式 theme dark light auto follow system"
        }
        L10nKey::SettingsSearchLegiblePaletteKeywords => {
            "颜色 低对比度 可读 纠偏 调色板 修正 contrast legible palette parameter"
        }
        L10nKey::SettingsSearchTerminalBellKeywords => {
            "终端铃声 铃声 提示音 闪烁 静音 两者 同时 beep bell terminal audible visual both"
        }
        L10nKey::SettingsSearchThemeKeywords => {
            "外观 颜色 主题 配色 深色 浅色 背景 前景 强调色 跟随系统 appearance color scheme dark light palette"
        }
        L10nKey::SettingsSearchTrimTrailingSpacesKeywords => {
            "复制时去除空格 去除末尾空格 剪贴板 空白 trim trailing spaces copy whitespace"
        }
        L10nKey::SettingsSearchVerifyHostKeysKeywords => {
            "校验主机密钥 主机密钥 known_hosts 指纹 mitm 安全 verification ssh host keys"
        }
        L10nKey::SettingsSearchWarnBeforeClosingKeywords => {
            "关闭前警告 确认关闭 SSH 标签页 窗格 会话 warn before closing ssh confirm"
        }
        L10nKey::SettingsSearchStartupWindowKeywords => {
            "启动窗口 启动 最大化 全屏 普通 startup window launch maximized fullscreen normal"
        }
        L10nKey::SwitcherNoMatch => "没有匹配的工作区或机器。",
        L10nKey::AddSshHost => "添加 SSH 主机…",
        L10nKey::RestartServer => "重启 tty7 server",
        L10nKey::Ok => "确定",
        L10nKey::SftpPasteUploadFailed => "无法将 {name} 上传到 {host}：{error}",
        L10nKey::ForwardDisconnected => "已断开",
        L10nKey::ForwardDisconnectedFrom => "与 {host} 的连接已断开",
        L10nKey::SshEditProfile => "编辑连接…",
        L10nKey::ForwardTooltipAdd => "添加转发",
        L10nKey::ForwardTooltipRemove => "移除",
        L10nKey::ForwardTooltipTurnOn => "开启",
        L10nKey::ForwardTooltipTurnOff => "关闭——规则会保留",
        L10nKey::ForwardSwitchFailed => "无法切换这条转发——{error}",
        L10nKey::ForwardLocal => "本地",
        L10nKey::ForwardRemote => "远程",
        L10nKey::ForwardDynamic => "动态",
        L10nKey::ForwardBindLabel => "绑定",
        L10nKey::ForwardToLabel => "到",
        L10nKey::ForwardSocksLabel => "SOCKS",
        L10nKey::ForwardAdd => "添加",
        L10nKey::ForwardPortLabel => "远程端口",
        L10nKey::ForwardPortHere => "在 localhost:{port} 打开",
        L10nKey::ForwardNeedsPort => "端口是 1 到 65535 之间的数字。",
        L10nKey::ForwardAdvancedToggle => "高级",
        L10nKey::ForwardSimpleToggle => "简单",
        L10nKey::ForwardRequestFailed => "联系不上这个会话——什么都没有改动。",
        L10nKey::SshPromptNewKey => "新 {fingerprint}",
        L10nKey::SshPromptOldKey => "旧 {old_fingerprint}",
        L10nKey::SshPromptHostKeyNewAlgorithm => {
            "此前已记录这台主机的 {previous_algorithm} 密钥。这是一把新的 {algorithm} 密钥，不会替换原有密钥。"
        }
        L10nKey::SshPromptTypeYesToOverride => "输入 yes 才能启用“覆盖”。",
        L10nKey::SettingsDiscard => "放弃",
        L10nKey::PanelNoSession => "没有活动会话。",
        L10nKey::PanelNoSessionHint => "打开一个标签页以在此处查看其 shell、目录和进程。",
        L10nKey::PanelSessionSubtitle => "会话",
        L10nKey::PanelProcessesSubtitle => "进程",
        L10nKey::PanelProcessesTotal => "合计",
        L10nKey::PanelPortsSubtitle => "端口",
        L10nKey::PanelLatency => "延迟",
        L10nKey::PanelPortsUnsupported => "对端的 tty7-server 太旧，列不出端口。",
        L10nKey::PanelPortsProbeFailed => "没能查出这个窗格在监听什么。",
        L10nKey::PanelPortsEmpty => "没有转发的端口",
        L10nKey::PanelPortsRestricted => "这里有以其他用户身份运行的进程，看不到它们的端口。",
        L10nKey::PortAutoForwarded => "远程 :{port} 现在是 http://localhost:{local}",
        L10nKey::PanelCwd => "工作目录",
        L10nKey::PanelShell => "shell",
        L10nKey::PanelSsh => "ssh",
        L10nKey::PanelAgentWorking => "进行中",
        L10nKey::PanelAgentWaiting => "等待中",
        L10nKey::PanelAgentDone => "已完成",
        L10nKey::PanelRevealInFinder => "在访达中显示",
        L10nKey::PanelOpenFolder => "打开文件夹",
        L10nKey::PanelOpenInBrowser => "在浏览器中打开",
        L10nKey::WindowStop => "停止",
        L10nKey::WindowDelete => "删除",
        L10nKey::WindowThisWorkspace => "此工作区",
        L10nKey::WindowConfirmTitle => "{verb}工作区“{name}”？",
        L10nKey::WindowStopUnreachable => "无法连接到其所在机器。仍在运行的 shell 将会被终止。",
        L10nKey::WindowDeleteUnreachable => {
            "无法连接到其所在机器。仍在运行的 shell 将会被终止，布局也将被清除。"
        }
        L10nKey::WindowStopShells => "{count} 个正在运行的 shell 将会被终止。",
        L10nKey::WindowDeleteShells => "{count} 个正在运行的 shell 将会被终止，布局也将被清除。",
        L10nKey::PendingConnecting => "正在连接 {machine}…",
        L10nKey::PendingUnreachable => "无法连接到 {machine}",
        L10nKey::HomeTimeJustNow => "刚刚",
        L10nKey::HomeTimeMinutesAgo => "{count} 分钟前",
        L10nKey::HomeTimeHourAgo => "1 小时前",
        L10nKey::HomeTimeHoursAgo => "{count} 小时前",
        L10nKey::HomeTimeYesterday => "昨天",
        L10nKey::HomeTimeDaysAgo => "{count} 天前",
        L10nKey::HomeTimeWeeksAgo => "{count} 周前",
        L10nKey::HomeTimeMonthsAgo => "{count} 个月前",
        L10nKey::HomeTimeOverYearAgo => "一年多前",
        L10nKey::HomeReopenNamed => "重新打开“{name}”",
        L10nKey::RemoteStripDisconnected => "未连接到 {machine}",
        L10nKey::RemoteStripConnecting => "正在连接 {machine}…",
        L10nKey::RemoteStripReconnecting => "正在重新连接 {machine}…",
        L10nKey::RemoteStripReconnectingAttempt => "正在重新连接 {machine}…（第 {count} 次尝试）",
        L10nKey::RemoteStripReconnectingWhy => "正在重新连接 {machine}…（上次失败：{error}）",
        L10nKey::RemoteStripReconnectingAttemptWhy => {
            "正在重新连接 {machine}…（第 {count} 次尝试，上次失败：{error}）"
        }
        L10nKey::RemoteStripPreempted => "此工作区已在 {by} 上打开",
        L10nKey::RemoteStripFailed => "未连接到 {machine}——{error}",
        L10nKey::RemoteStripRouteLost => "{machine} 的连接配置已不存在，无法重连",
        L10nKey::RemoteRouteParkedHint => {
            "其连接配置已不存在，不会再自动重连。远端会话仍在——新建配置连上去就能在工作区列表里找回。"
        }
        L10nKey::RemoteNoticePreempted => "已在别处打开——输入无效",
        L10nKey::RemoteNoticeDisconnected => "未连接——输入无效",
        L10nKey::RemoteActionRetryNow => "立即重试",
        L10nKey::RemoteActionTakeBack => "收回",
        L10nKey::PaneLeasedBy => "正在 {by} 上使用，按它的屏幕尺寸显示",
        L10nKey::RemoteActionConnect => "连接",
        L10nKey::RemoteActionRetry => "重试",
        L10nKey::RemoteActionRemoveEntry => "移除条目",
        L10nKey::RemoteNoConnectionDetails => {
            "此窗口是 {machine} 上的工作区，但 tty7 没有它的连接信息了——检查其 SSH 配置或 ~/.ssh/config 条目是否还在。"
        }
        L10nKey::RemoteThisComputer => "本机",
        L10nKey::RemoteProfileGone => "已删除的配置",
        L10nKey::RemoteRestartTitle => "重启“{machine}”上的 tty7 server？",
        L10nKey::RemoteRestartBody => {
            "这会结束 {machine} 上的所有 shell，包括此窗口没显示的。工作区和布局会保留，并以全新的 shell 恢复。"
        }
        L10nKey::RemoteReplaceBody => {
            "tty7 会在 {machine} 上安装匹配的 server 并启动它。\n\
             \n\
             {machine} 上运行的所有会话都会结束，包括此窗口未连接的会话。"
        }
        L10nKey::RemoteRestartFailedTitle => "“{machine}”上的 tty7 server 未被重启",
        L10nKey::RemoteRestartFailedBody => {
            "{error}\n\
             \n\
             那里仍在运行的会话用的还是旧版本。如果它们已经结束，重新连接就会启动此版本的 server。"
        }
        L10nKey::RemoteHostUnreachable => "无法连接到 {machine}：{error}",
        L10nKey::RemoteInstallTitle => "在“{machine}”上安装 tty7 server？",
        L10nKey::RemoteInstallDetail => {
            "tty7 会将其 server 二进制文件写入 {machine}，以便本机可以在那里托管\
             工作区。{machine} 上的其他内容不会被修改，也不会使用 sudo。\n\
             \n\
             {path_label}\u{2003}{path}\n\
             {version_label}\u{2003}{version}\n\
             {size_label}\u{2003}{size}\n\
             {from_label}\u{2003}{from}\n\
             {sha_label}\u{2003}{sha256}\n\
             \n\
             {silent_upgrades}"
        }
        L10nKey::RemoteInstallPathLabel => "路径",
        L10nKey::RemoteInstallVersionLabel => "版本",
        L10nKey::RemoteInstallSizeLabel => "大小",
        L10nKey::RemoteInstallFromLabel => "来源",
        L10nKey::RemoteInstallShaLabel => "SHA-256",
        L10nKey::RemoteInstallSilentUpgrades => "此后在该机器上的升级将静默安装。",
        L10nKey::RemoteInstallBytes => "字节",
        L10nKey::RemoteMismatchTitle => "更新“{machine}”上的 tty7 server？",
        L10nKey::RemoteMismatchDetail => {
            "{machine} 上正在运行的 server（{running}）与此客户端（{wanted}）的协议不兼容。匹配的 server 已经安装，但你的会话仍在当前运行的 server 上。\n\n{replace_server}\u{2003}换成 {wanted}，并结束它托管的所有会话。\n{cancel}\u{2003}保持 {machine} 现状。此窗口不会连接。"
        }
        L10nKey::RemoteMismatchReplaceServer => "更新 server",
        L10nKey::RemoteMismatchDowngradeServer => "替换 server",
        L10nKey::RemoteMismatchUnknownBuild => "未知构建",
        L10nKey::RemoteMismatchUnknownBuildFromExe => "未知构建（来自 {exe}）",
        L10nKey::RemoteServerOutdated => {
            "{machine} 上的 tty7 server 太旧（{build}），当前这份 tty7 连不上它。\
             更新它才能连接。"
        }
        L10nKey::RemoteServerTooNew => {
            "{machine} 上的 tty7 server（{build}）比当前这份 tty7 还新。\
             请更新本机的 tty7，或把那边的 server 替换成匹配的版本。"
        }
        L10nKey::RemoteDaemonStartFailed => "无法启动 tty7 本地 server：{error}",
        L10nKey::RemoteDaemonUnreachable => "无法连接到 tty7 本地 server：{error}",
        L10nKey::RemoteDaemonTooOld => {
            "本机的 tty7 本地 server 版本较旧，无法重启 {machine} 上的 server。请退出 tty7（这会停止本地 server）再打开，然后重试。"
        }
        L10nKey::RemoteProfileMissing => "该已保存的 SSH 主机配置已不存在",
        L10nKey::RemoteAliasMissing => "“{alias}”已不再位于 ~/.ssh/config 中",
        L10nKey::RemoteWslNoSsh => "WSL 工作区没有 SSH 连接",
        L10nKey::RemoteLocalStdioNoSsh => "本地 --stdio 工作区没有 SSH 连接",
        L10nKey::RemoteHostNotTty7 => "{machine} 已响应，但它不是 tty7 server：{error}",
        L10nKey::RemoteWorkspaceListFailed => "已连接到 {machine}，但其工作区列表获取失败：{error}",
        L10nKey::RemoteServerRestartFailed => "无法重启 {machine} 上的 tty7 server：{error}",
        L10nKey::RemoteNoRouteToHost => "tty7 已无法到达 {machine}",
        L10nKey::RemoteMachineTreeUnexpectedReply => "获取机器树时收到意外响应：{reply}",
        L10nKey::RemoteMismatchVersionFromExe => "{version}（来自 {exe}）",
        L10nKey::SwitcherThisComputer => "本机",
        L10nKey::SwitcherStartingServer => "正在启动 tty7 server…",
        L10nKey::SwitcherDownloadingServerWithTotal => "正在下载 tty7 server… {done} / {total}",
        L10nKey::SwitcherDownloadingServerNoTotal => "正在下载 tty7 server… {done}",
        L10nKey::SwitcherCopyingServer => "正在复制 tty7 server… {done} / {total}",
        L10nKey::SwitcherThisWindow => "当前窗口",
        L10nKey::SwitcherOpen => "已打开",
        L10nKey::SwitcherOffline => "离线",
        L10nKey::SwitcherDisconnect => "断开连接",
        L10nKey::SwitcherEditHost => "编辑主机…",
        L10nKey::SwitcherSaveAsHost => "保存为 SSH 主机…",
        L10nKey::SwitcherOpenInNewWindow => "在新窗口中打开",
        L10nKey::SwitcherRename => "重命名…",
        L10nKey::SwitcherPickAWorkspace => "选一个工作区查看它的标签页。",
        L10nKey::SwitcherNoTabs => "这个工作区没有标签页。",
        L10nKey::SwitcherNoTabMatch => "没有匹配的标签页。",
        L10nKey::SwitcherTabsAfterOpening => "打开这个工作区后才能看到它的标签页。",
        L10nKey::SwitcherOpenToManage => "打开这个工作区后才能重命名或停止它。",
        L10nKey::SwitcherOrphanPanes => "后台窗格——仍在运行、但不属于任何窗口的 shell：",
        L10nKey::SwitcherTabCount => "{n} 个标签页",
        L10nKey::SwitcherTabCountOne => "1 个标签页",
        L10nKey::SwitcherActiveTab => "当前",
        L10nKey::SwitcherHoldToSwitch => "按 Tab 移动 · 松开切换",
        L10nKey::SwitcherTabToCrossColumns => "按 Tab 换到另一列",
        L10nKey::SwitcherHintNavigate => "导航",
        L10nKey::SwitcherHintOpen => "打开",
        L10nKey::SwitcherHintNewWindow => "新窗口",
        L10nKey::SwitcherLocalHost => "本机",
        L10nKey::SwitcherConnectingTo => "正在连接 {machine}…",
        L10nKey::SwitcherFormName => "名字",
        L10nKey::SwitcherFormHost => "主机",
        L10nKey::SwitcherFormNamePlaceholder => "工作区名称",
        L10nKey::SwitcherFormBack => "返回",
        L10nKey::SwitcherFormCreate => "创建",
        L10nKey::SwitcherFormPickHint => "↑↓ 选择 · Enter 确定 · Esc 收起",
        L10nKey::SshPromptPasswordFor => "{user}@{host} 的密码",
        L10nKey::SshPromptPassphraseFor => "{key_path} 的口令",
        L10nKey::SshPromptTwoFactor => "双因素认证",
        L10nKey::SshPromptUnknownHost => "未知主机 {host}",
        L10nKey::SshPromptHostKeyChanged => "主机密钥已更改——可能存在中间人攻击",
        L10nKey::SshPromptHostKeyChangedBody => "主机密钥与之前信任的密钥不同，这可能是一次攻击。",
        L10nKey::SshPromptConnect => "连接",
        L10nKey::SshPromptUnlock => "解锁",
        L10nKey::SshPromptSubmit => "提交",
        L10nKey::IoDenied => "没有权限。",
        L10nKey::IoGone => "文件或目录已不存在。",
        L10nKey::IoNoSpace => "磁盘没有空间了。",
        L10nKey::IoReadOnly => "目标位置是只读的。",
        L10nKey::IoBusy => "它正被其他程序占用。",
        L10nKey::IoTimedOut => "对方没有在规定时间内响应。",
        L10nKey::TreeWindowOpenedEmpty => {
            "tty7 server 尚未返回这个窗口的标签页，所以窗口暂时是空的。会话没有丢失，server 响应后会自动恢复；如果长时间没有恢复，请在“设置 → 关于”中重启本机 tty7 server，或更新对应的远程 server。"
        }
        L10nKey::CmdGroupTabsPanes => "标签页与窗格",
        L10nKey::CmdGroupWorkspaces => "工作区",
        L10nKey::CmdGroupView => "视图",
        L10nKey::CmdGroupTerminal => "终端",
        L10nKey::CmdGroupSsh => "SSH",
        L10nKey::CmdGroupAgents => "Agents",
        L10nKey::CmdGroupApplication => "应用",
        L10nKey::CmdNewTab => "新标签页",
        L10nKey::CmdNewWindow => "新建窗口",
        L10nKey::CmdRenameTab => "重命名标签页…",
        L10nKey::CmdSplitRight => "向右分屏",
        L10nKey::CmdSplitDown => "向下分屏",
        L10nKey::CmdZoomPane => "缩放窗格",
        L10nKey::CmdNextPane => "下一窗格",
        L10nKey::CmdPreviousPane => "上一窗格",
        L10nKey::CmdFocusPaneLeft => "聚焦左侧窗格",
        L10nKey::CmdFocusPaneRight => "聚焦右侧窗格",
        L10nKey::CmdFocusPaneUp => "聚焦上方窗格",
        L10nKey::CmdFocusPaneDown => "聚焦下方窗格",
        L10nKey::CmdResizePaneLeft => "向左调整窗格",
        L10nKey::CmdResizePaneRight => "向右调整窗格",
        L10nKey::CmdResizePaneUp => "向上调整窗格",
        L10nKey::CmdResizePaneDown => "向下调整窗格",
        L10nKey::CmdSwapPaneNext => "与下一窗格交换",
        L10nKey::CmdSwapPanePrevious => "与上一窗格交换",
        L10nKey::CmdNextTab => "下一标签页",
        L10nKey::CmdPreviousTab => "上一标签页",
        L10nKey::CmdMoveTabUp => "标签页上移",
        L10nKey::CmdMoveTabDown => "标签页下移",
        L10nKey::CmdRecentTabSwitcher => "最近标签页切换器",
        L10nKey::CmdRecentTabSwitcherReverse => "最近标签页切换器（反向）",
        L10nKey::CmdCopyWorkingDirectory => "复制工作目录",
        L10nKey::CmdCopySessionId => "复制会话 ID",
        L10nKey::CmdNewAgentTab => "新建 Agent 标签页",
        L10nKey::CmdNextAgentNeedingAttention => "Agent：跳到下一个需要处理的 agent",
        L10nKey::AppNoAgentNeedsAttention => "已打开的窗口中没有需要你处理的 agent。",
        L10nKey::AgentWaitingFor => "已等待 {age}",
        L10nKey::AgeUnderMinute => "不到 1 分钟",
        L10nKey::AgeMinutes => "{count} 分钟",
        L10nKey::AgeHours => "{count} 小时",
        L10nKey::AgeDays => "{count} 天",
        L10nKey::CmdForkSession => "Fork 会话",
        L10nKey::CmdMarkTabAsUnread => "将标签页标记为未读",
        L10nKey::CmdHibernateTab => "休眠标签页",
        L10nKey::CmdClosePaneTab => "关闭窗格/标签页",
        L10nKey::CmdCloseWindow => "关闭窗口",
        L10nKey::CmdCloseOtherTabs => "关闭其他标签页",
        L10nKey::CmdCloseTabsBelow => "关闭下方标签页",
        L10nKey::CmdReopenClosedTab => "重新打开已关闭标签页",
        L10nKey::CmdNewWorkspace => "新建工作区…",
        L10nKey::CmdSwitchWorkspace => "切换工作区…",
        L10nKey::CmdRenameWorkspace => "重命名工作区…",
        L10nKey::CmdStopWorkspace => "停止工作区…",
        L10nKey::CmdDeleteWorkspace => "删除工作区…",
        L10nKey::CmdResetFontSize => "重置字号",
        L10nKey::CmdEnterFullScreen => "进入全屏",
        L10nKey::CmdClearScrollback => "清除回滚内容",
        L10nKey::CmdFindInTerminal => "在终端中查找…",
        L10nKey::CmdToggleComposer => "切换消息输入框",
        L10nKey::ComposerModeDefault => "编辑前询问",
        L10nKey::ComposerModeAcceptEdits => "自动接受编辑",
        L10nKey::ComposerModePlan => "计划模式",
        L10nKey::ComposerModeBypass => "跳过权限确认",
        L10nKey::ComposerModeAuto => "自动模式",
        L10nKey::ComposerModeTip => "权限模式  ·  Shift+Tab 切换",
        L10nKey::ComposerModel => "模型",
        L10nKey::ComposerModelTip => "切换模型",
        L10nKey::ComposerModelDefault => "默认（推荐）",
        L10nKey::ComposerEffort => "推理强度",
        L10nKey::ComposerEffortTip => "推理强度  ·  点击切换",
        L10nKey::ComposerPlaceholder => "给 {agent} 发消息   ·   @ 文件   / 命令",
        L10nKey::ComposerPlaceholderFiles => "给 {agent} 发消息   ·   @ 文件",
        L10nKey::ComposerSendTip => "发送（Enter）  ·  Shift+Enter 换行",
        L10nKey::ComposerAttach => "附加文件或图片",
        L10nKey::ComposerMenuCommands => "命令",
        L10nKey::ComposerCmdProject => "项目命令",
        L10nKey::ComposerCmdUser => "用户命令",
        L10nKey::ComposerAgentAsking => "{agent} 正在等你回答，请先在终端里回答再发送。",
        L10nKey::CmdFindNext => "查找下一个",
        L10nKey::CmdFindPrevious => "查找上一个",
        L10nKey::CmdCopy => "复制",
        L10nKey::CmdPaste => "粘贴",
        L10nKey::CmdAlternatePaste => "粘贴（全屏程序中除外）",
        L10nKey::CmdSshManageProfiles => "SSH：管理主机配置…",
        L10nKey::CmdSshReconnect => "SSH：重新连接",
        L10nKey::CmdSshPortForwarding => "SSH：端口转发",
        L10nKey::CmdSettings => "设置…",
        L10nKey::CmdKeyboardShortcuts => "快捷键",
        L10nKey::CmdAboutTty7 => "关于 tty7",
        L10nKey::CmdCheckForUpdates => "检查更新…",
        L10nKey::CmdDocumentation => "文档",
        L10nKey::CmdJoinDiscord => "加入 Discord",
        L10nKey::CmdReportIssue => "报告问题…",
        L10nKey::CmdQuitTty7 => "退出 tty7",
        L10nKey::AppRestartServerTitle => "重启 tty7 server？",
        L10nKey::AppRestartServerFailed => "无法重启 tty7 server：{error}",
        L10nKey::AppRestartServerMismatchDetail => {
            "tty7 server 的协议为 {protocol}（构建 v{build}），此应用为 {ours}，无法获取标签页。\n\n退出：什么都不变，tty7 server 和 shell 继续运行。\n重启：标签页会以全新的 shell 恢复，当前运行的程序会被终止。"
        }
        L10nKey::AppRestartServerDialectDetail => {
            "tty7 server 的控制协议版本为 v{dialect}（构建 v{build}），此应用为 v{ours}，版本不匹配，每个窗口都会打开为空。\n\n退出：什么都不变，tty7 server 和 shell 继续运行。\n重启：标签页会以全新的 shell 恢复，当前运行的程序会被终止。"
        }
        L10nKey::AppRestartServerDialectNewerDetail => {
            "tty7 server 的控制协议版本为 v{dialect}（构建 v{build}），此应用为 v{ours}，版本不匹配，每个窗口都会打开为空。\n\n退出并安装更新的版本（推荐）：所有 shell 都会保留。\n重启：标签页会以全新的 shell 恢复，当前运行的程序会被终止。"
        }
        L10nKey::AppRestartServerOldDetail => {
            "当前运行的 tty7 server 版本过旧，不支持版本握手，此应用无法判断它的协议版本。\n\n退出：什么都不变，tty7 server 和 shell 继续运行。\n重启：标签页会以全新的 shell 恢复，当前运行的程序会被终止。"
        }
        L10nKey::AppRestart => "重启",
        L10nKey::AppRestartServerNoServer => {
            "{label} 没有自己的 tty7 server 可重启——它是本机通过 --stdio 运行的程序。请改为停止其工作区。"
        }
        L10nKey::AppRestartServerBody => {
            "这会结束本机上所有 shell。标签页和布局会保留，并以全新的 shell 重新打开。"
        }
        L10nKey::ConfigQuarantinedStartup => {
            "config.json 无法解析。tty7 正以默认设置运行，原内容已保留为旁边的 config.json.corrupt。修好后会自动重载；在此之前，设置里的更改不会保存。"
        }
        L10nKey::ConfigQuarantinedReload => {
            "修改后的 config.json 无法解析。tty7 保留了当前在用的设置，文件内容另存为旁边的 config.json.corrupt。修好后会自动重载；在此之前保存设置会覆盖它。"
        }
        L10nKey::ConfigUnreadableStartup => {
            "config.json 读取失败。tty7 正以默认设置运行，文件原样保留。修好权限或内容后会自动重载；在此之前，设置里的更改不会保存。"
        }
        L10nKey::ConfigUnreadableReload => {
            "config.json 读取失败。tty7 保留了当前在用的设置，文件也原样保留。修好权限或内容后会自动重载；在此之前保存设置会覆盖它。"
        }
        L10nKey::AppReopenTabFailed => "无法重新打开标签页：没有启动终端",
        L10nKey::AppOpenTerminalFailed => "无法打开终端：{error}",
        L10nKey::AppTabsNotRestored => "上次的 {count} 个标签页没能重新打开",
        L10nKey::AppFullscreenEntered => "已进入全屏，按 {key} 退出",
        L10nKey::AppFullscreenEnteredNoKey => "已进入全屏，窗口按钮在退出前会一直隐藏",
        L10nKey::LaunchWorkspacesLeftRunning => {
            "只恢复了这个窗口——还有 {count} 个工作区在后台运行，可从侧边栏重新打开。"
        }
        L10nKey::AppSshConnectionFailed => "SSH 连接失败：{error}",
        L10nKey::AppSshReconnectFailed => "SSH 重新连接失败：{error}",
        L10nKey::AppSplitPaneFailed => "无法拆分窗格：{error}",
        L10nKey::PaneDragHandleTooltip => "拖动可把这个窗格挪到别处",
        L10nKey::AppPaneNoCodingAgent => "此窗格未运行编码 agent",
        L10nKey::AppForkNoCommand => "tty7 没有用于 {name} 的 fork 命令",
        L10nKey::AppForkLocalOnly => "无法在 SSH 或 WSL 会话里 fork {name} 会话",
        L10nKey::AppForkNoSessionId => {
            "tty7 尚未在此窗格中看到 {name} 的会话 ID，请在“设置 → 集成”中安装其 hook"
        }
        L10nKey::AppForkSessionIdNotToken => "{name} 的会话 ID 格式异常（不是单个连续字符串）",
        L10nKey::AppForkMidTurn => "{name} 正在处理中——fork 不会包含进行中的这一轮",
        L10nKey::AppCmdAgentLaunchTitle => "Agent：{name}",
        L10nKey::AppNoAgentOnPath => "在本机 PATH 中未找到编码 agent",
        L10nKey::AppNoAgentSeenHere => {
            "此工作区还没有运行过编码 agent——手动启动一次后，这里就会列出它"
        }
        L10nKey::AppAgentLaunchSaved => "{name} 以后将以此命令启动：{command}",
        L10nKey::AppAgentLaunchArgsUnknown => "{name} 未报告其启动参数",
        L10nKey::AppPlaceholderDescription => "描述",
        L10nKey::AppPlaceholderLoginShell => "登录 shell",
        L10nKey::AppPlaceholderNone => "无",
        L10nKey::AppThemeColorBackground => "背景",
        L10nKey::AppThemeColorForeground => "前景",
        L10nKey::AppThemeColorAccent => "强调色",
        L10nKey::AppThemeColorCursor => "光标",
        L10nKey::AppThemeColorSelection => "选区",
        L10nKey::AppThemeAnsiBlack => "黑色",
        L10nKey::AppThemeAnsiRed => "红色",
        L10nKey::AppThemeAnsiGreen => "绿色",
        L10nKey::AppThemeAnsiYellow => "黄色",
        L10nKey::AppThemeAnsiBlue => "蓝色",
        L10nKey::AppThemeAnsiMagenta => "洋红色",
        L10nKey::AppThemeAnsiCyan => "青色",
        L10nKey::AppThemeAnsiWhite => "白色",
        L10nKey::AppThemeAnsiBrightBlack => "亮黑色",
        L10nKey::AppThemeAnsiBrightRed => "亮红色",
        L10nKey::AppThemeAnsiBrightGreen => "亮绿色",
        L10nKey::AppThemeAnsiBrightYellow => "亮黄色",
        L10nKey::AppThemeAnsiBrightBlue => "亮蓝色",
        L10nKey::AppThemeAnsiBrightMagenta => "亮洋红色",
        L10nKey::AppThemeAnsiBrightCyan => "亮青色",
        L10nKey::AppThemeAnsiBrightWhite => "亮白色",
        L10nKey::AppAgentHooksThisComputer => "本机",
        L10nKey::AppAgentHooksRemoteMachine => "远程机器",
        L10nKey::AppAgentHooksNoHomeDir => {
            "tty7 无法确定这台计算机的主目录，因此没有可安装的位置。"
        }
        L10nKey::AppAgentHooksOffline => {
            "未连接到这台机器，因此无法读取或写入其 agent 配置。请在其上打开一个工作区后再回来。"
        }
        L10nKey::AppAgentHooksHomeDirUnresolved => "无法解析主目录",
        L10nKey::AppAgentHooksInstalled => "已安装",
        L10nKey::AppAgentHooksInstalledEnableCodexThere => {
            "已安装，还需要在那台机器上运行一次 `codex features enable hooks`"
        }
        L10nKey::AppAgentHooksInstalledCodexEnableFailed => {
            "已安装，但无法运行 `codex features enable hooks`（{error}）—— 请手动运行一次"
        }
        L10nKey::AppAgentHooksRemoved => "已移除",
        L10nKey::AppAgentHooksNothingInstalled => "没有安装任何内容，无需移除",
        L10nKey::AppAgentHooksNoTty7Hooks => "没有找到 tty7 钩子，无需移除",
        L10nKey::AppAgentHooksInstallFailed => "无法安装钩子：{error}",
        L10nKey::AppAgentHooksRemoveFailed => "无法移除钩子：{error}",
        L10nKey::AppKeybindingDisplacedNote => {
            "{action} 占用了原属于 {previous} 的快捷键，{previous} 现在没有快捷键了。"
        }
        L10nKey::AppLocalServerName => "本地 server",
        L10nKey::AppMenuMinimize => "最小化",
        L10nKey::AppMenuZoom => "缩放",
        L10nKey::SwitcherStatusTakenOver => "已被接管",
        L10nKey::SettingsFontDefault => "默认（匹配主字体）",
        L10nKey::SettingsUiFontDefault => "默认（系统界面字体）",
        L10nKey::ForwardDescriptionPlaceholder => "用途说明",
        L10nKey::SettingsShellDefaultLoginShell => "你的登录 shell",
        L10nKey::Replace => "覆盖",
        L10nKey::SettingsDaemonStaleDescInPlace => {
            "tty7 已更新。server 可原地替换，shell 不受影响；只有内置 SSH 的窗格会断开。"
        }
        L10nKey::AppRestartServerAfterUpdateDetail => {
            "tty7 已更新到 {ours}，但 server 仍在运行 {build}。它可以原地更新到当前版本并保留 shell，但用 tty7 内置 SSH 客户端的窗格会断开。现在更新，或稍后在设置中进行。"
        }
        L10nKey::AppRestartServerBodyInPlace => {
            "tty7 server 会原地更新到当前版本：shell 继续运行，窗口稍后自动重连。用 tty7 内置 SSH 客户端的窗格除外——那些连接会断开，需要重新打开。"
        }
        L10nKey::PaneRestoredScreenBanner => {
            "已恢复的画面：下面是新的 shell，上面的内容都已不在运行"
        }
        L10nKey::PaneTitleDisconnected => "{title} — 已断开",
        L10nKey::PaneTitleProcessExited => "{title} — 进程已退出",
        L10nKey::LoopbackForwardFailed => "无法转发 :{port}——{error}",
        L10nKey::TrayTooltipAgents => "tty7：{parts}",
        L10nKey::TrayAgentSep => "、",
        L10nKey::CursorShapeBlock => "块状",
        L10nKey::CursorShapeBar => "竖线",
        L10nKey::CursorShapeUnderline => "下划线",
        L10nKey::PromptCursorShapeFollow => "跟随",
        L10nKey::CmdUpdateRemoteServer => "更新“{machine}”上的 tty7 server…",
        L10nKey::CmdUpdateRemoteServerSubtitle => {
            "在那台机器上重新安装当前版本的 server；结束其上所有会话"
        }
        L10nKey::RemoteUpdateBody => {
            "tty7 会在 {machine} 上安装当前版本的 server（即使那里的 server 版本号相同也会覆盖），然后重启它。\n\n{machine} 上运行的所有会话都会结束，包括此窗口未连接的会话。"
        }
        L10nKey::RemoteUpdateNeedsLocalServer => {
            "本机的 tty7 server 版本过旧，无法更新 {machine} 上的 server。请先更新本机的 server，再重试。"
        }
        L10nKey::AppMenuAbout => "关于 tty7",
        L10nKey::AppMenuCheckForUpdates => "检查更新…",
        L10nKey::AppMenuSettings => "设置…",
        L10nKey::AppMenuServices => "服务",
        L10nKey::AppMenuHideApp => "隐藏 tty7",
        L10nKey::AppMenuHideOthers => "隐藏其他",
        L10nKey::AppMenuShowAll => "显示全部",
        L10nKey::AppMenuQuit => "退出 tty7",
        L10nKey::AppMenuFile => "文件",
        L10nKey::AppMenuEdit => "编辑",
        L10nKey::AppMenuView => "视图",
        L10nKey::AppMenuWindow => "窗口",
        L10nKey::AppMenuHelp => "帮助",
        L10nKey::AppMenuNewTab => "新标签页",
        L10nKey::AppMenuNewWorkspace => "新工作区…",
        L10nKey::AppMenuSplitRight => "向右分屏",
        L10nKey::AppMenuSplitLeft => "向左分屏",
        L10nKey::AppMenuSplitDown => "向下分屏",
        L10nKey::AppMenuSplitUp => "向上分屏",
        L10nKey::AppMenuRenameTab => "重命名标签页…",
        L10nKey::AppMenuCopyWorkingDirectory => "复制工作目录",
        L10nKey::AppMenuCopySessionId => "复制会话 ID",
        L10nKey::AppMenuForkSession => "Fork 会话",
        L10nKey::AppMenuSaveAgentLaunchArgs => "将当前启动参数设为默认",
        L10nKey::AppMenuClosePaneTab => "关闭",
        L10nKey::AppMenuCloseOtherTabs => "关闭其他标签页",
        L10nKey::AppMenuCloseTabsBelow => "关闭下方标签页",
        L10nKey::AppMenuReopenClosedTab => "重新打开已关闭的标签页",
        L10nKey::AppMenuRenameWorkspace => "重命名工作区…",
        L10nKey::AppMenuStopWorkspace => "停止工作区…",
        L10nKey::AppMenuDeleteWorkspace => "删除工作区…",
        L10nKey::AppMenuUndo => "撤销",
        L10nKey::AppMenuRedo => "重做",
        L10nKey::AppMenuCut => "剪切",
        L10nKey::AppMenuCopy => "复制",
        L10nKey::AppMenuPaste => "粘贴",
        L10nKey::AppMenuSelectAll => "全选",
        L10nKey::AppMenuFind => "查找…",
        L10nKey::AppMenuFindNext => "查找下一个",
        L10nKey::AppMenuFindPrevious => "查找上一个",
        L10nKey::AppMenuIncreaseFontSize => "增大字号",
        L10nKey::AppMenuDecreaseFontSize => "减小字号",
        L10nKey::AppMenuResetFontSize => "重置字号",
        L10nKey::AppMenuLeftSidebar => "左侧边栏",
        L10nKey::AppMenuRightPanel => "右侧面板",
        L10nKey::AppMenuFocusNextPane => "聚焦下一个窗格",
        L10nKey::AppMenuFocusPreviousPane => "聚焦上一个窗格",
        L10nKey::AppMenuZoomPane => "缩放窗格",
        L10nKey::AppMenuClearScrollback => "清除回滚内容",
        L10nKey::AppMenuDocumentation => "tty7 文档",
        L10nKey::AppMenuKeyboardShortcuts => "快捷键",
        L10nKey::AppMenuJoinDiscord => "加入 Discord",
        L10nKey::AppMenuReportIssue => "报告问题…",
        L10nKey::AppMenuRestartServer => "重启 tty7 server…",
        L10nKey::WindowUntitled => "未命名",
        L10nKey::TrayShowTty7 => "显示 tty7",
        L10nKey::TrayNotifications => "通知",
        L10nKey::TrayAgentNeedsInput => "需要输入",
        L10nKey::AgentStatusWorking => "运行中",
        L10nKey::AgentStatusWaiting => "需要输入",
        L10nKey::AgentStatusDone => "已完成",
        L10nKey::NotifyCommandFinished => "命令运行完成，用时 {secs} 秒",
        L10nKey::NotifyCommandFinishedWithCommand => "{command} 已完成，用时 {secs} 秒",
        L10nKey::NotifyAgentFinished => "已完成，用时 {secs} 秒",
        L10nKey::NotifyAgentWaiting => "等待你的输入",
        L10nKey::NotifyTurnFinished => "本轮已完成",
        L10nKey::TabTooltipMore => "更多",
        L10nKey::TabTooltipShowSidebar => "显示侧栏",
        L10nKey::TabTooltipHideSidebar => "隐藏侧栏",
        L10nKey::TabTooltipHideDetailPanel => "隐藏详情面板",
        L10nKey::TabTooltipShowDetailPanel => "显示详情面板",
        L10nKey::TabTooltipZoomed => "窗格已缩放，其他窗格已隐藏",
        L10nKey::TabMenuLocalShells => "本地",
        L10nKey::TabMenuAddHost => "添加 SSH 主机…",
        L10nKey::TabMenuAllHosts => "所有 SSH 主机…",
        L10nKey::TabMenuSplitHint => "按住 {key} 可分屏打开",
        L10nKey::TabUnnamedShell => "终端 {n}",
        L10nKey::ShellDefault => "默认",
        L10nKey::SidebarUngroupedGroup => "未分组",
        L10nKey::SidebarMoveToGroup => "移到分组",
        L10nKey::SidebarNewGroup => "新建分组…",
        L10nKey::SidebarRemoveFromGroup => "移出分组",
        L10nKey::SidebarNewGroupName => "新建分组",
        L10nKey::SidebarRenameGroup => "重命名分组",
        L10nKey::SidebarPinGroup => "固定分组",
        L10nKey::SidebarUnpinGroup => "取消固定",
        L10nKey::SidebarGroupNewTab => "新建标签页",
        L10nKey::SidebarSetGroupFolder => "设置文件夹…",
        L10nKey::SidebarUseCurrentTabFolder => "使用当前标签页的文件夹",
        L10nKey::SidebarClearGroupFolder => "清除文件夹",
        L10nKey::SidebarDeleteGroup => "删除分组",
        L10nKey::SidebarDropToPin => "拖到此处固定",
        L10nKey::TabContextCloseTab => "关闭标签页",
        L10nKey::TerminalContextClear => "清屏",
        L10nKey::Search => "搜索",
        L10nKey::TabContextMarkUnread => "标记为未读",
        L10nKey::TabContextHibernate => "休眠",
        L10nKey::TabContextWake => "唤醒",
        L10nKey::TabTooltipAsleep => "已休眠，选中即可唤醒",
        L10nKey::TabWakeFailed => "无法唤醒标签页：其中的窗格都无法启动",
        // ---- The right panel's GitHub tab ----
        L10nKey::CmdCopyRaw => "原样复制",
        L10nKey::SettingsSearchCopyJoinWrappedKeywords => {
            "剪贴板 复制 折行 合并 clipboard copy wrap unwrap join lines claude raw"
        }
        L10nKey::SettingsCopyJoinWrapped => "复制时合并折行",
        L10nKey::SettingsCopyJoinWrappedDesc => {
            "在 Claude Code 窗格中，将 Claude 在窗格边缘折行的内容重新合并，段落粘贴为一行。⌘⌥C 或“原样复制”按显示内容复制"
        }
    })
}

pub fn translate_variant_zh(key: L10nKey, branch: &'static str) -> Option<&'static str> {
    let res = match (key, branch) {
        (L10nKey::SettingsMatchCount, "zero") => "没有结果",
        (L10nKey::SettingsMatchCount, "one") => "1 个结果",
        (L10nKey::SettingsMatchCount, "other") => "{count} 个结果",
        (L10nKey::SettingsRestoreChanged, "zero") => "恢复修改",
        (L10nKey::SettingsRestoreChanged, "one") => "恢复 1 项修改",
        (L10nKey::SettingsRestoreChanged, "other") => "恢复 {count} 项修改",
        (L10nKey::SettingsMoreHosts, "zero") => "没有更多主机",
        (L10nKey::SettingsMoreHosts, "one") => "还有 1 台主机",
        (L10nKey::SettingsMoreHosts, "other") => "还有 {count} 台主机",
        (L10nKey::SettingsMoreAgents, "zero") => "没有更多 Agent",
        (L10nKey::SettingsMoreAgents, "one") => "还有 1 个可用 Agent",
        (L10nKey::SettingsMoreAgents, "other") => "还有 {count} 个可用 Agent",
        (L10nKey::SettingsImportSummary, "zero") => {
            "没有新主机——更新 {updated} 个，{unchanged} 个已是最新"
        }
        (L10nKey::SettingsImportSummary, "one") => {
            "新增 1 个主机——更新 {updated} 个，{unchanged} 个已是最新"
        }
        (L10nKey::SettingsImportSummary, "other") => {
            "新增 {count} 个主机——更新 {updated} 个，{unchanged} 个已是最新"
        }
        (L10nKey::SettingsImportIgnored, "zero") => "文件里的每个选项在 tty7 中都有对应设置。",
        (L10nKey::SettingsImportIgnored, "one") => {
            "有 1 个选项在 tty7 中没有对应设置，仍留在文件里：{options}"
        }
        (L10nKey::SettingsImportIgnored, "other") => {
            "有 {count} 个选项在 tty7 中没有对应设置，仍留在文件里：{options}"
        }
        (L10nKey::SettingsOfflineMachines, "zero") => {
            "还有 0 台已保存的机器未连接——在其中一台上打开工作区，即可在那台机器上安装 hook。"
        }
        (L10nKey::SettingsOfflineMachines, "one") => {
            "还有 1 台已保存的机器未连接——在那台机器上打开工作区，即可在那里安装 hook。"
        }
        (L10nKey::SettingsOfflineMachines, "other") => {
            "还有 {count} 台已保存的机器未连接——在其中一台上打开工作区，即可在那台机器上安装 hook。"
        }
        (L10nKey::CloseTabsTitle, "one") => "关闭 1 个标签页？",
        (L10nKey::CloseTabsTitle, "other") => "关闭 {count} 个标签页？",
        (L10nKey::AppTabsNotRestored, "one") => "上次的 1 个标签页没能重新打开",
        (L10nKey::AppTabsNotRestored, "other") => "上次的 {count} 个标签页没能重新打开",
        (L10nKey::LaunchWorkspacesLeftRunning, "one") => {
            "只恢复了这个窗口——还有 1 个工作区在后台运行，可从侧边栏重新打开。"
        }
        (L10nKey::LaunchWorkspacesLeftRunning, "other") => {
            "只恢复了这个窗口——还有 {count} 个工作区在后台运行，可从侧边栏重新打开。"
        }
        (L10nKey::HomeTimeMinutesAgo, "one") => "1 分钟前",
        (L10nKey::HomeTimeMinutesAgo, "other") => "{count} 分钟前",
        (L10nKey::HomeTimeHoursAgo, "one") => "1 小时前",
        (L10nKey::HomeTimeHoursAgo, "other") => "{count} 小时前",
        (L10nKey::HomeTimeDaysAgo, "one") => "1 天前",
        (L10nKey::HomeTimeDaysAgo, "other") => "{count} 天前",
        (L10nKey::HomeTimeWeeksAgo, "one") => "1 周前",
        (L10nKey::HomeTimeWeeksAgo, "other") => "{count} 周前",
        (L10nKey::HomeTimeMonthsAgo, "one") => "1 个月前",
        (L10nKey::HomeTimeMonthsAgo, "other") => "{count} 个月前",
        (L10nKey::WindowStopShells, "zero") => "其布局和工作目录将被清除。",
        (L10nKey::WindowStopShells, "one") => "1 个正在运行的 shell 将会被终止。",
        (L10nKey::WindowStopShells, "other") => "{count} 个正在运行的 shell 将会被终止。",
        (L10nKey::WindowDeleteShells, "zero") => "其布局和工作目录将被清除。",
        (L10nKey::WindowDeleteShells, "one") => {
            "1 个正在运行的 shell 将会被终止，其布局也将被清除。"
        }
        (L10nKey::WindowDeleteShells, "other") => {
            "{count} 个正在运行的 shell 将会被终止，布局也将被清除。"
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
    fn chinese_covers_every_key() {
        assert_eq!(translate_zh(L10nKey::SearchTabs), Some("搜索标签页…"));
    }
}
