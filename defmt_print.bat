@echo off
setlocal enabledelayedexpansion
set /a count=0
:loop
C:/Users/Administrator/.cargo/bin/defmt-print.exe -e ./target/thumbv7m-none-eabi/debug/%~1 tcp --port 6666
if %errorlevel% neq 0 (
    set /a count+=1
    echo Command failed, retrying in 1 seconds
    timeout /t 2 /nobreak
    cls
    goto loop
)