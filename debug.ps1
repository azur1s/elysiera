$ErrorActionPreference = "Stop"

# Build
cargo xtask bundle elysiera

$plugin_name = "Elysiera"
$vst3_name = "ElysieraDebug.vst3"

# Rename file
$build = Join-Path $pwd "target/bundled/$plugin_name.vst3/Contents/x86_64-win/$plugin_name.vst3"
$build_to = Join-Path $pwd "target/bundled/$plugin_name.vst3/Contents/x86_64-win/$vst3_name"
Write-Host "Renaming $build to $build_to"
Move-Item $build $build_to -Force
$build = $build_to

# Move the VST3 file to the target directory
$pwd = Get-Location
$to = $env:VST3_DIR
Write-Host "Moving $build to $to"
if ($to -eq $null) {
    Write-Host "VST3_DIR is not set. Skipping moving the VST3 file."
    exit 0
}
gsudo Copy-Item $build $to -Force
# Set "NIH_LOG" environment variable to a file path
$env:NIH_LOG = "$pwd/elysiera.log"
# Open Ableton
Start-Process -FilePath "C:\ProgramData\Ableton\Live 12 Beta\Program\Ableton Live 12 Beta.exe"