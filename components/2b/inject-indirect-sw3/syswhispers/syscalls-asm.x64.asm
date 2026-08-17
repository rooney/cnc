.code

EXTERN SW3_GetSyscallNumber: PROC

Sw3NtCreateThreadEx PROC
	mov [rsp +8], rcx          ; Save registers.
	mov [rsp+16], rdx
	mov [rsp+24], r8
	mov [rsp+32], r9
	sub rsp, 28h
	mov ecx, 010025CC6h        ; Load function hash into ECX.
	call SW3_GetSyscallNumber              ; Resolve function hash into syscall number.
	add rsp, 28h
	mov rcx, [rsp+8]                      ; Restore registers.
	mov rdx, [rsp+16]
	mov r8, [rsp+24]
	mov r9, [rsp+32]
	mov r10, rcx
	DB 68h                     ; "h"
	DB 0h                     ; "0"
	DB 0h                     ; "0"
	DB 69h                     ; "i"
	DB 68h                     ; "h"
	DB 0h                     ; "0"
	DB 0h                     ; "0"
	DB 69h                     ; "i"
	ret
Sw3NtCreateThreadEx ENDP

Sw3NtAllocateVirtualMemory PROC
	mov [rsp +8], rcx          ; Save registers.
	mov [rsp+16], rdx
	mov [rsp+24], r8
	mov [rsp+32], r9
	sub rsp, 28h
	mov ecx, 047D3534Fh        ; Load function hash into ECX.
	call SW3_GetSyscallNumber              ; Resolve function hash into syscall number.
	add rsp, 28h
	mov rcx, [rsp+8]                      ; Restore registers.
	mov rdx, [rsp+16]
	mov r8, [rsp+24]
	mov r9, [rsp+32]
	mov r10, rcx
	DB 68h                     ; "h"
	DB 0h                     ; "0"
	DB 0h                     ; "0"
	DB 69h                     ; "i"
	DB 68h                     ; "h"
	DB 0h                     ; "0"
	DB 0h                     ; "0"
	DB 69h                     ; "i"
	ret
Sw3NtAllocateVirtualMemory ENDP

Sw3NtWriteVirtualMemory PROC
	mov [rsp +8], rcx          ; Save registers.
	mov [rsp+16], rdx
	mov [rsp+24], r8
	mov [rsp+32], r9
	sub rsp, 28h
	mov ecx, 00A5730EBh        ; Load function hash into ECX.
	call SW3_GetSyscallNumber              ; Resolve function hash into syscall number.
	add rsp, 28h
	mov rcx, [rsp+8]                      ; Restore registers.
	mov rdx, [rsp+16]
	mov r8, [rsp+24]
	mov r9, [rsp+32]
	mov r10, rcx
	DB 68h                     ; "h"
	DB 0h                     ; "0"
	DB 0h                     ; "0"
	DB 69h                     ; "i"
	DB 68h                     ; "h"
	DB 0h                     ; "0"
	DB 0h                     ; "0"
	DB 69h                     ; "i"
	ret
Sw3NtWriteVirtualMemory ENDP

end