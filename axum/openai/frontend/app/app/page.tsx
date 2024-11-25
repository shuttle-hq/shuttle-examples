"use client";
import { ChatMessage } from "@/components/ChatMessage";
import { ConversationButton } from "@/components/ConversationButton";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";

export interface GPTMessage {
  role: string;
  message: string;
}

export default function AppPage() {
  const router = useRouter();
  const [model, setModel] = useState<string>("gpt-4o");
  const [message, setMessage] = useState<string>("");
  const [conversationId, setConversationId] = useState<number>(0);
  const [conversationList, setConversationList] = useState<number[]>([]);
  const [messages, setMessages] = useState<GPTMessage[]>([]);
  const [loading, setLoading] = useState<boolean>(false);

  function newChat() {
    setConversationId(0);
    setMessages([]);
  }

  async function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();

    if (conversationId === 0) {
      const res = await fetch(`/api/chat/create`, {
        method: "POST",
        headers: {
          "content-type": "application/json",
        },
        body: JSON.stringify({
          prompt: message,
          model: model,
        }),
      });

      const data = await res.json();

      const new_conversation_list = conversationList;
      new_conversation_list.unshift(data.conversation_id);
      console.log(data.conversation_id);
      setConversationId(data.conversation_id);
      setConversationList(new_conversation_list);
    }

    setMessages((prev) => {
      return [...prev, { message: message, role: "user" }];
    });
    setMessage("");
    setLoading(true);

    const conversation_res = await fetch(
      `/api/chat/conversations/${conversationId}`,
      {
        method: "POST",
        headers: {
          "content-type": "application/json",
        },
        body: JSON.stringify({
          prompt: message,
          model: model,
        }),
      },
    );

    const promptReqData = await conversation_res.json();

    setMessages((prev) => {
      return [...prev, { message: promptReqData.response, role: "system" }];
    });
    setLoading(false);
  }

  useEffect(() => {
    const fetchData = async () => {
      const res = await fetch("/api/chat/conversations");
      if (res.ok) {
        const data = await res.json();
        setConversationList(data);
      } else {
        router.replace("/login");
      }
    };

    fetchData();
  }, []);

  return (
    <main className="grid grid-cols-6 grid-rows-1 w-full h-full min-h-screen">
      <div
        id="sidebar"
        className="col-span-1 row-span-1 border-r border-[#333]"
      >
        <h1>Shuttle</h1>
        <div className="p-4">
          <button
            className="px-4 py-1 w-full text-left rounded-md bg-gradient-to-r from-orange-700 to-yellow-400"
            onClick={() => newChat()}
          >
            New chat
          </button>
          <h2 className="text-center font-bold mt-4">Conversations</h2>
          <div className="py-4 flex flex-col items-start gap-2">
            {conversationList.map((x) => (
              <ConversationButton
                id={x}
                key={x}
                setMessages={setMessages}
                setConversationId={setConversationId}
                active={conversationId === x}
              />
            ))}
          </div>
        </div>
      </div>
      <div className="col-span-5 row-span-1 flex flex-col p-4 w-full gap-4 max-h-screen overflow-auto">
        <select
          className="font-bold text-slate-300 w-[15%] bg-gray-800 px-4 py-2"
          onChange={(e) => setModel((e.target as HTMLSelectElement).value)}
        >
          <option className="text-black" value="gpt-4o">
            gpt-4o
          </option>
          <option className="text-black" value="gpt-4o-mini">
            gpt-4o-mini
          </option>
          <option className="text-black" value="gpt-4o-preview">
            gpt-4o-preview
          </option>
        </select>
        <div
          id="chatbox"
          className="flex flex-col gap-4 w-full h-full min-h-[90%-2px] max-h-[90%-2px] overflow-y-scroll"
        >
          {messages.map((x, idx) => (
            <ChatMessage
              message={x.message}
              role={x.role}
              key={`message=${idx}`}
            />
          ))}
          {loading ? <p> Waiting for response... </p> : null}
        </div>
        <form
          className="w-full flex flex-row gap-2 self-end "
          onSubmit={(e) => handleSubmit(e)}
        >
          <input
            className="w-full px-4 py-2 text-black"
            name="message"
            type="text"
            value={message}
            onInput={(e) => setMessage((e.target as HTMLInputElement).value)}
            required
          ></input>
          <button type="submit" className="bg-slate-300 text-black px-4 py-2">
            Send
          </button>
        </form>
      </div>
    </main>
  );
}
