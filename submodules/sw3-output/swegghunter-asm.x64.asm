.code

EXTERN SW3_GetSyscallNumber: PROC

Sw3NtCreateThreadEx PROC
	mov [rsp +8], rcx          ; Save registers.
	mov [rsp+16], rdx
	mov [rsp+24], r8
	mov [rsp+32], r9
	sub rsp, 28h
	mov ecx, 050A89EEEh        ; Load function hash into ECX.
	call SW3_GetSyscallNumber              ; Resolve function hash into syscall number.
	add rsp, 28h
	mov rcx, [rsp+8]                      ; Restore registers.
	mov rdx, [rsp+16]
	mov r8, [rsp+24]
	mov r9, [rsp+32]
	mov r10, rcx
	DB 75h                     ; "u"
	DB 0h                     ; "0"
	DB 0h                     ; "0"
	DB 6dh                     ; "m"
	DB 75h                     ; "u"
	DB 0h                     ; "0"
	DB 0h                     ; "0"
	DB 6dh                     ; "m"
	ret
Sw3NtCreateThreadEx ENDP

Sw3NtAllocateVirtualMemory PROC
	mov [rsp +8], rcx          ; Save registers.
	mov [rsp+16], rdx
	mov [rsp+24], r8
	mov [rsp+32], r9
	sub rsp, 28h
	mov ecx, 00192677Dh        ; Load function hash into ECX.
	call SW3_GetSyscallNumber              ; Resolve function hash into syscall number.
	add rsp, 28h
	mov rcx, [rsp+8]                      ; Restore registers.
	mov rdx, [rsp+16]
	mov r8, [rsp+24]
	mov r9, [rsp+32]
	mov r10, rcx
	DB 75h                     ; "u"
	DB 0h                     ; "0"
	DB 0h                     ; "0"
	DB 6dh                     ; "m"
	DB 75h                     ; "u"
	DB 0h                     ; "0"
	DB 0h                     ; "0"
	DB 6dh                     ; "m"
	ret
Sw3NtAllocateVirtualMemory ENDP

Sw3NtWriteVirtualMemory PROC
	mov [rsp +8], rcx          ; Save registers.
	mov [rsp+16], rdx
	mov [rsp+24], r8
	mov [rsp+32], r9
	sub rsp, 28h
	mov ecx, 00B99011Bh        ; Load function hash into ECX.
	call SW3_GetSyscallNumber              ; Resolve function hash into syscall number.
	add rsp, 28h
	mov rcx, [rsp+8]                      ; Restore registers.
	mov rdx, [rsp+16]
	mov r8, [rsp+24]
	mov r9, [rsp+32]
	mov r10, rcx
	DB 75h                     ; "u"
	DB 0h                     ; "0"
	DB 0h                     ; "0"
	DB 6dh                     ; "m"
	DB 75h                     ; "u"
	DB 0h                     ; "0"
	DB 0h                     ; "0"
	DB 6dh                     ; "m"
	ret
Sw3NtWriteVirtualMemory ENDP

Sw3NtProtectVirtualMemory PROC
	mov [rsp +8], rcx          ; Save registers.
	mov [rsp+16], rdx
	mov [rsp+24], r8
	mov [rsp+32], r9
	sub rsp, 28h
	mov ecx, 01F81091Fh        ; Load function hash into ECX.
	call SW3_GetSyscallNumber              ; Resolve function hash into syscall number.
	add rsp, 28h
	mov rcx, [rsp+8]                      ; Restore registers.
	mov rdx, [rsp+16]
	mov r8, [rsp+24]
	mov r9, [rsp+32]
	mov r10, rcx
	DB 75h                     ; "u"
	DB 0h                     ; "0"
	DB 0h                     ; "0"
	DB 6dh                     ; "m"
	DB 75h                     ; "u"
	DB 0h                     ; "0"
	DB 0h                     ; "0"
	DB 6dh                     ; "m"
	ret
Sw3NtProtectVirtualMemory ENDP

Sw3NtClose PROC
	mov [rsp +8], rcx          ; Save registers.
	mov [rsp+16], rdx
	mov [rsp+24], r8
	mov [rsp+32], r9
	sub rsp, 28h
	mov ecx, 0895E6112h        ; Load function hash into ECX.
	call SW3_GetSyscallNumber              ; Resolve function hash into syscall number.
	add rsp, 28h
	mov rcx, [rsp+8]                      ; Restore registers.
	mov rdx, [rsp+16]
	mov r8, [rsp+24]
	mov r9, [rsp+32]
	mov r10, rcx
	DB 75h                     ; "u"
	DB 0h                     ; "0"
	DB 0h                     ; "0"
	DB 6dh                     ; "m"
	DB 75h                     ; "u"
	DB 0h                     ; "0"
	DB 0h                     ; "0"
	DB 6dh                     ; "m"
	ret
Sw3NtClose ENDP

Sw3NtQueryInformationProcess PROC
	mov [rsp +8], rcx          ; Save registers.
	mov [rsp+16], rdx
	mov [rsp+24], r8
	mov [rsp+32], r9
	sub rsp, 28h
	mov ecx, 069AB4E00h        ; Load function hash into ECX.
	call SW3_GetSyscallNumber              ; Resolve function hash into syscall number.
	add rsp, 28h
	mov rcx, [rsp+8]                      ; Restore registers.
	mov rdx, [rsp+16]
	mov r8, [rsp+24]
	mov r9, [rsp+32]
	mov r10, rcx
	DB 75h                     ; "u"
	DB 0h                     ; "0"
	DB 0h                     ; "0"
	DB 6dh                     ; "m"
	DB 75h                     ; "u"
	DB 0h                     ; "0"
	DB 0h                     ; "0"
	DB 6dh                     ; "m"
	ret
Sw3NtQueryInformationProcess ENDP

end