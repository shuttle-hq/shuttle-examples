import Markdown from "react-markdown";
import remarkGfm from "remark-gfm";

interface Props {
  message: string;
  role: string;
}

export const ChatMessage = ({ message, role }: Props) => {
  const color = role === "user" ? "bg-slate-300" : "bg-orange-500";
  const align = role === "user" ? "self-end" : "self-start";
  return (
    <div
      className={`${color} ${align} text-black p-4 rounded-xl w-[80%] flex flex-col gap-8`}
    >
      {role === "user" ? (
        <p> {message}</p>
      ) : (
        <Markdown remarkPlugins={[remarkGfm]}>{message}</Markdown>
      )}
    </div>
  );
};
