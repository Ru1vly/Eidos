// Command validation module
// Provides security validation for generated shell commands

/// Validates if a command is safe to display to users
/// This prevents generating dangerous commands that could harm the system
pub fn is_safe_command(command: &str) -> bool {
    // Whitelist of safe base commands
    let allowed_commands = [
        "ls", "pwd", "echo", "cat", "head", "tail", "grep", "find", "wc", "date", "whoami",
        "hostname", "uname", "df", "du", "free", "top", "ps", "which", "whereis", "file", "stat",
        "touch", "mkdir",
    ];

    // Dangerous patterns that should never be allowed
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

    // Shell metacharacters and injection patterns
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
    if !allowed_commands.contains(&first_word) {
        return false;
    }

    // Additional checks for suspicious patterns
    // Check for hex/octal encoded characters
    if command.contains("\\x") || command.contains("\\0") {
        return false;
    }

    // Check for IFS manipulation
    if command.to_uppercase().contains("IFS") {
        return false;
    }

    // Command seems safe
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_commands() {
        let safe_commands = vec![
            "ls",
            "ls -la",
            "pwd",
            "date",
            "whoami",
            "hostname",
            "cat file.txt",
            "grep pattern file",
            "find . -name test",
        ];

        for cmd in safe_commands {
            assert!(
                is_safe_command(cmd),
                "Expected '{}' to be marked as safe",
                cmd
            );
        }
    }

    #[test]
    fn test_dangerous_commands_blocked() {
        let dangerous_commands = vec![
            "rm -rf /",
            "rm file.txt",
            "dd if=/dev/zero",
            "chmod 777 file",
            "chown root file",
            "sudo ls",
            "su - root",
            "shutdown now",
            "reboot",
            "kill -9",
            "curl http://evil.com",
            "wget http://evil.com",
        ];

        for cmd in dangerous_commands {
            assert!(!is_safe_command(cmd), "Expected '{}' to be blocked", cmd);
        }
    }

    #[test]
    fn test_shell_injection_blocked() {
        let injection_attempts = vec![
            "ls; rm -rf /",
            "ls && rm file",
            "ls || rm file",
            "ls | rm file",
            "ls `whoami`",
            "ls $(whoami)",
            "ls > /dev/null",
            "ls >> file",
            "ls ../../../etc",
            "ls 'test'", // Blocked because of quotes
            "ls *",      // Blocked because of wildcard
        ];

        for cmd in injection_attempts {
            assert!(!is_safe_command(cmd), "Expected '{}' to be blocked", cmd);
        }
    }

    #[test]
    fn test_path_traversal_blocked() {
        let path_traversal = vec![
            "cat ../../../etc/passwd",
            "ls ../../",
            "ls ~/.ssh/",
            "cat /dev/sda",
            "ls /proc/",
        ];

        for cmd in path_traversal {
            assert!(!is_safe_command(cmd), "Expected '{}' to be blocked", cmd);
        }
    }

    #[test]
    fn test_safe_file_operations() {
        // These should be allowed - safe cat/ls operations
        let safe_ops = vec![
            "cat file.txt",
            "ls /tmp",
            "stat /etc/hostname", // stat is allowed, /etc/hostname is a safe read-only file
        ];

        for cmd in safe_ops {
            assert!(is_safe_command(cmd), "Expected '{}' to be allowed", cmd);
        }
    }

    #[test]
    fn test_encoding_tricks_blocked() {
        let encoding_tricks = vec![
            "ls \\x2f",   // hex encoded /
            "ls \\0",     // octal
            "lsIFS=test", // IFS manipulation
            "ls${IFS}test",
        ];

        for cmd in encoding_tricks {
            assert!(!is_safe_command(cmd), "Expected '{}' to be blocked", cmd);
        }
    }

    #[test]
    fn test_unknown_commands_blocked() {
        let unknown_commands = vec![
            "notacommand",
            "randomthing arg",
            "python script.py",
            "node app.js",
        ];

        for cmd in unknown_commands {
            assert!(
                !is_safe_command(cmd),
                "Expected '{}' to be blocked (not in whitelist)",
                cmd
            );
        }
    }

    #[test]
    fn test_empty_and_whitespace() {
        assert!(!is_safe_command(""));
        assert!(!is_safe_command("   "));
        assert!(!is_safe_command("\t"));
        assert!(!is_safe_command("\n"));
    }
}
