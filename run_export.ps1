$exe     = 'c:\Users\trist\Claude\Rekord Box CLoned Repo\rekordcrate\target\debug\rekordcrate.exe'
$srcPdb  = 'C:\Users\trist\Desktop\playlists-stupid\export.pdb'
$outDir  = 'C:\Users\trist\Desktop\playlists-stupid-m3u'

# Create expected PIONEER/rekordbox/ structure alongside the PDB file
$exportRoot = 'C:\Users\trist\Desktop\playlists-stupid-export'
$pioneerDir = "$exportRoot\PIONEER\rekordbox"
New-Item -ItemType Directory -Force $pioneerDir | Out-Null
Copy-Item -LiteralPath $srcPdb -Destination "$pioneerDir\export.pdb" -Force

New-Item -ItemType Directory -Force $outDir | Out-Null

Write-Host "=== export-playlists ==="
& $exe export-playlists $exportRoot $outDir
Write-Host "Exit code: $LASTEXITCODE"

Write-Host ""
Write-Host "Files exported:"
Get-ChildItem -LiteralPath $outDir -Recurse -Filter '*.m3u' | Select-Object -ExpandProperty FullName | Select-Object -First 20
