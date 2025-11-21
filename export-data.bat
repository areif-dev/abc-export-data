@echo off
For /f "tokens=2-4 delims=/ " %%a in ('date /t') do (set mydate=%%c-%%a-%%b)
For /f "tokens=1-2 delims=/:" %%a in ('time /t') do (set mytime=%%a%%b)
echo %mydate%_%mytime% > C:\projects\export-data\logs.txt
C:\projects\export-data\export-data.exe > C:\projects\export-data\logs.txt 2>&1
copy /Y "C:\ABC Software\Database Export\Company001\Data\item.data" "C:\Users\User\Documents\Sync" 
copy /Y "C:\ABC Software\Database Export\Company001\Data\item_posted.data" "C:\Users\User\Documents\Sync" 
