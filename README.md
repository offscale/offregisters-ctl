offregisters-ctl
================

Concurrent task runner written in Rust.


## Config example
```yaml
name: taiga
version: 0.0.1-alpha
depends:
- exec: offregister-postgres
   args:
   - --version 9.6.4
   - --debug --foo
   env:
      FOO: haz  # inner `env` takes precendence over outer `env`
- exec: offregister-python
env:
  FOO: bar
  CAN: HAZ
pipe: offmetric --server influxdb://influx.offscale.io
```

## CLI examples

### Config file
```bash
offregisters -c <config_file>
```

---

For more ideas we're considering—and to add your own—see: https://github.com/offscale/offscale-rfcs
