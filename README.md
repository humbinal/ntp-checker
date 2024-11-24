# ntp-toolbox

<div align="center">
  <img src="./public/logo.png" alt="Logo" width="160">
  <h1 align="center">NTP ToolBox: ntp check and serving</h1>
</div>

## 📖 Features

- [x] check whether the specified ntp server address is valid.
- [x] starting ntp server for temporary test.

Currently, only Windows is supported, Linux and macOS will be supported in the future.

## 📜 License

Distributed under the Apache License. See [`LICENSE`](./LICENSE) for more information.

## 📝 Build & Development

### Requirements

Rust >= 1.82.0

Node.js >= 18.0.0

pnpm >= 8.5.0

### Start compilation

1. Clone the repository
    ```bash
    git clone https://github.com/humbinal/ntp-toolbox.git
    ```

2. Install dependencies
    ```bash
    cd ntp-toolbox
    pnpm install
    ```

3. Development (Optional)
    ```bash
    pnpm tauri dev # Run the app in development mode
    ```

4. Build
    ```bash
    pnpm tauri build # Build into installation package
    ```

## 📚Acknowledgement

- [Tauri](https://tauri.app/) A user-friendly GUI framework.
- [Vue.js](https://vuejs.org/) The Progressive JavaScript Framework.
- [Element Plus](https://element-plus.org/) A Vue 3 based component library.
- [rsntp](https://github.com/mlichvar/rsntp) High-performance NTP server written in Rust.
