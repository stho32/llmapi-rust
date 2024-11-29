# Configuration Parameters
param(
    [Parameter(Mandatory=$false)]
    [ValidateSet('install', 'uninstall', 'reinstall')]
    [string]$Action = 'install',

    [Parameter(Mandatory=$false)]
    [int]$Port = 3000,

    [Parameter(Mandatory=$false)]
    [ValidateSet('Automatic', 'Manual', 'Disabled')]
    [string]$StartupType = 'Automatic',

    [Parameter(Mandatory=$false)]
    [string]$CustomPath
)

# Script must run with administrator privileges
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Error "This script must be run with administrator privileges."
    exit 1
}

# Service configuration
$serviceName = "LlmApiService"
$serviceDisplayName = "LLM API Service"
$serviceDescription = "LLM API Windows Service for interacting with Language Learning Models"

# Determine service path
if ($CustomPath) {
    $servicePath = $CustomPath
} else {
    $servicePath = Join-Path (Get-Location).Path "target\release\llm-api.exe"
}

# Verify executable exists
if (-not (Test-Path $servicePath)) {
    Write-Error "Service executable not found at: $servicePath"
    Write-Error "Please build the project with 'cargo build --release' first or specify a custom path with -CustomPath"
    exit 1
}

function Remove-ExistingService {
    $service = Get-Service -Name $serviceName -ErrorAction SilentlyContinue
    if ($service) {
        Write-Host "Stopping existing service..."
        Stop-Service -Name $serviceName -Force -ErrorAction SilentlyContinue
        Start-Sleep -Seconds 2
        
        Write-Host "Removing existing service..."
        Remove-Service -Name $serviceName -ErrorAction SilentlyContinue
        Start-Sleep -Seconds 2
    }
}

function Install-Service {
    Write-Host "Installing service..."
    $binaryPath = """$servicePath"" service --port $Port"
    New-Service -Name $serviceName `
                -BinaryPathName $binaryPath `
                -DisplayName $serviceDisplayName `
                -StartupType $StartupType `
                -Description $serviceDescription

    Write-Host "Service installed successfully with the following configuration:"
    Write-Host "- Name: $serviceName"
    Write-Host "- Path: $servicePath"
    Write-Host "- Port: $Port"
    Write-Host "- Startup: $StartupType"
    Write-Host "`nYou can now:"
    Write-Host "- Start the service: Start-Service $serviceName"
    Write-Host "- Check status: Get-Service $serviceName"
    Write-Host "- Stop the service: Stop-Service $serviceName"
}

# Main execution
switch ($Action) {
    'install' {
        Remove-ExistingService
        Install-Service
    }
    'uninstall' {
        Remove-ExistingService
        Write-Host "Service uninstalled successfully."
    }
    'reinstall' {
        Remove-ExistingService
        Install-Service
    }
}
