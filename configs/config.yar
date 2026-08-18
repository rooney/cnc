rule Suspect
{
    strings:
        $win32Alloc = "VirtualAllocEx"
        $win32Write = "WriteProcessMemory"
        $win32Start = "CreateRemoteThread"
        $winNtAlloc = "NtAllocateVirtualMemory"
        $winNtWrite = "NtWriteVirtualMemory"
        $winNtStart = "NtCreateThreadEx"
        $sw3NtAlloc = "Sw3NtAllocateVirtualMemory"
        $sw3NtWrite = "Sw3NtWriteVirtualMemory"
        $sw3NtStart = "Sw3NtCreateThreadEx"

    condition:
        1 of them
}
