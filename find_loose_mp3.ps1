$targets = @('Blackwater', 'Keep-Pushin', 'You-Make-Me-Feel')
$drives = @('J:', 'O:', 'D:', 'G:', 'I:', 'M:')

foreach ($t in $targets) {
    Write-Host "Searching for '$t':"
    foreach ($d in $drives) {
        $results = Get-ChildItem -LiteralPath "$d\" -Filter "*$t*" -File -ErrorAction SilentlyContinue
        foreach ($r in $results) { Write-Host "  $($r.FullName)" }
    }
}
