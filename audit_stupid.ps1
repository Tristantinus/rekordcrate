$m3uRoot = 'C:\Users\trist\Desktop\playlists-stupid-m3u'
$files = Get-ChildItem -LiteralPath $m3uRoot -Recurse -Filter '*.m3u'
$total = 0; $ok = 0; $missing = @()

foreach ($f in $files) {
    $lines = [System.IO.File]::ReadAllLines($f.FullName, [System.Text.Encoding]::UTF8)
    foreach ($line in $lines) {
        if ($line.Length -gt 3 -and $line[1] -eq ':' -and $line[2] -eq '\') {
            $total++
            if (Test-Path -LiteralPath $line) { $ok++ }
            else { $missing += $line }
        }
    }
}

Write-Host "Total: $total  OK: $ok  Missing: $($missing.Count)"
$missing | Select-Object -Unique | ForEach-Object { Write-Host "  MISSING: $_" }
