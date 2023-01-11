# Configuration

Default configuration structure
```yaml
behaviour_on_incidents: <default behaviour on unhandled calls>
logfile_path: <path to log file on local filesystem>
rules:
  <syscall1>:
    - <syscall1 rule1>
    - <syscall1 rule2>
  <syscall2>:
    - <syscall2 rule1>
    - <syscall2 rule2>
```

Example configuration
```yaml
behaviour_on_incidents: KillProcess
logfile_path: "/var/log/sova.log"
rules:
  execve:
    - subject: Pathname
      behaviour_on_violation: KillProcess
      condition: MustBeIn
      objects:
        - /bin/sh
```


Supported syscalls:
- [execv](syscalls/execv.md)
- [execve](syscalls/execve.md)
- [open](syscalls/open.md)
- [bind](syscalls/bind.md)
- [connect](syscalls/connect.md)
