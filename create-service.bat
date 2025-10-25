sc create ChrisWindowsDriverSec binPath="%~dp0target\x86_64-pc-windows-msvc\debug\ChrisWindowsDriverSec.sys" type=kernel

sc start ChrisWindowsDriverSec

REM sc stop ChrisWindowsDriverSec
REM sc delete ChrisWindowsDriverSec
REM bcdedit.exe -set TESTSIGNING ON