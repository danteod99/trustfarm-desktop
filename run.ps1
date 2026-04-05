$env:MATRIX_APP_WORK_DIR = "C:\Users\Administrator\AppData\Roaming\com.trustfarm"
$env:MATRIX_APP_NAME = "TrustFarm"
$env:RUST_BACKTRACE=1
taskkill /F /IM agent.exe
& "C:\Users\Administrator\AppData\Roaming\com.trustfarm\bin\agent.exe"
