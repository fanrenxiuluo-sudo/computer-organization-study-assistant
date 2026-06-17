!macro customInstall
  CreateShortCut "$DESKTOP\计组备考助手.lnk" "$INSTDIR\计组备考助手.exe" "" "$INSTDIR\计组备考助手.exe" 0
!macroend

!macro customUnInstall
  Delete "$DESKTOP\计组备考助手.lnk"
!macroend