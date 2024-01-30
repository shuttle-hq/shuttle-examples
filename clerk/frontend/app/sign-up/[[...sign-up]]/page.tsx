import { SignUp } from "@clerk/nextjs";

export default function Page() {
  return (
    <div className="w-full h-full flex justify-center items-center pt-10">
      <SignUp />
    </div>
  );
}
