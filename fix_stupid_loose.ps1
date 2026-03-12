# Fix 3 loose MP3 files that are at J:\ root (not under O:\PIONEER\Stupid\Contents\)
$m3uRoot = 'C:\Users\trist\Desktop\playlists-stupid-m3u'
$fixes = @{
    'O:\PIONEER\Stupid\Blackwater-feat.-Ann-Saunderson-&-Urban-Soul-Orchestra-Octave-One.mp3' = 'J:\Blackwater-feat.-Ann-Saunderson-&-Urban-Soul-Orchestra-Octave-One.mp3'
    'O:\PIONEER\Stupid\Keep-Pushin-Boris-Dlugosch.mp3' = 'J:\Keep-Pushin-Boris-Dlugosch.mp3'
    'O:\PIONEER\Stupid\You-Make-Me-Feel-Sylvester.mp3' = 'J:\You-Make-Me-Feel-Sylvester.mp3'
}

$fixed = 0
Get-ChildItem -LiteralPath $m3uRoot -Recurse -Filter '*.m3u' | ForEach-Object {
    $m3u = $_
    $lines = [System.IO.File]::ReadAllLines($m3u.FullName, [System.Text.Encoding]::UTF8)
    $changed = $false
    $newLines = $lines | ForEach-Object {
        $line = $_
        if ($fixes.ContainsKey($line)) {
            Write-Host "Fixed in $($m3u.Name): $($fixes[$line])"
            $fixed++
            $changed = $true
            $fixes[$line]
        } else { $line }
    }
    if ($changed) {
        [System.IO.File]::WriteAllLines($m3u.FullName, $newLines, [System.Text.Encoding]::UTF8)
    }
}
Write-Host "Total fixed: $fixed"
