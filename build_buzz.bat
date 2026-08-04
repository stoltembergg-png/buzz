@echo off
REM build_buzz.bat — local Windows build script for Buzz Desktop releases.
REM This file is gitignored (see .gitignore).
REM
REM Required env:
REM   C:\BuildTools\Common7\Tools\VsDevCmd.bat (VS Build Tools 2022)
REM
REM Output:
REM   desktop\src-tauri\target\x86_64-pc-windows-msvc\release\Buzz.exe
REM   desktop\src-tauri\target\x86_64-pc-windows-msvc\release\bundle\nsis\Buzz_*_x64-setup.exe

setlocal
call "C:\BuildTools\Common7\Tools\VsDevCmd.bat" -arch=x64 -host_arch=x64 || goto :error

cd /d C:\Users\Gabriel\Desktop\buzz-png

REM 1. Compile sidecars
echo === Building sidecars ===
cargo build --release --target x86_64-pc-windows-msvc -p buzz-acp -p buzz-agent -p buzz-dev-mcp -p git-credential-nostr -p buzz-cli || goto :error

REM 2. Bundle sidecars
echo === Bundling sidecars ===
bash ./scripts/bundle-sidecars.sh x86_64-pc-windows-msvc || goto :error

REM 3. Install pnpm deps (idempotent)
echo === Installing pnpm deps ===
call npx --yes pnpm@11.4.0 install --frozen-lockfile || goto :error

REM 4. Tauri build (NSIS bundle, no signing)
echo === Tauri build ===
cd desktop
call npx --yes pnpm@11.4.0 exec tauri build --target x86_64-pc-windows-msvc --bundles nsis --no-sign || goto :error

echo.
echo === Build complete ===
echo Installer: desktop\src-tauri\target\x86_64-pc-windows-msvc\release\bundle\nsis\Buzz_*_x64-setup.exe
exit /b 0

:error
echo.
echo === Build failed ===
exit /b 1
