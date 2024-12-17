# Configuration
$exeUrl = "http://jb-daylon/qdbeacon.exe"  # URL of the service executable
$installDir = "C:\ProgramData\qdbeacon"    # Installation directory
$exePath = "$installDir\qdbeacon.exe"     # Full path to the service executable
$serviceName = "qdbeacon"                 # Name of the Windows service

# Ensure the installation directory exists
if (-Not (Test-Path $installDir)) {
    Write-Host "Creating installation directory at $installDir..."
    try {
        New-Item -ItemType Directory -Path $installDir -Force | Out-Null
        Write-Host "Installation directory created successfully."
    } catch {
        Write-Host "ERROR: Failed to create installation directory: $_"
        Exit 1
    }
} else {
    Write-Host "Installation directory already exists at $installDir."
}

# Download the service executable
Write-Host "Downloading QDBeacon executable from $exeUrl..."
try {
    Invoke-WebRequest -Uri $exeUrl -OutFile $exePath -UseBasicParsing -ErrorAction Stop
    Write-Host "Executable downloaded successfully to $exePath."
} catch {
    Write-Host "ERROR: Failed to download the executable: $_"
    Exit 1
}

# Check if the service already exists
Write-Host "Checking if the service '$serviceName' exists..."
if (sc.exe query "$serviceName" > $null 2>&1) {
    Write-Host "Service '$serviceName' exists. Stopping and deleting it..."
    try {
        sc.exe stop "$serviceName" > $null 2>&1
        sc.exe delete "$serviceName" > $null 2>&1
        Start-Sleep -Seconds 2
        Write-Host "Service '$serviceName' removed successfully."
    } catch {
        Write-Host "ERROR: Failed to remove existing service: $_"
        Exit 1
    }
} else {
    Write-Host "Service '$serviceName' does not exist. Proceeding with creation."
}

# Construct the service creation command
$scCommand = "sc.exe create $serviceName binPath= `"$exePath`" start= auto"
Write-Host "DEBUG: Service creation command: $scCommand"

# Create the service
Write-Host "Creating the service '$serviceName'..."
try {
    Invoke-Expression $scCommand
    Write-Host "Service '$serviceName' created successfully."
} catch {
    Write-Host "ERROR: Failed to create the service. Check the command and try again."
    Exit 1
}

# Start the service
Write-Host "Starting the service '$serviceName'..."
try {
    sc.exe start "$serviceName" > $null 2>&1
    Write-Host "Service '$serviceName' started successfully."
} catch {
    Write-Host "ERROR: Failed to start the service: $_"
    Exit 1
}

# Verify the service status
Write-Host "Verifying the service status..."
Start-Sleep -Seconds 2
$service = Get-Service -Name $serviceName -ErrorAction SilentlyContinue

if ($service.Status -eq 'Running') {
    Write-Host "Service '$serviceName' is running successfully."
} else {
    Write-Host "ERROR: Service '$serviceName' failed to start. Status: $($service.Status)"
    Exit 1
}

Write-Host "QDBeacon installation completed successfully."
