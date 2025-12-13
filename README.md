# Helium v3 (Tauri Application)

A modern storage analyzer tool built with:
- **Tauri** (Rust) for the backend
- **Next.js** (React) for the frontend
- **Fluent UI 2** for the design system

## Prerequisites
- Node.js (v22 recommended)
- Rust (latest stable)

## How to Run

### Development Mode
To start the application in development mode with hot-reloading:

```bash
npm run tauri dev
```

### Build for Production
To create an optimized executable for your operating system:

```bash
npm run tauri build
```

The executable will be located in `src-tauri/target/release/bundle/`.
