cargo skyline build --release
%SystemRoot%\explorer.exe "C:\Users\Matthew Tuan\AppData\Roaming\eden\sdmc\ultimate\mods\[Moveset] KratosVillager"
xcopy "C:\Users\Matthew Tuan\Documents\GitHub\AxefighterVillager\target\aarch64-skyline-switch\release\libskyline_rs_template.nro" "C:\Users\Matthew Tuan\AppData\Roaming\eden\sdmc\ultimate\mods\[Moveset] KratosVillager"
del "C:\Users\Matthew Tuan\AppData\Roaming\eden\sdmc\ultimate\mods\[Moveset] KratosVillager\plugin.nro"
ren "C:\Users\Matthew Tuan\AppData\Roaming\eden\sdmc\ultimate\mods\[Moveset] KratosVillager\libskyline_rs_template.nro" "plugin.nro"