use std::sync::atomic::{AtomicU8, Ordering};

mod en;
mod ja;
mod ru;
mod zh;

use en::{translate_en, translate_variant_en};
use ja::{translate_ja, translate_variant_ja};
use ru::{translate_ru, translate_variant_ru};
use zh::{translate_variant_zh, translate_zh};

pub struct LanguageInfo {
    pub code: &'static str,
    pub label_key: L10nKey,
    pub translate_fn: fn(L10nKey) -> Option<&'static str>,
    pub translate_variant_fn: fn(L10nKey, &'static str) -> Option<&'static str>,
}

pub const SUPPORTED_LANGUAGES: &[LanguageInfo] = &[
    LanguageInfo {
        code: "en",
        label_key: L10nKey::SettingsLanguageEnglish,
        translate_fn: |k| Some(translate_en(k)),
        translate_variant_fn: translate_variant_en,
    },
    LanguageInfo {
        code: "zh-CN",
        label_key: L10nKey::SettingsLanguageChinese,
        translate_fn: translate_zh,
        translate_variant_fn: translate_variant_zh,
    },
    LanguageInfo {
        code: "ja-JP",
        label_key: L10nKey::SettingsLanguageJapanese,
        translate_fn: translate_ja,
        translate_variant_fn: translate_variant_ja,
    },
    LanguageInfo {
        code: "ru-RU",
        label_key: L10nKey::SettingsLanguageRussian,
        translate_fn: translate_ru,
        translate_variant_fn: translate_variant_ru,
    },
];

pub fn find_language(code: &str) -> Option<&'static LanguageInfo> {
    SUPPORTED_LANGUAGES.iter().find(|lang| lang.code == code)
}

pub fn default_language_code() -> &'static str {
    SUPPORTED_LANGUAGES[0].code
}

static CURRENT: AtomicU8 = AtomicU8::new(0);

#[cfg(test)]
thread_local! {
    static TEST_LOCALE: std::cell::Cell<Option<u8>> = const { std::cell::Cell::new(None) };
}

/// Defines the key enum and, with it, the list of every key that exists.
///
/// The parity guard used to walk a hand-maintained copy of this list, so a key
/// added without being added there was checked by nobody. There is exactly one
/// list now, and it is the one the compiler reads.
macro_rules! l10n_keys {
    ($($key:ident),* $(,)?) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum L10nKey { $($key),* }

        impl L10nKey {
            pub(crate) const ALL: &'static [L10nKey] = &[$(L10nKey::$key),*];
        }
    };
}

l10n_keys! {
    SettingsNoMatchesShort,
    SettingsModifiedTitle,
    SettingsMatchCount,
    SettingsNothingModified,
    SettingsKeyThen,
    SettingsStartupRestore,
    SettingsSettingsFile,
    SettingsReveal,
    SettingsThemeModeSystem,
    SettingsThemeModeSystemDesc,
    SettingsThemeModeLightDesc,
    SettingsThemeModeDarkDesc,
    SettingsThemeSlotLightDesc,
    SettingsThemeSlotDarkDesc,
    SettingsThemeSlotDesc,
    SettingsLightThemeLabel,
    SettingsDarkThemeLabel,
    SettingsNoThemesMatch,
    SettingsRestoreChanged,
    SettingsShortcutsHint,
    SettingsShortcutsHintTmux,
    SettingsShortcutConflict,
    SettingsReplace,
    SettingsNoActionsMatch,
    SettingsSearchShortcuts,
    SettingsHostsDesc,
    SettingsAddHost,
    SettingsNoHostsMatch,
    SettingsHostsFromFiles,
    SettingsMoreHosts,
    SettingsShowRecentOnly,
    SettingsShowAll,
    SettingsUnsaved,
    SettingsNever,
    SettingsDefinedIn,
    SettingsConnectInNewTab,
    SettingsCopied,
    SettingsNavMobile,
    SettingsMobileAccess,
    SettingsMobileAccessDesc,
    SettingsMobileStatusStarting,
    SettingsMobileStatusFailed,
    SettingsMobileStartFailed,
    SettingsMobileNoAnswer,
    SettingsMobilePair,
    SettingsMobileShowCode,
    SettingsMobilePairDesc,
    SettingsMobilePairNeedsAccess,
    SettingsMobilePairScan,
    SettingsMobilePairValid,
    SettingsMobileNewCode,
    SettingsMobilePairExpired,
    SettingsMobilePairTried,
    SettingsMobilePairReplaced,
    SettingsMobileCopyCode,
    SettingsMobilePaired,
    SettingsMobilePhones,
    SettingsMobileNoPhones,
    SettingsMobileUnpair,
    SettingsSearchMobileKeywords,
    SettingsCopySshCommand,
    SettingsStoredInTty7,
    SettingsClickAgainToRemove,
    SettingsRemoveHost,
    SettingsEditHost,
    SettingsLive,
    SettingsPressKeysShort,
    SettingsSearchAgents,
    SettingsConnectedMachines,
    SettingsInstalledCount,
    SettingsMachine,
    SettingsMachineLocalDesc,
    SettingsMachineRemoteDesc,
    SettingsAgentsInstalledSummary,
    SettingsNoAgentsInstalled,
    SettingsNoAgentsMatch,
    SettingsMoreAgents,
    SettingsShowInstalledOnly,
    SettingsWorking,
    SettingsUpdateAvailable,
    SettingsRevealHookFile,
    SettingsSaveError,
    SettingsRetrySave,

    SettingsNavGeneral,
    SettingsEditShortcuts,
    SettingsModifiedOnly,
    SettingsModified,
    SettingsSearchResults,
    SettingsOpenSetting,
    SettingsNoModified,
    SettingsTerminalFontGroup,
    SettingsInterfaceFontGroup,
    SettingsUnsavedTitle,
    SettingsUnsavedBody,
    SettingsSaveChanges,
    SettingsThemeDraft,

    SearchTabs,
    SearchSettings,
    FilterHosts,
    SearchTheme,
    Search,
    SearchWorkspacesAndMachines,
    SearchFind,
    SearchMatchCase,
    SearchUseRegex,
    HomeNewTab,
    HomeReopenClosedTab,
    HomeSwitchWorkspace,
    HomeSplitRight,
    HomeSplitDown,
    HomeSettings,
    Reconnect,
    None,
    TryAgain,
    Delete,
    Dismiss,
    StoredPasswordRejected,
    StoredPassphraseRejected,
    Trust,
    Abort,
    HostKeyOverrideMessage,
    Override,
    RememberKeychain,
    Cancel,
    Close,
    QuitStopServerTitle,
    QuitStopServerBody,
    QuitAndStop,
    CloseSshConnectionTitle,
    CloseSshConnectionBody,
    ClosePaneBusyTitle,
    CloseTabBusyTitle,
    CloseBusyCommandBody,
    CloseBusyAgentBody,
    CloseIdleBody,
    CloseTabsTitle,
    CloseTabsBody,
    Keep,
    SettingsNavAppearance,
    SettingsNavTerminal,
    SettingsNavInput,
    SettingsNavSsh,
    SettingsNavAgents,
    SettingsNavKeybindings,
    SettingsNavAbout,
    SettingsWindowTitle,
    Reset,
    Save,
    Connect,
    Download,
    Link,
    SettingsThemeIntroTitle,
    SettingsFontSize,
    SettingsFontSizeDesc,
    SettingsUiFontSize,
    SettingsUiFontSizeDesc,
    SettingsUiFontFamily,
    SettingsUiFontFamilyDesc,
    SettingsLineHeight,
    SettingsLineHeightDesc,
    SettingsFontFamily,
    SettingsFontFamilyDesc,
    SettingsBoldFont,
    SettingsBoldFontDesc,
    SettingsItalicFont,
    SettingsItalicFontDesc,
    SettingsFontLigatures,
    SettingsFontLigaturesDesc,
    SettingsFontThicken,
    SettingsFontThickenDesc,
    SettingsCursor,
    SettingsCursorShape,
    SettingsCursorShapeDesc,
    SettingsPromptCursorShape,
    SettingsPromptCursorShapeDesc,
    SettingsCursorBlink,
    SettingsCursorBlinkDesc,
    SettingsLanguage,
    SettingsLanguageDesc,
    SettingsLanguageEnglish,
    SettingsLanguageChinese,
    SettingsLanguageJapanese,
    SettingsLanguageRussian,
    SettingsSearchLanguageKeywords,
    SettingsTransparency,
    SettingsOpacity,
    SettingsOpacityDesc,
    SettingsBlur,
    SettingsBlurDesc,
    SettingsBlurAutoDesc,
    SettingsBackdrop,
    SettingsBackdropDesc,
    SettingsSearchBackdropKeywords,
    SettingsBackdropAuto,
    SettingsBackdropBlur,
    SettingsBackdropMica,
    SettingsBackdropMicaAlt,
    SettingsBackdropAcrylic,
    SettingsBackdropOff,
    FollowTheme,
    SettingsDimInactivePanes,
    SettingsDimInactivePanesDesc,
    SettingsAutoHideTitlebarButtons,
    SettingsAutoHideTitlebarButtonsDesc,
    SettingsOpenThemesFolder,
    SettingsChangeThemeImage,
    SettingsChooseThemeImage,
    SettingsRemoveThemeImage,
    SettingsImageOpacity,
    SettingsImageOpacityDesc,
    SettingsEditTheme,
    SettingsEditThemeIntro,
    SettingsBackgroundImage,
    SettingsBackgroundImageDesc,
    SettingsAnsiColors,
    SettingsCustomThemes,
    SettingsThemesRejected,
    ThemeDuplicateFailed,
    ThemeSaveFailed,
    OpenInFileManagerFailed,
    ExplorerMenuOpenIn,
    ExplorerMenuOpenHere,
    SettingsDuplicateToEdit,
    SettingsHosts,
    SettingsNothingMatches,
    SettingsInTty7,
    SettingsImportFromSshConfig,
    SettingsNoHostsYet,
    SettingsImportUnreadable,
    SettingsImportNoHosts,
    SettingsImportSummary,
    SettingsImportIgnored,
    SettingsImportMoreOptions,
    SettingsCouldntForgetPassword,
    SettingsSecurity,
    SettingsSecurityIntro,
    SettingsVerifyHostKeys,
    SettingsVerifyHostKeysDesc,
    WarnBeforeClosing,
    SettingsWarnBeforeClosingDesc,
    SettingsNewHost,
    SettingsKeepEditing,
    SettingsHostRequired,
    SettingsPortInvalid,
    SettingsNameHint,
    SettingsHostHint,
    SettingsUserHint,
    SettingsPassword,
    SettingsPasswordHint,
    SettingsKeyPassphrase,
    SettingsPassphraseNeedsKey,
    SettingsCouldntSavePassword,
    SettingsCouldntSavePassphrase,
    SettingsJumpHostUnknown,
    SettingsJumpHostSelf,
    SettingsFwdNeedsBoth,
    SettingsFwdNeedsListen,
    SettingsProxyPortInvalid,
    SettingsDefault,
    SettingsShell,
    SettingsShellIntro,
    SettingsProgram,
    SettingsProgramDesc,
    SettingsArguments,
    SettingsArgumentsDesc,
    SettingsArgumentsInvalid,
    SettingsStartIn,
    SettingsStartInDesc,
    SettingsCustomPath,
    SettingsCustomPathDesc,
    SettingsWdInherit,
    SettingsWdHome,
    SettingsWdCustom,
    SettingsWdPathInvalid,
    SettingsShellFooter,
    SettingsScrolling,
    SettingsScrollback,
    SettingsScrollbackDesc,
    SettingsScrollSpeed,
    SettingsScrollSpeedDesc,
    SettingsSmoothScroll,
    SettingsSmoothScrollDesc,
    SettingsMouse,
    SettingsFocusFollowsMouse,
    SettingsFocusFollowsMouseDesc,
    SettingsHideMouseWhileTyping,
    SettingsHideMouseWhileTypingDesc,
    SettingsMouseZoom,
    SettingsMouseZoomDesc,
    SettingsMouseZoomOff,
    SettingsReportMouseToApps,
    SettingsReportMouseToAppsDesc,
    SettingsMouseRightClick,
    SettingsMouseRightClickDesc,
    SettingsMouseRightClickMenu,
    SettingsMouseRightClickCopyPaste,
    SettingsBell,
    SettingsTerminalBell,
    SettingsTerminalBellDesc,
    SettingsLinks,
    DetectUrls,
    SettingsDetectUrlsDesc,
    ForwardSshLoopbackLinks,
    SettingsForwardSshLoopbackLinksDesc,
    SettingsBellModeOff,
    SettingsBellModeVisual,
    SettingsBellModeAudible,
    SettingsBellModeBoth,
    SettingsSelectionClipboard,
    SettingsSmartSelection,
    SettingsSmartSelectionDesc,
    SettingsCopyOnSelect,
    SettingsCopyOnSelectDesc,
    SettingsTrimTrailingSpaces,
    SettingsTrimTrailingSpacesDesc,
    SettingsKeyboard,
    SettingsOptionAsMeta,
    SettingsOptionAsMetaDesc,
    SettingsAgentsIntro,
    SettingsAgentsIntroDesc,
    SettingsReadingAgentConfig,
    SettingsStatusInstalled,
    SettingsInstall,
    SettingsReinstall,
    SettingsUpdate,
    SettingsUninstall,
    SettingsOfflineMachines,
    SettingsSyncWithSystem,
    SettingsSyncWithSystemDesc,
    SettingsLegiblePalette,
    SettingsLegiblePaletteDesc,
    SettingsSearchLegiblePaletteKeywords,
    SettingsCustom,
    SettingsCustomValue,
    SettingsDark,
    SettingsLight,
    SettingsStartupWindow,
    SettingsStartupWindowDesc,
    SettingsRememberWindowSize,
    SettingsRememberWindowSizeDesc,
    SettingsRestoreLastLayout,
    SettingsRestoreLastLayoutDesc,
    SettingsShowTrayIcon,
    SettingsShowTrayIconDesc,
    SettingsTabs,
    SettingsNewTabPosition,
    SettingsNewTabPositionDesc,
    SettingsConfirmClose,
    SettingsConfirmCloseDesc,
    ConfirmCloseNever,
    ConfirmCloseWhenBusy,
    ConfirmCloseAlways,
    SettingsTabBarPosition,
    SettingsTabBarPositionDesc,
    SettingsSidebarGrouping,
    SettingsSidebarGroupingDesc,
    SettingsNotifications,
    SettingsWindow,
    SettingsNotifyOnCommandFinish,
    SettingsNotifyOnCommandFinishDesc,
    SettingsNotifyThreshold,
    SettingsNotifyThresholdDesc,
    NotifyModeNever,
    NotifyModeUnfocused,
    NotifyModeAlways,
    ProgramNotesDropped,
    SettingsStartupNormal,
    SettingsStartupMaximized,
    SettingsStartupFullscreen,
    SettingsAfterCurrent,
    SettingsAtEnd,
    SettingsTop,
    SettingsLeft,
    SettingsPrefix,
    SettingsKeybindingsIntroDesc,
    KeybindGoToTab,
    KeybindGoToWorkspace,
    KeybindForkSessionRight,
    KeybindForkSessionLeft,
    KeybindForkSessionDown,
    KeybindForkSessionUp,
    SettingsAboutDesc1,
    SettingsDefaultTerminal,
    SettingsDefaultTerminalDesc,
    SettingsDefaultTerminalSet,
    SettingsDefaultTerminalSetSuccess,
    SettingsDefaultTerminalSetFailed,
    SettingsVersion,
    SettingsUpdates,
    SettingsUpdateAndRelaunch,
    SettingsUpdateViewRelease,
    SettingsUpdateChecking,
    SettingsUpdateUpToDate,
    SettingsUpdateDownloadingPercent,
    SettingsUpdateDownloadingBytes,
    SettingsUpdateVerifying,
    SettingsUpdateInstalling,
    SettingsUpdateCheckNow,
    SettingsUpdateCancel,
    SettingsUpdateRetry,
    SettingsUpdateDismiss,
    SettingsUpdateDownloadManually,
    SettingsUpdateFailedTitle,
    SettingsUpdateReady,
    SettingsUpdateReadyNextLaunch,
    SettingsUpdateInstallNow,
    SettingsUpdateDiscard,
    SettingsAutoDownload,
    SettingsAutoDownloadDesc,
    SettingsUpdateChannel,
    SettingsUpdateChannelDesc,
    SettingsUpdateChannelStable,
    SettingsUpdateChannelNightly,
    SettingsDaemonStale,
    SettingsDaemonStaleDesc,
    UpdateDialogTitle,
    UpdateDialogDetail,
    UpdateDialogDetailWindows,
    UpdateDialogDetailManual,
    UpdateDialogCannotSelfUpdate,
    UpdateDialogLater,
    UpdateDialogNextLaunch,
    UpdateDialogNeedsElevation,
    SettingsUpdateCheckFailed,
    SettingsUpdatePrepareFailed,
    SettingsUpdateLaunchFailed,
    SettingsUpdateUnsupportedMacos,
    SettingsUpdateUnsupportedLinux,
    SettingsUpdateLinuxPackage,
    SettingsUpdateUnsupportedWindows,
    SettingsUpdateWindowsAllUsers,
    SettingsUpdateUnsupportedPlatform,
    SettingsUpdateMissingPackage,
    SettingsUpdateMissingChecksums,
    SettingsVersionAvailable,
    SettingsCheckUpdatesDesc,
    SettingsCheckUpdatesOnLaunch,
    SettingsCommandLine,
    SettingsCommandLineDesc,
    SettingsInstallCliOnPath,
    SettingsServer,
    SettingsServerDesc,
    SettingsRestartServer,
    SettingsAppHttpProxy,
    SettingsAppHttpProxyDesc,
    SettingsAppHttpProxyInvalid,
    SettingsAgentClaudeCode,
    SettingsAgentCodex,
    SettingsAgentTraeCode,
    SettingsAgentCopilotCli,
    SettingsAgentOpencode,
    SettingsAgentPi,
    SettingsAgentGrokBuild,
    SettingsAgentOhMyPi,
    SettingsAgentGemini,
    SettingsAgentDroid,
    SettingsAgentQwenCode,
    SettingsAgentGoose,
    SettingsAgentKimiCode,
    SettingsAgentQoderCLI,
    SettingsAgentCrush,
    SettingsAgentCodeBuddy,
    SettingsAgentCursorCli,
    SettingsAgentPrimeAgent,
    SettingsAgentAntigravity,
    SettingsAgentEmpryo,
    SettingsAgentJcode,
    SettingsAgentMuse,
    SettingsMuseManualInstall,
    SettingsAgentQoderCn,
    SettingsSearchAppHttpProxyKeywords,
    SettingsSearchAboutKeywords,
    SettingsSearchAutoDownloadKeywords,
    SettingsSearchCheckUpdatesOnLaunchKeywords,
    SettingsSearchUpdateChannelKeywords,
    SettingsSearchAnsiColorsKeywords,
    SettingsSearchBackgroundImageKeywords,
    SettingsSearchImageOpacityKeywords,
    SettingsSearchArgumentsKeywords,
    SettingsSearchBlurKeywords,
    SettingsSearchBoldFontKeywords,
    SettingsSearchClaudeCodeKeywords,
    SettingsSearchCodexKeywords,
    SettingsSearchTraeCodeKeywords,
    SettingsSearchCommandLineToolKeywords,
    SettingsSearchCopilotCliKeywords,
    SettingsSearchCopyOnSelectKeywords,
    SettingsSearchCursorBlinkKeywords,
    SettingsSearchCursorShapeKeywords,
    SettingsSearchPromptCursorShapeKeywords,
    SettingsSearchCustomThemesKeywords,
    SettingsSearchDetectUrlsKeywords,
    SettingsSearchDimInactivePanesKeywords,
    SettingsSearchAutoHideTitlebarButtonsKeywords,
    SettingsSearchDroidKeywords,
    SettingsSearchFocusFollowsMouseKeywords,
    SettingsSearchFontFamilyKeywords,
    SettingsSearchFontLigaturesKeywords,
    SettingsSearchFontThickenKeywords,
    SettingsSearchFontSizeKeywords,
    SettingsSearchForwardSshLoopbackLinksKeywords,
    SettingsSearchGeminiKeywords,
    SettingsSearchGooseKeywords,
    SettingsSearchGrokBuildKeywords,
    SettingsSearchHideMouseWhileTypingKeywords,
    SettingsSearchHostsKeywords,
    SettingsSearchItalicFontKeywords,
    SettingsSearchKeybindingsKeywords,
    SettingsSearchKeybindingsTitle,
    SettingsSearchKimiCodeKeywords,
    SettingsSearchLineHeightKeywords,
    SettingsSearchUiFontFamilyKeywords,
    SettingsSearchNewTabPositionKeywords,
    SettingsSearchConfirmCloseKeywords,
    SettingsSearchNotifyOnCommandFinishKeywords,
    SettingsSearchNotifyThresholdKeywords,
    SettingsSearchOhMyPiKeywords,
    SettingsSearchOpacityKeywords,
    SettingsSearchOpencodeKeywords,
    SettingsSearchOptionAsMetaKeywords,
    SettingsSearchPiKeywords,
    SettingsSearchProgramKeywords,
    SettingsSearchQwenCodeKeywords,
    SettingsSearchQoderCLIKeywords,
    SettingsSearchCrushKeywords,
    SettingsSearchCodeBuddyKeywords,
    SettingsSearchCursorCliKeywords,
    SettingsSearchPrimeAgentKeywords,
    SettingsSearchAntigravityKeywords,
    SettingsSearchEmpryoKeywords,
    SettingsSearchJcodeKeywords,
    SettingsSearchMuseKeywords,
    SettingsSearchQoderCnKeywords,
    SettingsSearchRememberWindowSizeKeywords,
    SettingsSearchReportMouseToAppsKeywords,
    SettingsSearchRestoreLastLayoutKeywords,
    SettingsSearchScrollSpeedKeywords,
    SettingsSearchSmoothScrollKeywords,
    SettingsSearchScrollbackKeywords,
    SettingsSearchShowTrayIconKeywords,
    SettingsSearchSidebarGroupingKeywords,
    SettingsSearchSmartSelectionKeywords,
    SettingsSearchStartInKeywords,
    SettingsSearchSyncWithSystemKeywords,
    SettingsSearchTabBarPositionKeywords,
    SettingsSearchTerminalBellKeywords,
    SettingsSearchThemeKeywords,
    SettingsSearchTrimTrailingSpacesKeywords,
    SettingsSearchVerifyHostKeysKeywords,
    SettingsSearchWarnBeforeClosingKeywords,
    SettingsSearchStartupWindowKeywords,
    SwitcherNoMatch,
    AddSshHost,
    RestartServer,
    Ok,
    SftpPasteUploadFailed,
    ForwardDisconnected,
    ForwardDisconnectedFrom,
    SshEditProfile,
    ForwardTooltipAdd,
    ForwardTooltipRemove,
    ForwardTooltipTurnOn,
    ForwardTooltipTurnOff,
    ForwardSwitchFailed,
    ForwardLocal,
    ForwardRemote,
    ForwardDynamic,
    ForwardBindLabel,
    ForwardToLabel,
    ForwardSocksLabel,
    ForwardAdd,
    ForwardPortLabel,
    ForwardPortHere,
    ForwardNeedsPort,
    ForwardAdvancedToggle,
    ForwardSimpleToggle,
    ForwardRequestFailed,
    SshPromptNewKey,
    SshPromptOldKey,
    SshPromptHostKeyNewAlgorithm,
    SshPromptTypeYesToOverride,
    SettingsDiscard,
    PanelNoSession,
    PanelNoSessionHint,
    PanelSessionSubtitle,
    PanelProcessesSubtitle,
    PanelProcessesTotal,
    PanelPortsSubtitle,
    PanelPortsUnsupported,
    PanelPortsProbeFailed,
    PanelPortsRestricted,
    PanelPortsEmpty,
    PanelLatency,
    PortAutoForwarded,
    PanelCwd,
    PanelShell,
    PanelSsh,
    PanelAgentWorking,
    PanelAgentWaiting,
    PanelAgentDone,
    PanelRevealInFinder,
    PanelOpenFolder,
    PanelOpenInBrowser,
    WindowStop,
    WindowDelete,
    WindowThisWorkspace,
    WindowConfirmTitle,
    WindowStopUnreachable,
    WindowDeleteUnreachable,
    WindowStopShells,
    WindowDeleteShells,
    PendingConnecting,
    PendingUnreachable,
    HomeTimeJustNow,
    HomeTimeMinutesAgo,
    HomeTimeHourAgo,
    HomeTimeHoursAgo,
    HomeTimeYesterday,
    HomeTimeDaysAgo,
    HomeTimeWeeksAgo,
    HomeTimeMonthsAgo,
    HomeTimeOverYearAgo,
    HomeReopenNamed,
    AppMenuAbout,
    AppMenuCheckForUpdates,
    AppMenuSettings,
    AppMenuServices,
    AppMenuHideApp,
    AppMenuHideOthers,
    AppMenuShowAll,
    AppMenuQuit,
    AppMenuFile,
    AppMenuEdit,
    AppMenuView,
    AppMenuWindow,
    AppMenuHelp,
    AppMenuNewTab,
    AppMenuNewWorkspace,
    AppMenuSplitRight,
    AppMenuSplitLeft,
    AppMenuSplitDown,
    AppMenuSplitUp,
    AppMenuRenameTab,
    AppMenuCopyWorkingDirectory,
    AppMenuCopySessionId,
    AppMenuForkSession,
    AppMenuSaveAgentLaunchArgs,
    AppMenuClosePaneTab,
    AppMenuCloseOtherTabs,
    AppMenuCloseTabsRight,
    AppMenuReopenClosedTab,
    AppMenuRenameWorkspace,
    AppMenuStopWorkspace,
    AppMenuDeleteWorkspace,
    AppMenuUndo,
    AppMenuRedo,
    AppMenuCut,
    AppMenuCopy,
    AppMenuPaste,
    AppMenuSelectAll,
    AppMenuFind,
    AppMenuFindNext,
    AppMenuFindPrevious,
    AppMenuIncreaseFontSize,
    AppMenuDecreaseFontSize,
    AppMenuResetFontSize,
    AppMenuLeftSidebar,
    AppMenuRightPanel,
    AppMenuTabBarPosition,
    AppMenuFocusNextPane,
    AppMenuFocusPreviousPane,
    AppMenuZoomPane,
    AppMenuClearScrollback,
    AppMenuDocumentation,
    AppMenuKeyboardShortcuts,
    AppMenuJoinDiscord,
    AppMenuReportIssue,
    AppMenuRestartServer,
    WindowUntitled,
    TrayShowTty7,
    TrayNotifications,
    TrayAgentNeedsInput,
    AgentStatusWorking,
    AgentStatusWaiting,
    AgentStatusDone,
    NotifyCommandFinished,
    NotifyCommandFinishedWithCommand,
    NotifyAgentFinished,
    NotifyAgentWaiting,
    NotifyTurnFinished,
    TabTooltipMore,
    TabTooltipShowSidebar,
    TabTooltipHideSidebar,
    TabTooltipHideDetailPanel,
    TabTooltipShowDetailPanel,
    TabTooltipZoomed,
    TabMenuLocalShells,
    TabMenuAddHost,
    TabMenuAllHosts,
    TabMenuSplitHint,
    TabUnnamedShell,
    ShellDefault,
    SidebarUngroupedGroup,
    SidebarMoveToGroup,
    SidebarNewGroup,
    SidebarRemoveFromGroup,
    SidebarNewGroupName,
    SidebarRenameGroup,
    SidebarPinGroup,
    SidebarUnpinGroup,
    SidebarGroupNewTab,
    SidebarSetGroupFolder,
    SidebarUseCurrentTabFolder,
    SidebarClearGroupFolder,
    SidebarDeleteGroup,
    SidebarDropToPin,
    TabContextCloseTab,
    TerminalContextClear,
    TabContextCloseTabsBelow,
    TabContextMarkUnread,
    TabContextHibernate,
    TabContextWake,
    TabTooltipAsleep,
    TabWakeFailed,
    RemoteStripDisconnected,
    RemoteStripConnecting,
    RemoteStripReconnecting,
    RemoteStripReconnectingAttempt,
    RemoteStripReconnectingWhy,
    RemoteStripReconnectingAttemptWhy,
    RemoteStripPreempted,
    RemoteStripFailed,
    RemoteStripRouteLost,
    RemoteRouteParkedHint,
    RemoteNoticePreempted,
    RemoteNoticeDisconnected,
    RemoteActionRetryNow,
    RemoteActionTakeBack,
    PaneLeasedBy,
    RemoteActionConnect,
    RemoteActionRetry,
    RemoteActionRemoveEntry,
    RemoteNoConnectionDetails,
    RemoteThisComputer,
    RemoteProfileGone,
    RemoteRestartTitle,
    RemoteRestartBody,
    RemoteReplaceBody,
    RemoteRestartFailedTitle,
    RemoteRestartFailedBody,
    RemoteHostUnreachable,
    RemoteInstallTitle,
    RemoteInstallDetail,
    RemoteInstallPathLabel,
    RemoteInstallVersionLabel,
    RemoteInstallSizeLabel,
    RemoteInstallFromLabel,
    RemoteInstallShaLabel,
    RemoteInstallSilentUpgrades,
    RemoteInstallBytes,
    RemoteMismatchTitle,
    RemoteMismatchDetail,
    RemoteMismatchUnknownBuild,
    RemoteMismatchUnknownBuildFromExe,
    RemoteMismatchReplaceServer,
    RemoteMismatchDowngradeServer,
    RemoteServerOutdated,
    RemoteServerTooNew,
    RemoteDaemonStartFailed,
    RemoteDaemonUnreachable,
    RemoteDaemonTooOld,
    RemoteProfileMissing,
    RemoteAliasMissing,
    RemoteWslNoSsh,
    RemoteLocalStdioNoSsh,
    RemoteHostNotTty7,
    RemoteWorkspaceListFailed,
    RemoteServerRestartFailed,
    RemoteNoRouteToHost,
    RemoteMachineTreeUnexpectedReply,
    RemoteMismatchVersionFromExe,
    SwitcherThisComputer,
    SwitcherStartingServer,
    SwitcherDownloadingServerWithTotal,
    SwitcherDownloadingServerNoTotal,
    SwitcherCopyingServer,
    SwitcherThisWindow,
    SwitcherOpen,
    SwitcherOffline,
    SwitcherDisconnect,
    SwitcherEditHost,
    SwitcherSaveAsHost,
    SwitcherOpenInNewWindow,
    SwitcherRename,
    SwitcherPickAWorkspace,
    SwitcherNoTabs,
    SwitcherNoTabMatch,
    SwitcherTabsAfterOpening,
    SwitcherOpenToManage,
    SwitcherOrphanPanes,
    SwitcherTabCount,
    SwitcherTabCountOne,
    SwitcherActiveTab,
    SwitcherHoldToSwitch,
    SwitcherTabToCrossColumns,
    SwitcherHintNavigate,
    SwitcherHintOpen,
    SwitcherHintNewWindow,
    SwitcherLocalHost,
    SwitcherConnectingTo,
    SwitcherFormName,
    SwitcherFormHost,
    SwitcherFormNamePlaceholder,
    SwitcherFormBack,
    SwitcherFormCreate,
    SwitcherFormPickHint,
    SshPromptPasswordFor,
    SshPromptPassphraseFor,
    SshPromptTwoFactor,
    SshPromptUnknownHost,
    SshPromptHostKeyChanged,
    SshPromptHostKeyChangedBody,
    SshPromptConnect,
    SshPromptUnlock,
    SshPromptSubmit,
    IoDenied,
    IoGone,
    IoNoSpace,
    IoReadOnly,
    IoBusy,
    IoTimedOut,
    TreeWindowOpenedEmpty,
    CmdGroupTabsPanes,
    CmdGroupWorkspaces,
    CmdGroupView,
    CmdGroupTerminal,
    CmdGroupSsh,
    CmdGroupAgents,
    CmdGroupApplication,
    CmdNewTab,
    CmdNewWindow,
    CmdRenameTab,
    CmdSplitRight,
    CmdSplitDown,
    CmdZoomPane,
    CmdNextPane,
    CmdPreviousPane,
    CmdFocusPaneLeft,
    CmdFocusPaneRight,
    CmdFocusPaneUp,
    CmdFocusPaneDown,
    CmdResizePaneLeft,
    CmdResizePaneRight,
    CmdResizePaneUp,
    CmdResizePaneDown,
    CmdSwapPaneNext,
    CmdSwapPanePrevious,
    CmdNextTab,
    CmdPreviousTab,
    CmdMoveTabLeft,
    CmdMoveTabRight,
    CmdRecentTabSwitcher,
    CmdRecentTabSwitcherReverse,
    CmdCopyWorkingDirectory,
    CmdCopySessionId,
    CmdNewAgentTab,
    CmdNextAgentNeedingAttention,
    AppNoAgentNeedsAttention,
    AgentWaitingFor,
    AgeUnderMinute,
    AgeMinutes,
    AgeHours,
    AgeDays,
    CmdForkSession,
    CmdMarkTabAsUnread,
    CmdHibernateTab,
    CmdClosePaneTab,
    CmdCloseWindow,
    CmdCloseOtherTabs,
    CmdCloseTabsToTheRight,
    CmdReopenClosedTab,
    CmdNewWorkspace,
    CmdSwitchWorkspace,
    CmdRenameWorkspace,
    CmdStopWorkspace,
    CmdDeleteWorkspace,
    CmdResetFontSize,
    CmdEnterFullScreen,
    CmdClearScrollback,
    CmdFindInTerminal,
    CmdToggleComposer,
    ComposerPlaceholder,
    ComposerPlaceholderFiles,
    ComposerSendTip,
    ComposerAttach,
    ComposerMenuCommands,
    ComposerCmdProject,
    ComposerCmdUser,
    ComposerAgentAsking,
    ComposerModeDefault,
    ComposerModeAcceptEdits,
    ComposerModePlan,
    ComposerModeBypass,
    ComposerModeAuto,
    ComposerModeTip,
    ComposerModel,
    ComposerModelTip,
    ComposerModelDefault,
    ComposerEffort,
    ComposerEffortTip,
    CmdFindNext,
    CmdFindPrevious,
    CmdCopy,
    CmdPaste,
    CmdAlternatePaste,
    CmdSshManageProfiles,
    CmdSshReconnect,
    CmdSshPortForwarding,
    CmdSettings,
    CmdKeyboardShortcuts,
    CmdAboutTty7,
    CmdCheckForUpdates,
    CmdDocumentation,
    CmdJoinDiscord,
    CmdReportIssue,
    CmdQuitTty7,
    AppRestartServerTitle,
    AppRestartServerFailed,
    AppRestartServerMismatchDetail,
    AppRestartServerDialectDetail,
    AppRestartServerDialectNewerDetail,
    AppRestartServerOldDetail,
    AppRestartServerAfterUpdateDetail,
    AppRestart,
    AppRestartServerNoServer,
    AppRestartServerBody,
    ConfigQuarantinedStartup,
    ConfigQuarantinedReload,
    ConfigUnreadableStartup,
    ConfigUnreadableReload,
    AppReopenTabFailed,
    AppOpenTerminalFailed,
    AppTabsNotRestored,
    AppFullscreenEntered,
    AppFullscreenEnteredNoKey,
    LaunchWorkspacesLeftRunning,
    AppSshConnectionFailed,
    AppSshReconnectFailed,
    AppSplitPaneFailed,
    PaneDragHandleTooltip,
    AppPaneNoCodingAgent,
    AppForkNoCommand,
    AppForkLocalOnly,
    AppForkNoSessionId,
    AppForkSessionIdNotToken,
    AppForkMidTurn,
    AppCmdAgentLaunchTitle,
    AppNoAgentOnPath,
    AppNoAgentSeenHere,
    AppAgentLaunchSaved,
    AppAgentLaunchArgsUnknown,
    AppPlaceholderDescription,
    AppPlaceholderLoginShell,
    AppPlaceholderNone,
    AppThemeColorBackground,
    AppThemeColorForeground,
    AppThemeColorAccent,
    AppThemeColorCursor,
    AppThemeColorSelection,
    AppThemeAnsiBlack,
    AppThemeAnsiRed,
    AppThemeAnsiGreen,
    AppThemeAnsiYellow,
    AppThemeAnsiBlue,
    AppThemeAnsiMagenta,
    AppThemeAnsiCyan,
    AppThemeAnsiWhite,
    AppThemeAnsiBrightBlack,
    AppThemeAnsiBrightRed,
    AppThemeAnsiBrightGreen,
    AppThemeAnsiBrightYellow,
    AppThemeAnsiBrightBlue,
    AppThemeAnsiBrightMagenta,
    AppThemeAnsiBrightCyan,
    AppThemeAnsiBrightWhite,
    AppAgentHooksThisComputer,
    AppAgentHooksRemoteMachine,
    AppAgentHooksNoHomeDir,
    AppAgentHooksOffline,
    AppAgentHooksHomeDirUnresolved,
    AppAgentHooksInstalled,
    AppAgentHooksInstalledEnableCodexThere,
    AppAgentHooksMuseManualInstall,
    AppAgentHooksInstalledCodexEnableFailed,
    AppAgentHooksRemoved,
    AppAgentHooksNothingInstalled,
    AppAgentHooksNoTty7Hooks,
    AppAgentHooksInstallFailed,
    AppAgentHooksRemoveFailed,
    AppKeybindingDisplacedNote,
    AppLocalServerName,
    AppMenuMinimize,
    AppMenuZoom,
    SwitcherStatusTakenOver,
    SettingsFontDefault,
    SettingsUiFontDefault,
    ForwardDescriptionPlaceholder,
    SettingsShellDefaultLoginShell,
    Replace,
    PaneRestoredScreenBanner,
    AppRestartServerBodyInPlace,
    SettingsDaemonStaleDescInPlace,
    PaneTitleDisconnected,
    PaneTitleProcessExited,
    LoopbackForwardFailed,
    TrayTooltipAgents,
    TrayAgentSep,
    CursorShapeBlock,
    CursorShapeBar,
    CursorShapeUnderline,
    PromptCursorShapeFollow,
    CmdUpdateRemoteServer,
    CmdUpdateRemoteServerSubtitle,
    RemoteUpdateBody,
    RemoteUpdateNeedsLocalServer,
    CmdCopyRaw,
    SettingsCopyJoinWrapped,
    SettingsCopyJoinWrappedDesc,
    SettingsSearchCopyJoinWrappedKeywords,
}

pub fn set_locale(gui_language: &str) {
    let index = SUPPORTED_LANGUAGES
        .iter()
        .position(|lang| lang.code == gui_language)
        .unwrap_or(0) as u8;
    #[cfg(test)]
    TEST_LOCALE.with(|slot| slot.set(Some(index)));
    #[cfg(not(test))]
    CURRENT.store(index, Ordering::Relaxed);
}

pub fn t(key: L10nKey) -> &'static str {
    translate(current_locale_index(), key)
}

pub fn t_fmt(key: L10nKey, args: &[(&str, &str)]) -> String {
    apply_template(t(key), args, None)
}

pub fn alias_translations(key: L10nKey) -> Vec<&'static str> {
    let shown = t(key);
    let mut out = Vec::with_capacity(SUPPORTED_LANGUAGES.len() - 1);
    for idx in 0..SUPPORTED_LANGUAGES.len() {
        let text = translate(idx, key);
        if text != shown && !out.contains(&text) {
            out.push(text);
        }
    }
    out
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PluralCategory {
    Zero,
    One,
    Few,
    Many,
    Other,
}

impl PluralCategory {
    pub fn from_count(n: usize) -> Self {
        if n == 0 {
            return Self::Zero;
        }
        if SUPPORTED_LANGUAGES[current_locale_index()].code == "ru-RU" {
            // CLDR Russian cardinal rules for integers. Zero remains an explicit UI branch.
            match (n % 10, n % 100) {
                (1, last_two) if last_two != 11 => Self::One,
                (2..=4, last_two) if !(12..=14).contains(&last_two) => Self::Few,
                _ => Self::Many,
            }
        } else if n == 1 {
            Self::One
        } else {
            Self::Other
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Zero => "zero",
            Self::One => "one",
            Self::Few => "few",
            Self::Many => "many",
            Self::Other => "other",
        }
    }
}

/// Select a plural-aware translation and fill placeholders.
/// The template may use `{count}`; it is always substituted first.
pub fn t_plural(key: L10nKey, count: usize, args: &[(&str, &str)]) -> String {
    let branch = PluralCategory::from_count(count).as_str();
    apply_template(
        translate_variant(current_locale_index(), key, branch),
        args,
        Some(count),
    )
}

fn apply_template(template: &'static str, args: &[(&str, &str)], count: Option<usize>) -> String {
    let mut text = template.to_string();
    if let Some(n) = count {
        text = text.replace("{count}", &n.to_string());
    }
    for (name, value) in args {
        text = text.replace(&format!("{{{name}}}"), value);
    }
    text
}

fn current_locale_index() -> usize {
    #[cfg(test)]
    if let Some(idx) = TEST_LOCALE.with(|slot| slot.get()) {
        return idx as usize;
    }
    CURRENT.load(Ordering::Relaxed) as usize
}

fn translate(locale_idx: usize, key: L10nKey) -> &'static str {
    if let Some(lang) = SUPPORTED_LANGUAGES.get(locale_idx) {
        if let Some(text) = (lang.translate_fn)(key) {
            return text;
        }
    }
    translate_en(key)
}

fn translate_variant(locale_idx: usize, key: L10nKey, branch: &'static str) -> &'static str {
    if let Some(lang) = SUPPORTED_LANGUAGES.get(locale_idx) {
        if let Some(text) = (lang.translate_variant_fn)(key, branch) {
            return text;
        }
    }
    // Locales without few/many use their own other form before falling back to English.
    let fallback = match branch {
        "few" | "many" => "other",
        _ => branch,
    };
    if fallback != branch {
        if let Some(lang) = SUPPORTED_LANGUAGES.get(locale_idx) {
            if let Some(text) = (lang.translate_variant_fn)(key, fallback) {
                return text;
            }
        }
    }
    translate_variant_en(key, fallback).unwrap_or_else(|| translate(locale_idx, key))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_key_is_translated_in_every_locale() {
        // Deliberately the same in every locale. Each of these reads as
        // English because English is the right answer there, not because
        // nobody got to it.
        const KEPT_IN_ENGLISH: &[L10nKey] = &[
            // Protocol, format and command names no locale translates.
            L10nKey::CmdGroupSsh,
            L10nKey::SettingsNavSsh,
            L10nKey::ForwardSocksLabel,
            L10nKey::RemoteInstallShaLabel,
            // Every locale spells this button the same way.
            L10nKey::Ok,
            // Product names.
            L10nKey::SettingsServer,
            L10nKey::SettingsAgentClaudeCode,
            L10nKey::SettingsAgentCodex,
            L10nKey::SettingsAgentTraeCode,
            L10nKey::SettingsAgentCopilotCli,
            L10nKey::SettingsAgentDroid,
            L10nKey::SettingsAgentGemini,
            L10nKey::SettingsAgentGoose,
            L10nKey::SettingsAgentGrokBuild,
            L10nKey::SettingsAgentKimiCode,
            L10nKey::SettingsAgentOhMyPi,
            L10nKey::SettingsAgentOpencode,
            L10nKey::SettingsAgentPi,
            L10nKey::SettingsAgentQwenCode,
            L10nKey::SettingsAgentQoderCLI,
            L10nKey::SettingsAgentCrush,
            L10nKey::SettingsAgentCodeBuddy,
            L10nKey::SettingsAgentCursorCli,
            L10nKey::SettingsAgentPrimeAgent,
            L10nKey::SettingsAgentAntigravity,
            L10nKey::SettingsAgentEmpryo,
            L10nKey::SettingsAgentJcode,
            L10nKey::SettingsAgentMuse,
            L10nKey::SettingsAgentQoderCn,
            // Windows names its backdrop materials, and Japanese Windows keeps
            // those names in Latin script — so does this list. Chinese does
            // translate them (云母 / 亚克力), which is what Microsoft's own
            // zh-CN copy uses, so only the ja side reads as English here.
            L10nKey::SettingsBackdropMica,
            L10nKey::SettingsBackdropMicaAlt,
            L10nKey::SettingsBackdropAcrylic,
            // A language is named in its own language on every locale's list.
            L10nKey::SettingsLanguageEnglish,
            L10nKey::SettingsLanguageChinese,
            L10nKey::SettingsLanguageJapanese,
            L10nKey::SettingsLanguageRussian,
            // The pane-type labels: one names a program, the other a protocol,
            // and no locale renames either.
            L10nKey::PanelShell,
            L10nKey::PanelSsh,
            // "Shell" and "Agent" are the words the Chinese- and
            // Japanese-speaking developer audience uses for these; a
            // translation here would be less clear, not more.
            L10nKey::SettingsShell,
            L10nKey::CmdGroupAgents,
            L10nKey::AppCmdAgentLaunchTitle,
            // A product name, spelled the same in every language.
            // Russian keeps this punctuation separator and product-name template.
            L10nKey::TrayAgentSep,
            L10nKey::TrayTooltipAgents,
        ];

        for &key in KEPT_IN_ENGLISH {
            let en = translate_en(key);
            let zh = translate_zh(key).unwrap();
            let ja = translate_ja(key).unwrap();
            let ru = translate_ru(key).unwrap();
            if zh != en && ja != en && ru != en {
                println!("STALE ALLOWLIST ENTRY: {key:?} en={en:?} zh={zh:?} ja={ja:?}");
            }
        }
        for &key in L10nKey::ALL {
            let en = translate_en(key);
            assert!(!en.is_empty(), "missing en translation for {key:?}");
            for (name, text) in [
                ("zh", translate_zh(key)),
                ("ja", translate_ja(key)),
                ("ru", translate_ru(key)),
            ] {
                let text = text.unwrap_or_else(|| panic!("missing {name} translation for {key:?}"));
                assert!(!text.is_empty(), "empty {name} translation for {key:?}");
                // A value byte-identical to the English one is almost always
                // a key that was added and never translated. The exceptions
                // are listed above, with the reason.
                assert!(
                    text != en || KEPT_IN_ENGLISH.contains(&key),
                    "{name} still reads the English for {key:?}: {en:?}"
                );
            }
        }
    }

    #[test]
    fn wording_that_names_a_platform_names_this_one() {
        for lang in SUPPORTED_LANGUAGES {
            set_locale(lang.code);
            let code = lang.code;

            let copy = t(L10nKey::SettingsCopyOnSelectDesc);

            // This row is shown on macOS and Linux alike (Windows gets the
            // backdrop picker instead), and both honor the switch, so naming
            // either one of them is wrong wherever it is read.
            let blur = t(L10nKey::SettingsBlurDesc);
            assert!(!blur.contains("macOS"), "{code} blur desc: {blur:?}");

            if cfg!(target_os = "macos") {
                assert!(copy.contains('⌘'), "{code} copy-on-select: {copy:?}");
            } else {
                assert!(!copy.contains('⌘'), "{code} copy-on-select: {copy:?}");
            }
        }
        set_locale(default_language_code());
    }

    #[test]
    fn explicit_languages_select_the_right_locale() {
        set_locale("zh-CN");
        assert_eq!(current_locale_index(), 1);
        set_locale("ja-JP");
        assert_eq!(current_locale_index(), 2);
        set_locale("ru-RU");
        assert_eq!(current_locale_index(), 3);
        assert_eq!(t(L10nKey::SettingsLanguageRussian), "Русский");
        set_locale("en");
        assert_eq!(current_locale_index(), 0);
        set_locale("ko");
        assert_eq!(current_locale_index(), 0);
    }

    #[test]
    fn russian_plural_forms_follow_the_count_and_locale() {
        set_locale("ru-RU");
        for (count, expected) in [
            (0, "Нет совпадений"),
            (1, "1 совпадение"),
            (2, "2 совпадения"),
            (5, "5 совпадений"),
            (11, "11 совпадений"),
            (12, "12 совпадений"),
            (14, "14 совпадений"),
            (21, "21 совпадение"),
            (22, "22 совпадения"),
            (25, "25 совпадений"),
            (101, "101 совпадение"),
            (111, "111 совпадений"),
        ] {
            assert_eq!(t_plural(L10nKey::SettingsMatchCount, count, &[]), expected);
        }
        for lang in ["en", "zh-CN", "ja-JP"] {
            set_locale(lang);
            assert_eq!(PluralCategory::from_count(0), PluralCategory::Zero);
            assert_eq!(PluralCategory::from_count(1), PluralCategory::One);
            for count in [2, 5, 11, 21, 22, 25] {
                assert_eq!(PluralCategory::from_count(count), PluralCategory::Other);
            }
            let idx = current_locale_index();
            for branch in ["few", "many"] {
                assert_eq!(
                    translate_variant(idx, L10nKey::SettingsMatchCount, branch),
                    translate_variant(idx, L10nKey::SettingsMatchCount, "other"),
                );
            }
        }
        set_locale(default_language_code());
    }

    #[test]
    fn plural_and_select_branches_are_translated() {
        let plural_keys = [
            L10nKey::CloseTabsTitle,
            L10nKey::AppTabsNotRestored,
            L10nKey::LaunchWorkspacesLeftRunning,
            L10nKey::SettingsMoreAgents,
            L10nKey::SettingsMoreHosts,
            L10nKey::SettingsRestoreChanged,
            L10nKey::SettingsMatchCount,
            L10nKey::SettingsImportSummary,
            L10nKey::SettingsImportIgnored,
            L10nKey::SettingsOfflineMachines,
            L10nKey::WindowStopShells,
            L10nKey::WindowDeleteShells,
            L10nKey::HomeTimeMinutesAgo,
            L10nKey::HomeTimeHoursAgo,
            L10nKey::HomeTimeDaysAgo,
            L10nKey::HomeTimeWeeksAgo,
            L10nKey::HomeTimeMonthsAgo,
        ];
        for key in plural_keys {
            for branch in ["zero", "one", "few", "many", "other"] {
                let text = translate_variant_ru(key, branch)
                    .unwrap_or_else(|| panic!("ru is missing branch {branch:?} for {key:?}"));
                assert!(!text.is_empty(), "empty ru branch {branch:?} for {key:?}");
            }
            for branch in ["zero", "one", "other"] {
                assert!(
                    !translate_variant(0, key, branch).is_empty(),
                    "missing en plural/select branch {branch:?} for {key:?}"
                );
                assert!(
                    !translate_variant(1, key, branch).is_empty(),
                    "missing zh plural/select branch {branch:?} for {key:?}"
                );
                assert!(
                    !translate_variant(2, key, branch).is_empty(),
                    "missing ja plural/select branch {branch:?} for {key:?}"
                );
                // Not every key spells out a "zero" branch; the ones that do
                // must spell it out in every language rather than lean on the
                // English fallback.
                if translate_variant_en(key, branch).is_some() {
                    assert!(
                        translate_variant_zh(key, branch).is_some(),
                        "zh is missing plural/select branch {branch:?} for {key:?}"
                    );
                    assert!(
                        translate_variant_ja(key, branch).is_some(),
                        "ja is missing plural/select branch {branch:?} for {key:?}"
                    );
                }
            }
            // Smoke-check t_plural does not produce empty strings.
            assert!(!t_plural(key, 0, &[]).is_empty());
            assert!(!t_plural(key, 1, &[]).is_empty());
            assert!(!t_plural(key, 5, &[]).is_empty());
        }
    }
}
