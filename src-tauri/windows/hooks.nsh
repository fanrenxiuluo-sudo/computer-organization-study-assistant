; Tauri 2 NSIS 安装钩子
; 官方文档：https://v2.tauri.app/distribute/windows-installer/
;
; 支持的钩子宏：
;   NSIS_HOOK_PREINSTALL   — 在文件复制、注册表设置、快捷方式创建之前
;   NSIS_HOOK_POSTINSTALL  — 在所有文件复制、注册表设置和快捷方式创建之后
;   NSIS_HOOK_PREUNINSTALL — 在删除任何文件、注册表键和快捷方式之前
;   NSIS_HOOK_POSTUNINSTALL— 在文件、注册表键和快捷方式已被删除之后
;
; 文件位置：src-tauri/windows/hooks.nsh
; 配置：tauri.conf.json → bundle.windows.nsis.installerHooks = "./windows/hooks.nsh"

!define LEGACY_DESKTOP_DIR "$DESKTOP\计组备考助手"

; ============================================================================
; 安装前钩子：强制安装路径到 %LOCALAPPDATA%，防止用户误选桌面
; NSIS 的 $INSTDIR 控制文件解压目标、注册表条目和快捷方式指向。
; 无论用户在安装向导选择什么目录，此处强制重定向到正确路径。
; ============================================================================
!macro NSIS_HOOK_PREINSTALL
  StrCpy $INSTDIR "$LOCALAPPDATA\计组备考助手"
!macroend

; ============================================================================
; 安装后钩子：创建桌面快捷方式 + 清理旧版桌面残留文件夹
; 保护 study.db：如果文件夹内有用户的学习数据，保留不动
; ============================================================================
!macro NSIS_HOOK_POSTINSTALL
  ; 创建桌面快捷方式
  CreateShortCut "$DESKTOP\计组备考助手.lnk" "$INSTDIR\计组备考助手.exe"

  ; 清理旧版桌面残留文件夹
  IfFileExists "${LEGACY_DESKTOP_DIR}\study.db" has_data no_data
  has_data:
    goto done_cleanup
  no_data:
    RMDir /r "${LEGACY_DESKTOP_DIR}"
  done_cleanup:
!macroend

; ============================================================================
; 卸载后钩子：删除桌面快捷方式 + 清理旧版桌面残留
; ============================================================================
!macro NSIS_HOOK_POSTUNINSTALL
  ; 删除桌面快捷方式
  Delete "$DESKTOP\计组备考助手.lnk"

  ; 清理桌面残留文件夹（保护 study.db）
  IfFileExists "${LEGACY_DESKTOP_DIR}\study.db" un_has_data un_no_data
  un_has_data:
    goto un_done
  un_no_data:
    RMDir /r "${LEGACY_DESKTOP_DIR}"
  un_done:
!macroend
