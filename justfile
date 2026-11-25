[working-directory: 'app']
app:
    pnpm install
    pnpm tauri dev

[working-directory: 'app']
web:
    pnpm install
    pnpm dev

[working-directory: 'e2e']
e2e:
    pnpm  install
    pnpm exec playwright test