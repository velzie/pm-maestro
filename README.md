# pm-maestro

An intuitive web frontend for managing processes and services on a web server. An easy solution for monitoring your self-hosted software.

![image](https://github.com/CoolElectronics/pm-maestro/assets/58010778/9595089e-64ab-4558-9a13-063a7f42edf6)


# Installation
```
git clone https://github.com/CoolElectronics/pm-maestro
cd pm-maestro
cargo install --path .
cd frontend
pnpm i
pnpm build
```

Then, setup nginx to serve `/frontend/dist` at `/` on a subdomain, and proxy `/api/` to `localhost:8232`, and setup a systemd service for running the `pm-maestro` command that was installed.

If you want to be able to access it from anywhere, it's reccommended to use [oauth-proxy-rs-nginx](https://github.com/CoolElectronics/oauth-proxy-rs-nginx) to maintain security.



note: due to an bug in an upstream library some terminal features may not work on firefox. consider using a chromium based browser.
