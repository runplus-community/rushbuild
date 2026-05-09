param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$RemainingArgs
)

$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$target = Join-Path $scriptDir "rushbuild.sh"

if (-not (Test-Path -LiteralPath $target -PathType Leaf)) {
    throw "Missing rushbuild.sh next to rushbuild.ps1"
}

function Convert-ToBashPath {
    param([string]$PathValue)

    $cygpath = Get-Command cygpath -ErrorAction SilentlyContinue
    if ($cygpath) {
        return (& cygpath -u $PathValue).Trim()
    }

    return ($PathValue -replace "\\", "/")
}

$bashTarget = Convert-ToBashPath $target
$bashArgs = foreach ($arg in $RemainingArgs) {
    if ($arg -match "^[A-Za-z]:\\" -or $arg -match "^[.][\\/]" -or $arg -match "^[.][.][\\/]") {
        Convert-ToBashPath $arg
    } else {
        $arg
    }
}

& bash $bashTarget @bashArgs
exit $LASTEXITCODE
