!macro customInstall
  ; 清理旧版本可能在桌面创建的文件夹
  RMDir /r "$DESKTOP\计组备考助手"

  ; 只在桌面创建单个快捷方式
  CreateShortCut "$DESKTOP\计组备考助手.lnk" "$INSTDIR\计组备考助手.exe" "" "$INSTDIR\计组备考助手.exe" 0
!macroend

!macro customUnInstall
  ; 卸载时删除桌面快捷方式
  Delete "$DESKTOP\计组备考助手.lnk"

  ; 清理可能的旧文件夹
  RMDir /r "$DESKTOP\计组备考助手"
!macroend
