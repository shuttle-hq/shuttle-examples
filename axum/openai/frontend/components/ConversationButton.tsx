"use client";

import { GPTMessage } from "@/app/app/page";
import { SetStateAction } from "react";

interface Props {
  setMessages: React.Dispatch<SetStateAction<GPTMessage[]>>;
  setConversationId: React.Dispatch<SetStateAction<number>>;
  id: number;
  active: boolean;
}

export const ConversationButton = ({
  setMessages,
  setConversationId,
  id,
  active,
}: Props) => {
  async function handleClick(e: React.MouseEvent<HTMLButtonElement>) {
    e.preventDefault();

    const res = await fetch(`/api/chat/conversations/${id}`);
    const data = await res.json();
    setMessages(data.messages);
    setConversationId(id);
  }

  const isActiveClasses = active
    ? "bg-gradient-to-r from-orange-700 to-yellow-400 text-slate-200"
    : "text-white bg-slate-100/10";
  return (
    <button
      className={` ${isActiveClasses} px-4 py-1 w-full text-left rounded-md transition duration-300 hover:bg-gradient-to-r hover:from-orange-700 hover:to-yellow-400`}
      onClick={(e) => handleClick(e)}
    >
      Conversation {id}
    </button>
  );
};
