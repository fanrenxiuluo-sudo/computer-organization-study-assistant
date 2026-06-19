; Tauri 2 NSIS 钩子脚本
; 核心修复：阻止安装到桌面，强制安装路径到 %LOCALAPPDATA%
;
; 问题根因：NSIS 安装向导允许用户选择桌面作为安装目录，
; 导致整个应用（exe/dll/resources）被解压到桌面，形成残留文件夹。
; v0.3.2 的 POSTINSTALL 清理只处理数据文件，未阻止程序文件被装到桌面。
;
; 修复方案：
; 1. PREINSTALL：无条件强制 $INSTDIR 到正确路径，覆盖任何用户选择/注册表残留
; 2. POSTINSTALL：继续清理旧版桌面残留数据文件夹
; 3. POSTUNINSTALL：同理清理

!define LEGACY_DESKTOP_DIR "$DESKTOP\计组备考助手"

; ============================================================================
; 安装前钩子：强制安装路径，防止桌面安装
;
; NSIS 的 $INSTDIR 控制文件解压目标、注册表条目和快捷方式。
; 无论用户在"选择目录"页面选了什么，此处都强制重定向到 %LOCALAPPDATA%。
; ============================================================================
!macro NSIS_HOOK_PREINSTALL
  StrCpy $INSTDIR "$LOCALAPPDATA\计组备考助手"
!macroend

; ============================================================================
; 安装后钩子：清理旧版在桌面残留的「计组备考助手」文件夹
; 保护：若文件夹内存在 study.db（用户旧版学习数据），保留不动，
;       交给应用运行时的 migrate_legacy_desktop_db 迁移后再清理，避免丢失数据。
; ============================================================================
!macro NSIS_HOOK_POSTINSTALL
  IfFileExists "${LEGACY_DESKTOP_DIR}\study.db" legacy_has_data legacy_no_data

  legacy_has_data:
    goto legacy_done

  legacy_no_data:
    RMDir /r "${LEGACY_DESKTOP_DIR}"

  legacy_done:
!macroend

; ============================================================================
; 卸载后钩子：清理桌面残留文件夹
; 同样保护 study.db：有用户旧版数据则不删（卸载新版不应删除用户旧版数据）
; ============================================================================
!macro NSIS_HOOK_POSTUNINSTALL
  IfFileExists "${LEGACY_DESKTOP_DIR}\study.db" un_legacy_has_data un_legacy_no_data

  un_legacy_has_data:
    goto un_legacy_done

  un_legacy_no_data:
    RMDir /r "${LEGACY_DESKTOP_DIR}"

  un_legacy_done:
!macroend