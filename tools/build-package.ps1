param(
    [string]$Output,
    [string]$RackForgeRoot = $env:RACKFORGE_ROOT
)

$ErrorActionPreference = 'Stop'
$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
if (-not $RackForgeRoot) {
    $RackForgeRoot = Join-Path (Split-Path $repoRoot) 'rackforge'
}
$RackForgeRoot = (Resolve-Path $RackForgeRoot).Path
if (-not $Output) {
    $Output = Join-Path $repoRoot 'artifacts\rf-106-0.2.12.rfplugin'
}
$Output = [System.IO.Path]::GetFullPath($Output)
if (-not $Output.EndsWith('.rfplugin', [System.StringComparison]::OrdinalIgnoreCase)) {
    throw 'Plugin package output must end in .rfplugin'
}
if (Test-Path -LiteralPath $Output) {
    throw "Refusing to overwrite existing package $Output"
}
$component = Join-Path $repoRoot 'target\wasm32-unknown-unknown\release\rackforge_rf_106.wasm'
$package = Join-Path $repoRoot 'plugin\package'
$rackForgeManifest = Join-Path $RackForgeRoot 'Cargo.toml'
if (-not (Test-Path -LiteralPath $rackForgeManifest)) {
    throw "RackForge checkout not found at $RackForgeRoot"
}

$tempRoot = [System.IO.Path]::GetFullPath([System.IO.Path]::GetTempPath())
$stage = Join-Path $tempRoot ("rf-106-package-" + [guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $stage | Out-Null

Push-Location $repoRoot
try {
    cargo run --locked -p rf-106-metadata
    if ($LASTEXITCODE -ne 0) { throw 'Metadata generation failed' }
    & (Join-Path $PSScriptRoot 'build-web-ui.ps1')
    if ($LASTEXITCODE -ne 0) { throw 'Rust web UI build failed' }
    cargo build --locked --release -p rackforge-rf-106 --target wasm32-unknown-unknown
    if ($LASTEXITCODE -ne 0) { throw 'WebAssembly build failed' }
    New-Item -ItemType Directory -Force (Split-Path $Output) | Out-Null
    Copy-Item (Join-Path $package '*') $stage -Recurse
    Copy-Item (Join-Path $repoRoot 'LICENSE') $stage
    Copy-Item (Join-Path $repoRoot 'NOTICE.md') $stage
    cargo +stable-x86_64-pc-windows-msvc run --manifest-path $rackForgeManifest --locked -p rackforge-store -- pack-wasm $stage $component $Output
    if ($LASTEXITCODE -ne 0) { throw 'RackForge packaging failed' }
} finally {
    Pop-Location
    $resolvedStage = [System.IO.Path]::GetFullPath($stage)
    if ($resolvedStage.StartsWith($tempRoot, [System.StringComparison]::OrdinalIgnoreCase)) {
        Remove-Item -LiteralPath $resolvedStage -Recurse -Force
    }
}

Write-Output "RFPLUGIN_BUILT path=$Output component=$component"
