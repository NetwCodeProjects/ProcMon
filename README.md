### **Instructions for ProcessBeacon by NetwCodeProjects**

---

### **Audience 1: True Beginners**

#### **Introduction: Purpose and Scope**

ProccessBeacon is a lightweight service designed to monitor a specific system process like chrome.exe and log its activity. It is primarily used for administrative purposes to track when a particular program starts creating a reliable log for analysis or auditing. This guide will walk you through installing and configuring ProcessBeacon to monitor a single application (e.g., `Chrome.exe`). 

#### **Warnings**

* You need **administrator privileges** to install and run ProcessBeacon.  
* The instructions involve using PowerShell and editing Windows services. Follow each step carefully.  
* Avoid renaming or moving installed files after setup.

#### **Equipment and Supplies**

* A Windows computer with administrator access.  
* A provided PowerShell installation script.  
* Internet connection to download necessary files.

#### **Steps**

1. **Download the Files**  
   * Visit [ProcessBeacon Download Page](http://example.com) and download the following:  
     * `InstallProcessBeacon.ps1` (the installation script).  
   * Save these files to your **Desktop**.  
2. **Start PowerShell as Admin**  
   * Press the Windows button on your keyboard and type “powershell”.  
   * Click **Run as Administrator** under PowerShell or Right-click “Windows PowerShell” then click **Run as Administrator**.  
   * If prompted, confirm with **Yes** to allow administrator mode.  
3. **Navigate to** `InstallProcessBeacon.ps1` **in PowerShell window**  
   * Inside PowerShell window type cd \~\\Desktop  
   * Type “installproc” and press tab to autocomplete  
   * `.\InstallProcessBeacon.ps1 will be the command to run`  
4. **Run installation command**  
   * Run `.\InstallProcessBeacon.ps1`  
   * Press enter to execute that command  
   * You are done installing\!  
5. **Verify Logs**   
   * Navigate to `C:/ProgramData/ProcessBeacon`  
   * Open `ProcessBeacon.log` to verify entries.  
6. **Test the Service**  
   * Restart your computer then open and close `chrome.exe`  
   * Confirm new log entries are recorded each time `chrome.exe` starts (you’ll need to refresh the notepad to view new entries made while the log file was open).

---

### **Audience 2: Intermediate/Advanced Users**

#### **Introduction: Purpose and Scope**

ProcessBeacon is a lightweight service for monitoring when specific system processes like Chrome.exe are running and logging their activity. Designed for IT professionals, this guide explains how to install ProcessBeacon as a service and confirm its functionality for administrative or cybersecurity purposes.

#### **Warnings**

* Ensure you have **administrator privileges** to install and manage ProcessBeacon.  
* Verify that the executable is not blocked by antivirus or security policies before installation.

#### **Equipment and Supplies**

* A Windows computer with PowerShell.  
* The `InstallProcessBeacon.ps1` PowerShell script.  
* A stable internet connection to download necessary files.  
* Knowledge of basic service management using PowerShell or `sc.exe`.

#### **Steps**

1. **Download the Required Files**  
   * Visit the [ProcessBeacon Download Page](http://example.com) to download:  
     * `InstallProcessBeacon.ps1` (installation script).  
   * Save the file to a folder, such as `C:\Temp`.  
2. **Install ProcessBeacon as a Service**  
   * Open **PowerShell as Administrator**:  
   * Navigate to the folder where the `InstallProcessBeacon.ps1` script is located.  
   * Execute the script to install ProcessBeacon:  
     `.\InstallProcessBeacon.ps1`  
   * Verify the installation using the `sc.exe` command:  
     `sc.exe qc ProcessBeacon`  
3. **Verify Logs**  
   * Navigate to the log directory ( `C:\ProgramData\ProcessBeacon)`.  
   * Open `ProcessBeacon.log`  
4. **Test the Service**  
   * Restart the computer, then open and close `chrome.exe` Confirm that entries for each process are logged in `ProcessBeacon.log`.

---

