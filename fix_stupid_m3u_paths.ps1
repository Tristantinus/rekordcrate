# Fix M3U paths exported from playlists-stupid
# rekordcrate wrote: \\?\C:\Users\trist\Desktop\playlists-stupid-export\Contents\...
# Actual files are at: J:\Contents\...

$m3uRoot  = 'C:\Users\trist\Desktop\playlists-stupid-m3u'
$wrongPrefix = 'C:\Users\trist\Desktop\playlists-stupid-export'  # rekordcrate used fake export root
$correctPrefix = 'O:\PIONEER\Stupid'  # actual location on O: drive

$fixed = 0
$broken = 0
$total = 0

$m3uFiles = Get-ChildItem -LiteralPath $m3uRoot -Recurse -Filter '*.m3u'
foreach ($m3u in $m3uFiles) {
    $lines = [System.IO.File]::ReadAllLines($m3u.FullName, [System.Text.Encoding]::UTF8)
    $changed = $false
    $newLines = $lines | ForEach-Object {
        $line = $_
        # Strip \\?\ prefix if present, then replace wrong prefix
        $cleaned = $line -replace '^\\\\\?\\', ''
        if ($cleaned.StartsWith($wrongPrefix, [System.StringComparison]::OrdinalIgnoreCase)) {
            $relative = $cleaned.Substring($wrongPrefix.Length)
            $newPath = $correctPrefix + $relative
            $total++
            if (Test-Path -LiteralPath $newPath) {
                $fixed++
                $changed = $true
                $newPath
            } else {
                $broken++
                $newPath  # write corrected path even if not found (may be on drive later)
            }
        } else {
            $line
        }
    }
    if ($changed -or ($lines -join '') -ne ($newLines -join '')) {
        [System.IO.File]::WriteAllLines($m3u.FullName, $newLines, [System.Text.Encoding]::UTF8)
    }
}

Write-Host "Total path lines processed: $total"
Write-Host "Paths that resolve (exist on J:): $fixed"
Write-Host "Paths not found: $broken"
