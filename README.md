# swatchdog - simple watchdog!

Just... Send an HTTP request per interval to a given URL. You can use any uptime monitoring service to monitor your machine.

## Download & Install

The latest version is available at [GitHub Releases](https://github.com/ImSingee/swatchdog/releases). It's just a single binary file, so you can download it and run it directly.

## Usage

Just as simple as... No need to explain anything! Just run it with `--help` to see the help message.

```
Usage: swatchdog [OPTIONS] --url <URL>

Options:
  -u, --url <URL>
      --method <METHOD>      [default: GET]
      --interval <INTERVAL>  [default: 60s]
      --verbose
  -h, --help                 Print help
  -V, --version              Print version
```

The tool is tested with [uptime-kuma](https://github.com/louislam/uptime-kuma) and I personally recommend it.

## Configure

You may need to configure it to make it run as a service (start on boot).

**linux systemd service example**

```ini
[Unit]
Description=swatchdog

[Service]
User=nobody
Group=nobody
ExecStart=/path/to/swatchdog -u http://example.com --interval 60s

[Install]
WantedBy=multi-user.target
```

(place it under `/lib/systemd/system/swatchdog.service` and run `systemctl enable swatchdog`)

**macos launchd service example**

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
  <dict>
    <key>Label</key>
    <string>me.singee.swatchdog</string>
    <key>ProgramArguments</key>
    <array>
      <string>/path/to/your/swatchdog</string>
      <string>-u=http://example.com</string>
      <string>--interval=60s</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>/Users/USERNAME/.swatchdog.log</string>
    <key>StandardErrorPath</key>
    <string>/Users/USERNAME/.swatchdog.log</string>
  </dict>
</plist>
```

(place it under `~/Library/LaunchAgents/me.singee.swatchdog.plist` and run `launchctl load ~/Library/LaunchAgents/me.singee.swatchdog.plist`)

## License

MIT