# Safety & Security Model

This document explains Eidos' security philosophy and the rationale behind command validation decisions.

## Core Principle: Display-Only, Never Execute

**Eidos NEVER executes commands automatically.** All generated commands are displayed for user review before execution. This is the foundational security layer.

## Command Validation Strategy

### Defense-in-Depth Layers

1. **Whitelist-Only Base Commands**
   - Only 23 read-only commands are allowed
   - Commands cannot modify system state
   - Examples: `ls`, `pwd`, `cat`, `grep`, `find`

2. **Dangerous Command Blocking**
   - 60+ destructive commands explicitly blocked
   - Includes: `rm`, `dd`, `chmod`, `sudo`, network tools, etc.

3. **Shell Injection Prevention**
   - All shell metacharacters rejected: `|`, `&`, `;`, `$()`, backticks
   - Quotes blocked (prevents string arguments with malicious content)
   - Redirects blocked: `>`, `>>`, `<`

4. **Path Traversal Protection**
   - Blocks `../` patterns
   - Blocks sensitive directories: `/dev/`, `/proc/`, `/sys/`, `~/.ssh/`

5. **Encoding Attack Prevention**
   - Hex-encoded characters blocked: `\\x`
   - Octal-encoded characters blocked: `\\0`
   - IFS manipulation blocked

### Why This Approach?

**False Positives > False Negatives**

We intentionally reject many legitimate commands to ensure no dangerous commands pass through. Examples:

- ❌ `cat "my file.txt"` - Rejected (contains quotes)
- ❌ `ls *.txt` - Rejected (contains wildcard)
- ✅ `cat file.txt` - Allowed (simple arguments)

This is acceptable because:
1. Users can still execute any command manually
2. The tool is for **generating** commands, not executing them
3. Better to be overly cautious than risk system damage

## Whitelisted Commands

### Information Gathering (11)
- `ls` - List directory contents
- `pwd` - Print working directory
- `whoami` - Show current user
- `hostname` - Show hostname
- `uname` - Show system information
- `date` - Show date/time
- `which` - Show command location
- `whereis` - Locate binary/source/manual
- `file` - Determine file type
- `stat` - Display file status
- `free` - Show memory usage

### File Reading (4)
- `cat` - Concatenate and display files
- `head` - Show first lines of file
- `tail` - Show last lines of file
- `grep` - Search file contents

### File Analysis (2)
- `wc` - Word/line/character count
- `find` - Search for files (NOTE: `-exec` is blocked)

### System Monitoring (3)
- `df` - Show disk usage
- `du` - Show directory size
- `top` - Show processes
- `ps` - Show process status

### File Operations (Read-Only) (2)
- `touch` - Update timestamp (allowed for creating empty files)
- `mkdir` - Create directory (allowed as non-destructive)

## Blocked Command Categories

### Destructive Operations
- File deletion: `rm`, `rmdir`
- Disk operations: `dd`, `mkfs`, `fdisk`
- Permission changes: `chmod`, `chown`, `chgrp`

### System Control
- Power: `shutdown`, `reboot`, `halt`, `poweroff`
- Process: `kill`, `killall`, `pkill`
- Init: `init`, `systemctl`

### Privilege Escalation
- `sudo`, `su`, `doas`
- User management: `useradd`, `userdel`, `passwd`

### Network Operations
- Download: `curl`, `wget`
- Transfer: `scp`, `sftp`, `rsync`
- Connection: `ssh`, `telnet`, `nc`, `netcat`

### Filesystem Operations
- Mounting: `mount`, `umount`, `mkswap`, `swapon`
- Firewall: `iptables`, `ip6tables`, `nft`

## Security Testing

All 60+ dangerous patterns are tested in:
- `lib_core/src/validation.rs` (8 test suites)
- Continuous integration verifies all tests pass

## Adding New Commands

To add a new whitelisted command:

1. **Verify it's read-only** - Must not modify system state
2. **Add to whitelist** in `lib_core/src/validation.rs`
3. **Add tests** for the new command
4. **Update this document** with rationale
5. **Security review** - Get approval from maintainers

## Future Enhancements

Considered for future releases:

1. **Configurable validation levels**
   - Conservative (current)
   - Balanced (allow quoted arguments)
   - Permissive (allow more commands)

2. **Command-specific validators**
   - `find` with `-exec` blocked
   - `grep` with specific flag whitelist

3. **Machine learning classification**
   - Train on dangerous command corpus
   - Probabilistic scoring

## References

- OWASP Command Injection: https://owasp.org/www-community/attacks/Command_Injection
- CWE-78: https://cwe.mitre.org/data/definitions/78.html

---

Last updated: 2025-11-17
Version: 0.2.0-beta
