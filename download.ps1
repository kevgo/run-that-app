param(
  [string]$Version = "0.23.0"
)

$ErrorActionPreference = "Stop"

Set-Variable -Name "version" -Value $Version

function Welcome() {
  Write-Output "RUN-THAT-APP DOWNLOAD SCRIPT"
  Write-Output ""
  Write-Output "This script is under development. Please report issues at"
  Write-Output "https://github.com/kevgo/run-that-app/issues"
  Write-Output ""
}

function Main() {
  Welcome
  $cpuArchitecture = Get-CPUArchitecture
  $zipPath = Receive-Archive -version $version -cpuArchitecture $cpuArchitecture
  Expand-Archive $zipPath
  Remove-Item -Path $zipPath
}

function Get-CPUArchitecture {
  $architecture = (Get-WmiObject -Class Win32_Processor).Architecture
  if ($architecture -eq 9) { return "intel_64" }
  elseif ($architecture -eq 12) { return "arm_64" }
  else { return "Unknown architecture" }
}

function Receive-Archive {
  param (
    [Parameter(Mandatory = $true)]
    [string]$version,

    [Parameter(Mandatory = $true)]
    [string]$cpuArchitecture
  )
  $url = "https://github.com/kevgo/run-that-app/releases/download/v${version}/run_that_app_windows_${cpuArchitecture}.zip"
  $archiveName = [System.IO.Path]::GetFileName($url)
  $tempDir = [System.IO.Path]::GetTempPath()
  $zipPath = Join-Path $tempDir $archiveName
  Invoke-WebRequest -Uri $url -OutFile $zipPath
  return $zipPath
}

function Expand-Archive {
  param (
    [Parameter(Mandatory = $true)]
    [string]$archivePath
  )
  Add-Type -AssemblyName System.IO.Compression.FileSystem
  $zip = [System.IO.Compression.ZipFile]::OpenRead($archivePath)
  $zipEntry = $zip.Entries | Where-Object { $_.Name -eq "rta.exe" }
  $currentDirectory = Get-Location
  $targetPath = Join-Path $currentDirectory "rta.exe"
  [System.IO.Compression.ZipFileExtensions]::ExtractToFile($zipEntry, $targetPath, $true)
  $zip.Dispose()
}

Main
