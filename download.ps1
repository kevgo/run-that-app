$ErrorActionPreference = "Stop"

# the version to release
Set-Variable -Name "Version" -Value "0.0.3" -Option Constant

Add-Type -AssemblyName System.IO.Compression.FileSystem


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
  Write-Output "I'm running on an $cpuArchitecture CPU."
  $zipPath = Receive-Archive -version $Version -CpuArchitecture $cpuArchitecture
  Write-Output "zip path: $zipPath"
  Expand-Archive $zipPath
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
    [string]$CpuArchitecture
  )
  $url = "https://github.com/kevgo/run-that-app/releases/download/v${version}/run_that_app_windows_${CpuArchitecture}.zip"
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
  $zip = [System.IO.Compression.ZipFile]::OpenRead($archivePath)
  $zipEntry = $zip.Entries | Where-Object { $_.Name -eq "run-that-app.exe" }
  Write-Output "Zip entry: $zipEntry"
  [System.IO.Compression.ZipFileExtensions]::ExtractToFile($zipEntry, "run-that-app.exe")
  $zip.Dispose()
}

Main
