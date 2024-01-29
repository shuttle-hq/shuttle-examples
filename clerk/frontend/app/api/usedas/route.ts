import { auth } from "@clerk/nextjs";
import { NextResponse } from "next/server";

export async function GET() {
    const { sessionId } = auth();

    const res = await (await fetch(process.env.NEXT_PUBLIC_API_BASE_URL + "/users", {
        credentials: "same-origin",
    })).json();
    console.log(res);
    return NextResponse.json({ data: "dasdasd" })
}