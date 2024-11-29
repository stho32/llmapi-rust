$servicePath = (Get-Location).Path + "\target\release\llm-api.exe"
$serviceName = "LlmApiService"

# Check if service already exists
$service = Get-Service -Name $serviceName -ErrorAction SilentlyContinue
if ($service) {
    Write-Host "Service already exists. Stopping and removing..."
    Stop-Service -Name $serviceName -Force
    Start-Sleep -Seconds 2
    Remove-Service -Name $serviceName
}

Write-Host "Installing service..."
New-Service -Name $serviceName `
            -BinaryPathName "$servicePath service" `
            -DisplayName "LLM API Service" `
            -StartupType Automatic `
            -Description "LLM API Windows Service"

Write-Host "Service installed successfully. You can now start it with: Start-Service $serviceName"
