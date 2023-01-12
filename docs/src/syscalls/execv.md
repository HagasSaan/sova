# Execv

Handle [execv](https://man7.org/linux/man-pages/man3/exec.3.html) syscal.

<details>
Helpful to avoid executing specific programs or scripts.
</details>

Available subjects:
- `Pathname`
    - Available conditions: `MustBeIn`, `MustNotBeIn`
    - Available objects: string - full path to file
- 'Argv'
    - Available conditions: `MustBeIn`, `MustNotBeIn`
    - Available objects: string - argument for executable

#### Basic structure
```yaml
---
behaviour_on_incidents: KillProcess
logfile_path: "/var/log/sova.log"
rules:
  execv:
    - subject: <Subject to check>
      behaviour_on_violation: <Behaviour on violation>
      condition: <Condition>
      objects:
        - <Object to check>
```

#### Example - allow to execute only `/bin/cat` and `/bin/sh`
```yaml
---
behaviour_on_incidents: KillProcess
logfile_path: "/var/log/sova.log"
rules:
  execv:
  - subject: Pathname
    behaviour_on_violation: KillProcess
    condition: MustBeIn
    objects:
      - /bin/cat
      - /bin/sh
```

#### Example - forbid any executions with `/etc/passwd` file, like `cat /etc/passwd`
```yaml
---
behaviour_on_incidents: KillProcess
logfile_path: "/var/log/sova.log"
rules:
  execv:
  - subject: Argv
    behaviour_on_violation: KillProcess
    condition: MustNotBeIn
    objects:
      - /etc/passwd
```


