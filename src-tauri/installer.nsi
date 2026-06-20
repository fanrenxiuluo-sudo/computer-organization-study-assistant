; Tauri 2 NSIS 钩子脚本
;
; Tauri 2 使用 customInit / customInstall / customUnInstall 宏名
; （不同于 Tauri 1 的 NSIS_HOOK_PREINSTALL / NSIS_HOOK_POSTINSTALL 等）
;
; 核心修复：
; 1. customInit：强制安装路径到 %LOCALAPPDATA%，防止用户选择桌面
; 2. customInstall：创建桌面快捷方式，清理旧版桌面残留
; 3. customUnInstall：卸载时删除桌面快捷方式，清理旧版桌面残留

!define LEGACY_DESKTOP_DIR "$DESKTOP\计组备考助手"
!define DESKTOP_LNK "$DESKTOP\计组备考助手.lnk"

; ============================================================================
; 安装初始化：强制安装路径到 %LOCALAPPDATA%
;
; Tauri 2 在 NSIS 向导显示前调用 customInit。
; 无论用户在"选择目录"页面选了什么，此处都强制重定向。
; ============================================================================
!macro customInit
  StrCpy $INSTDIR "$LOCALAPPDATA\计组备考助手"
!macroend

; ============================================================================
; 安装后：创建桌面快捷方式 + 清理旧版桌面文件夹
;
; Tauri 2 在文件解压完毕后调用 customInstall。
; ============================================================================
!macro customInstall
  ; 在桌面创建单个快捷方式
  CreateShortCut "${DESKTOP_LNK}" "$INSTDIR\计组备考助手.exe" "" "$INSTDIR\计组备考助手.exe" 0

  ; 清理旧版桌面残留文件夹（保护 study.db 不丢失）
  IfFileExists "${LEGACY_DESKTOP_DIR}\study.db" has_data no_data
  has_data:
    goto done_cleanup
  no_data:
    RMDir /r "${LEGACY_DESKTOP_DIR}"
  done_cleanup:
!macroend

; ============================================================================
; 卸载后：删除桌面快捷方式 + 清理旧版桌面残留
; ============================================================================
!macro customUnInstall
  ; 删除桌面快捷方式
  Delete "${DESKTOP_LNK}"

  ; 清理桌面残留文件夹（保护 study.db）
  IfFileExists "${LEGACY_DESKTOP_DIR}\study.db" un_has_data un_no_data
  un_has_data:
    goto un_done
  un_no_data:
    RMDir /r "${LEGACY_DESKTOP_DIR}"
  un_done:
!macroend
