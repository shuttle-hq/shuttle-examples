import { SignIn, SignedIn, SignedOut } from "@clerk/clerk-react";
import UsersTable from "./components/users-table";

function App() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <SignedOut>
        <SignIn />
      </SignedOut>
      <SignedIn>
        <UsersTable />
      </SignedIn>
    </main>
  );
}

export default App;
