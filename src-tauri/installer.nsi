; Tauri 2 NSIS 钩子脚本
; 注意：Tauri 2 只识别 NSIS_HOOK_POSTINSTALL / NSIS_HOOK_POSTUNINSTALL 等钩子宏，
; 不识别 Tauri 1 的 customInstall / customUnInstall。此前用错宏名导致清理逻辑从未生效。

; 桌面旧版残留文件夹名（早期版本曾把数据目录建在桌面）
!define LEGACY_DESKTOP_DIR "$DESKTOP\计组备考助手"

; ============================================================================
; 安装后钩子：清理旧版在桌面残留的「计组备考助手」文件夹
; 保护：若文件夹内存在 study.db（用户旧版学习数据），保留不动，
;       交给应用运行时的 migrate_legacy_desktop_db 迁移后再清理，避免丢失数据。
; ============================================================================
!macro NSIS_HOOK_POSTINSTALL
  ; 桌面快捷方式由 Tauri 官方模板负责创建，此处不重复创建。

  IfFileExists "${LEGACY_DESKTOP_DIR}\study.db" legacy_has_data legacy_no_data

  legacy_has_data:
    ; 存在用户旧版数据，保留文件夹，等待应用运行时迁移并清理
    goto legacy_done

  legacy_no_data:
    ; 纯残留空文件夹（或非数据垃圾），安全删除
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
