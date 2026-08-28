# Rust + egui / eframe Project Template & Setup Guide

This folder contains a ready-to-use **eframe / egui** desktop and web (WASM) Rust application template pre-configured for modern Rust (Rust 2024 edition, stable toolchain, `egui 0.36+`).

---

## 🚀 How to Set Up a New Project from this Template

### Step 1: Copy this folder
Copy the contents of this `template` directory into your new project folder:
```powershell
Copy-Item -Path "path\to\template\*" -Destination "path\to\new_project\" -Recurse
```

---

### Step 2: Rename the Project (Automatic or Manual)

#### Option A: Quick PowerShell Command (Recommended)
Open PowerShell in your new project folder and run:
```powershell
$crate = "my_new_app_name"
$title = "My New App Title"

(Get-Content Cargo.toml) -replace 'mattabor_foglalo_2026', $crate | Set-Content Cargo.toml
(Get-Content src\main.rs) -replace 'mattabor_foglalo_2026', $crate -replace 'Mátábor Foglaló 2026', $title | Set-Content src\main.rs
(Get-Content index.html) -replace 'Mátábor Foglaló 2026', $title | Set-Content index.html
(Get-Content assets\sw.js) -replace 'mattabor_foglalo_2026', $crate | Set-Content assets\sw.js
```

#### Option B: Manual File Changes
If you prefer editing manually, update these 4 files:
1. **`Cargo.toml`**:
   - Change `name = "mattabor_foglalo_2026"` to `name = "your_project_name"`
2. **`src/main.rs`**:
   - Change `mattabor_foglalo_2026::TemplateApp` to `your_project_name::TemplateApp`
   - Change window title `"Mátábor Foglaló 2026"` to your desired title
3. **`index.html`**:
   - Change `<title>Mátábor Foglaló 2026</title>` to your app's title
4. **`assets/sw.js`**:
   - Change `./mattabor_foglalo_2026.js` and `./mattabor_foglalo_2026_bg.wasm` to match your crate name

---

### Step 3: Update Rust Toolchain & Dependencies

1. **Update Rust toolchain to latest**:
   ```powershell
   rustup update
   ```

2. **Add wasm32 target (if targeting Web/WASM)**:
   ```powershell
   rustup target add wasm32-unknown-unknown
   ```

3. **Update dependencies to newest crate versions**:
   ```powershell
   cargo update
   ```

---

### Step 4: Verify the Setup

1. **Check compilation**:
   ```powershell
   cargo check
   ```

2. **Run tests**:
   ```powershell
   cargo test
   ```

---

### Step 5: Run the Application

- **Desktop (Native)**:
  ```powershell
  # Debug mode (fast compilation):
  cargo run

  # Release mode (optimized performance):
  cargo run --release
  ```

- **Web (WASM with Trunk)**:
  ```powershell
  # Install trunk (once, if not already installed):
  cargo install --locked trunk

  # Run local dev server with auto-reload:
  trunk serve

  # Build release wasm bundle into dist/ directory:
  trunk build --release
  ```

---

### Step 6: Initialize Git Repository

```powershell
git init
git add .
git commit -m "Initial commit from egui template"
```
