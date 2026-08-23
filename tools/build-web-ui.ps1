param()

$ErrorActionPreference = 'Stop'
$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$inputWasm = Join-Path $repoRoot 'target\wasm32-unknown-unknown\release\rf_106_web.wasm'
$webRoot = Join-Path $repoRoot 'plugin\package\web'
$generatedJs = Join-Path $webRoot 'app.js'

Push-Location $repoRoot
try {
    cargo build --locked --release --target wasm32-unknown-unknown -p rf-106-web
    if ($LASTEXITCODE -ne 0) { throw 'Rust web UI build failed' }
    wasm-bindgen $inputWasm --out-dir $webRoot --out-name app --target web --no-typescript
    if ($LASTEXITCODE -ne 0) { throw 'wasm-bindgen generation failed' }
    Add-Content -LiteralPath $generatedJs -Encoding utf8 -Value "`n// Generated bootstrap: all UI behavior lives in the Rust WebAssembly module.`n__wbg_init();"
} finally {
    Pop-Location
}

Write-Output "RF_106_WEB_UI_BUILT wasm=$(Join-Path $webRoot 'app_bg.wasm')"
