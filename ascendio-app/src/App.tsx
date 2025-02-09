import "./App.css";

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

import {
  Select,
  SelectContent,
  SelectItem,
  SelectLabel,
  SelectGroup,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [eventName, setEventName] = useState("");

  async function greet() {
    const message: string = await invoke("send_event");
    setGreetMsg(message);
  }

  async function checkconnection() {
    setGreetMsg("Checking Connection...");
    const message: string = await invoke("is_connected");
    message ? setGreetMsg("Connected") : setGreetMsg("Not Connected");
  }

  return (
    <main className="flex min-h-svh w-full items-center justify-center p-6 md:p-10">


      <Select>
        <SelectTrigger className="w-[180px]">
          < SelectValue placeholder="Select a fruit" />
        </SelectTrigger >
        <SelectContent>
          <SelectGroup>
            <SelectLabel>Fruits</SelectLabel>
            <SelectItem value="apple">Apple</SelectItem>
            <SelectItem value="banana">Banana</SelectItem>
            <SelectItem value="blueberry">Blueberry</SelectItem>
            <SelectItem value="grapes">Grapes</SelectItem>
            <SelectItem value="pineapple">Pineapple</SelectItem>
          </SelectGroup>
        </SelectContent>
      </Select >


      <p>{greetMsg}</p>
    </main >
  );
}

export default App;
