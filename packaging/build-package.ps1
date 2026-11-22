<#
.SYNOPSIS
    Genera la versión portable (.zip) y el instalador (.exe) de la demo.

.DESCRIPTION
    1. Compila en release para Windows de 64 bits con el runtime de C incluido
       (crt-static), para que funcione en equipos sin Visual C++ Redistributable.
    2. Prepara dist\Compraventa3Diamantes\ con el ejecutable y las licencias.
    3. Crea dist\Compraventa3Diamantes-<versión>-portable.zip.
    4. Si Inno Setup 6 está instalado, crea dist\Compraventa3Diamantes-<versión>-instalador.exe.

.EXAMPLE
    powershell -ExecutionPolicy Bypass -File packaging\build-package.ps1
#>
[CmdletBinding()]
param(
    # Sólo genera el .zip portable, sin instalador.
    [switch]$PortableOnly
)

$ErrorActionPreference = 'Stop'
$root = Split-Path $PSScriptRoot -Parent
$target = 'x86_64-pc-windows-msvc'
# Carpeta de compilación propia: los RUSTFLAGS distintos no invalidan la de desarrollo.
# En GitHub Actions se indica con BOS_PACKAGE_TARGET_DIR para aprovechar la caché.
if ($env:BOS_PACKAGE_TARGET_DIR) {
    $targetDir = $env:BOS_PACKAGE_TARGET_DIR
} else {
    $targetDir = Join-Path $env:USERPROFILE '.cargo-targets\bos-package'
}
$appName = 'Compraventa3Diamantes'

Push-Location $root
try {
    $metadata = cargo metadata --no-deps --format-version 1 | ConvertFrom-Json
    $version = ($metadata.packages | Where-Object { $_.name -eq 'pos_desktop' }).version
    Write-Host "Versión $version" -ForegroundColor Cyan

    Write-Host 'Compilando (release, runtime de C incluido)...' -ForegroundColor Cyan
    $env:RUSTFLAGS = '-C target-feature=+crt-static'
    cargo build --release -p pos_desktop --target $target --target-dir $targetDir
    if ($LASTEXITCODE -ne 0) { throw 'La compilación falló.' }
    Remove-Item Env:RUSTFLAGS

    $dist = Join-Path $root 'dist'
    $staging = Join-Path $dist $appName
    if (Test-Path $staging) { Remove-Item $staging -Recurse -Force }
    New-Item -ItemType Directory -Force (Join-Path $staging 'licencias') | Out-Null

    Copy-Item (Join-Path $targetDir "$target\release\pos_desktop.exe") (Join-Path $staging "$appName.exe")
    Copy-Item (Join-Path $root 'crates\pos_desktop\assets\fonts\LICENSE-Phosphor.txt') (Join-Path $staging 'licencias')
    Set-Content -Encoding utf8 (Join-Path $staging 'LEEME.txt') @"
Compraventa 3 Diamantes - Sistema de Gestión de Empeños (demo $version)

Esta es una versión de demostración: los datos son de ejemplo y no se guardan
al cerrar la aplicación. Requiere Windows 10 u 11 de 64 bits.
"@

    $zip = Join-Path $dist "$appName-$version-portable.zip"
    if (Test-Path $zip) { Remove-Item $zip -Force }
    Compress-Archive -Path $staging -DestinationPath $zip
    Write-Host "Portable: $zip" -ForegroundColor Green

    if ($PortableOnly) { return }

    $iscc = @(
        "${env:ProgramFiles(x86)}\Inno Setup 6\ISCC.exe",
        "$env:ProgramFiles\Inno Setup 6\ISCC.exe",
        "$env:LOCALAPPDATA\Programs\Inno Setup 6\ISCC.exe"
    ) | Where-Object { Test-Path $_ } | Select-Object -First 1
    if (-not $iscc) {
        Write-Warning 'Inno Setup 6 no está instalado; sólo se generó el portable.'
        Write-Warning 'Instálelo con: winget install --id JRSoftware.InnoSetup -e'
        return
    }

    & $iscc "/DAppVersion=$version" "/DSourceDir=$staging" "/DOutputDir=$dist" (Join-Path $PSScriptRoot 'installer.iss')
    if ($LASTEXITCODE -ne 0) { throw 'Inno Setup no pudo crear el instalador.' }
    Write-Host "Instalador: $(Join-Path $dist "$appName-$version-instalador.exe")" -ForegroundColor Green
}
finally {
    Pop-Location
}
