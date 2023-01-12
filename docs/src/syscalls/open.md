# Open

Handle [open](https://man7.org/linux/man-pages/man2/open.2.html) syscall.

<details>
Helpful to avoid opening confidential files on local filesystem
</details>

Available subjects:
- `Pathname`
  - Available conditions: `MustBeIn`, `MustNotBeIn`
  - Available objects: string - full path to file

#### Basic structure
```yaml
---
behaviour_on_incidents: KillProcess
logfile_path: "/var/log/sova.log"
rules:
  open:
    - subject: <Subject to check>
      behaviour_on_violation: <Behaviour on violation>
      condition: <Condition>
      objects:
        - <Object to check>
```

#### Example - forbid opening file `/etc/blabla` and `/etc/passwd`
```yaml
---
behaviour_on_incidents: KillProcess
logfile_path: "/var/log/sova.log"
rules:
  open:
    - subject: Pathname
      behaviour_on_violation: KillProcess
      condition: MustNotBeIn
      objects:
        - /etc/blabla
        - /etc/passwd 
```