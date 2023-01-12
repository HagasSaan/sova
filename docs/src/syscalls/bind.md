# Bind

Handle [bind](https://man7.org/linux/man-pages/man2/bind.2.html) syscall.

<details>
Helpful to avoid creating reverse shells like `nc -lvp 4445` or opening ports by vulnerable app
</details>

Available subjects:
- `Port`
  - Available conditions: `MustBeIn`, `MustNotBeIn`
  - Available objects: int - values from 0 to 65536

#### Basic structure
```yaml
---
behaviour_on_incidents: KillProcess
logfile_path: "/var/log/sova.log"
rules:
  bind:
    - subject: <Subject to check>
      behaviour_on_violation: <Behaviour on violation>
      condition: <Condition>
      objects:
        - <Object to check>
```

#### Example - forbid binding sockets on ports 4445 and 7878
```yaml
---
behaviour_on_incidents: KillProcess
logfile_path: "/var/log/sova.log"
rules:
  bind:
    - subject: Port
      behaviour_on_violation: KillProcess
      condition: MustNotBeIn
      objects:
        - 4445
        - 7878
```