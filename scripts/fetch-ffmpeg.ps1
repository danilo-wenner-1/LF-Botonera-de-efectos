# Descarga el FFmpeg que la Botonera incluye para convertir M4A/AAC que no se
# pueden reproducir directamente (HE-AAC). Build oficial "essentials" de
# gyan.dev (GPL v3, incluye libmp3lame). Verifica el SHA-256 publicado.
#
# Uso (desde la raiz del repo):  powershell -ExecutionPolicy Bypass -File scripts\fetch-ffmpeg.ps1
#
# El binario (~100 MB) NO se sube al repositorio: supera el limite de GitHub.
$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'
$base = 'https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip'
$bin = Join-Path $PSScriptRoot '..\src-tauri\binaries'
$target = Join-Path $bin 'ffmpeg-x86_64-pc-windows-msvc.exe'
if (Test-Path $target) { Write-Host "Ya existe: $target"; exit 0 }
$tmp = Join-Path ([IO.Path]::GetTempPath()) ("lf-ffmpeg-" + [guid]::NewGuid())
New-Item -ItemType Directory -Force $tmp, $bin | Out-Null
try {
    Invoke-WebRequest $base -OutFile "$tmp\ffmpeg.zip"
    Invoke-WebRequest "$base.sha256" -OutFile "$tmp\ffmpeg.zip.sha256"
    $expected = (Get-Content "$tmp\ffmpeg.zip.sha256" -Raw).Trim().Split(' ')[0]
    $actual = (Get-FileHash "$tmp\ffmpeg.zip" -Algorithm SHA256).Hash
    if ($expected -ine $actual) { throw "SHA-256 no coincide ($actual)" }
    Expand-Archive "$tmp\ffmpeg.zip" -DestinationPath "$tmp\x"
    $exe = Get-ChildItem "$tmp\x" -Recurse -Filter ffmpeg.exe | Select-Object -First 1
    $root = Split-Path (Split-Path $exe.FullName -Parent) -Parent
    Copy-Item $exe.FullName $target
    Copy-Item (Join-Path $root 'LICENSE') (Join-Path $bin 'FFMPEG-LICENSE.txt') -Force
    Write-Host "FFmpeg listo en $target"
} finally {
    Remove-Item $tmp -Recurse -Force -ErrorAction SilentlyContinue
}
