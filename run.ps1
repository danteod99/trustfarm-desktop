$env:MATRIX_APP_WORK_DIR = "C:\Users\Administrator\AppData\Roaming\com.tikfarm"
$env:MATRIX_APP_NAME = "TikFarm"
$env:RUST_BACKTRACE=1
taskkill /F /IM agent.exe
& "C:\Users\Administrator\AppData\Roaming\com.tikfarm\bin\agent.exe"
