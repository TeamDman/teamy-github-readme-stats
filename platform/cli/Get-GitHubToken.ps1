if (-not (Test-Path -Path Env:\GH_TOKEN)) {
    Write-Host "[Get-GitHubToken] Make sure to dot-source this file!"
    $password = op read "op://Private/teamy-github-readme-stats access token/credential" --no-newline
    $env:GH_TOKEN = $password
}