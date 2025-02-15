import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Menu } from '@tauri-apps/api/menu';
import { TrayIcon, TrayIconEvent } from '@tauri-apps/api/tray';
import appIcon from './assets/app-icon.svg'

const menu = await Menu.new({
  items: [
    {
      id: 'settings',
      text: 'Settings',
      action: async () => {
        console.log('settings pressed');
        await invoke('open_main_window', [])
      },
    },
    {
      id: 'quit',
      text: 'Quit',
      action: async () => {
        console.log('quit pressed')
        await invoke('close_app', [])
      },
    },
  ],
});


const options = {
  menu,
  action: (event: TrayIconEvent) => {
    switch (event.type) {
      case 'Click':
        console.log(
          `mouse ${event.button} button pressed, state: ${event.buttonState}`
        );
        break;
      case 'DoubleClick':
        console.log(`mouse ${event.button} button pressed`);
        break;
      case 'Enter':
        console.log(
          `mouse hovered tray at ${event.rect.position.x}, ${event.rect.position.y}`
        );
        break;
      case 'Move':
        console.log(
          `mouse moved on tray at ${event.rect.position.x}, ${event.rect.position.y}`
        );
        break;
      case 'Leave':
        console.log(
          `mouse left tray at ${event.rect.position.x}, ${event.rect.position.y}`
        );
        break;
    }
  },
};

const tray = await TrayIcon.new(options);

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main className="w-full h-full flex justify-center p-2 bg-gray-100">
      <h1 className="text-4xl font-bold">Secure Connect</h1>
    </main>
  );
}

export default App;
