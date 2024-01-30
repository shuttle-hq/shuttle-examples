import './App.css'
import { SignOutButton, SignInButton, SignedIn, SignedOut } from "@clerk/clerk-react"

function App() {
  return (
    <div>
      <SignedOut>
        <SignInButton />
        <p>This content is public. Only signed out users can see the SignInButton above this text.</p>
      </SignedOut>
      <SignedIn>
        <SignOutButton/>
        <p>This content is private. Only signed in users can see the SignOutButton above this text.</p>
      </SignedIn>
    </div>
  )
}

export default App
