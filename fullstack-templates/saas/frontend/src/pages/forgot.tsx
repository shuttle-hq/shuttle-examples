import Layout from "@/components/Layout"
import React from "react"
import { useRouter } from 'next/router'

export default function CreateCustomer() {

  const [firstName, setFirstName] = React.useState<string>("");
  const [lastName, setLastName] = React.useState<string>("");
  const [email, setEmail] = React.useState<string>("");
  const [phone, setPhone] = React.useState<string>("");
  const [priority, setPriority] = React.useState<string>("");

  let router = useRouter();


  const handleSubmit = async (e: React.SyntheticEvent) => {
    e.preventDefault()

    const url = `//${window.location.host}/api/auth/register`

    try {
      let res = await fetch(url,
        {
          method: "POST",
          mode: "cors",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            firstName: firstName,
            lastName: lastName,
            email: email,
            phone: phone,
            priority: Number(priority)
          }),
        })

      if (res.ok) {
        router.push("/dashboard/customers");
      }

    } catch (e: any) {
      console.log(`Error: ${e}`)
    }
  }

  return (
    <>
      <Layout>
        <form className="py-10 flex flex-col gap-4 justify-center items-center">
          <h1 className="lg:text-2xl text-xl text-center">Create Customer</h1>
          <label htmlFor="firstname">
            <span>First name: </span>
            <input type="text" name="firstname" className="px-5 py-2" value={firstName} onInput={(e) => setFirstName((e.target as HTMLInputElement).value)}></input>
          </label>
          <label htmlFor="lastname">
            <span>Last name: </span>
            <input type="email" name="lastname" className="px-5 py-2" value={lastName} onInput={(e) => setLastName((e.target as HTMLInputElement).value)}></input>
          </label>
          <label htmlFor="email">
            <span>Email address: </span>
            <input type="text" name="email" className="px-5 py-2" value={email} onInput={(e) => setEmail((e.target as HTMLInputElement).value)}></input>
          </label>
          <label htmlFor="phone">
            <span>Mobile number: </span>
            <input type="text" name="phone" className="px-5 py-2" value={phone} onInput={(e) => setPhone((e.target as HTMLInputElement).value)}></input>
          </label>
          <label htmlFor="priority">
            <span>Priority: </span>
            <select name="priority" value={priority} onChange={(e) => setPriority((e.target as HTMLSelectElement).value)}>
              <option value="1">Very Low</option>
              <option value="2">Low</option>
              <option value="3">Medium</option>
              <option value="4">High</option>
              <option value="5">Very High</option>
            </select>
          </label>
          <button type="submit">Submit</button>
        </form>
      </Layout>
    </>
  )
}



