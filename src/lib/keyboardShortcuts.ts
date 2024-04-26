import { register } from "@tauri-apps/api/globalShortcut";

export default async function handleKeyboardShortcuts() {
  await register('CommandOrControl+W', () => {
    console.log('kek');
  });
}
