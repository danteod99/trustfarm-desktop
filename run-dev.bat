@echo off
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
set PATH=C:\Users\Usuario\.cargo\bin;%PATH%
cd /d C:\Users\Usuario\tikfarm-desktop
npm run start
