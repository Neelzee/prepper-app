set working-directory := "app"

app:
    pnpm install
    pnpm tauri dev

web:
    pnpm install
    pnpm dev
