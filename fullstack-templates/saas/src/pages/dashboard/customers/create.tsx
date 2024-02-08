import Layout from '../../../components/Layout';
import React from 'react';
import { useRouter } from 'next/router';
import { accountStore } from '@/stores/zustandStore';

export default function CreateCustomer() {
  const [firstName, setFirstName] = React.useState<string>('');
  const [lastName, setLastName] = React.useState<string>('');
  const [custEmail, setCustEmail] = React.useState<string>('');
  const [phone, setPhone] = React.useState<string>('');
  const [priority, setPriority] = React.useState<string>('1');

  const { email } = accountStore();

  let router = useRouter();

  const handleSubmit = async (e: React.SyntheticEvent) => {
    e.preventDefault();

    const url = `//${window.location.host}/api/customers/create`;

    try {
      let res = await fetch(url, {
        method: 'POST',
        mode: 'cors',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          firstName: firstName,
          lastName: lastName,
          email: custEmail,
          phone: phone,
          priority: Number(priority),
          userEmail: email,
        }),
      });

      if (res.ok) {
        router.push('/dashboard/customers');
      }
    } catch (e: any) {
      console.log(`Error: ${e}`);
    }
  };

  return (
    <>
      <Layout>
        <form className="min-h-screen flex flex-col items-center justify-center bg-gray-100" onSubmit={handleSubmit}>
          <div
            className="
          flex flex-col
          bg-white
          shadow-md
          px-4
          sm:px-6
          md:px-8
          lg:px-10
          py-8
          rounded-md
          w-50
          max-w-md"
          >
            <h1 className="lg:text-2xl text-xl text-center">New Customer</h1>
            <fieldset className="mt-10">
              <label htmlFor="first_name" className="text-xs tracking-wide text-gray-600">
                First name:
                <input
                  type="text"
                  name="first_name"
                  className="text-sm mb-4
                    placeholder-gray-500
                    px-4
                    rounded-md
                    border border-gray-400
                    w-full
                    py-2
                    focus:outline-none focus:border-black"
                  value={firstName}
                  onInput={(e) => setFirstName((e.target as HTMLInputElement).value)}
                  placeholder="Enter first name"
                />
              </label>

              <label htmlFor="last_name" className="text-xs tracking-wide text-gray-600">
                Last name:
                <input
                  type="text"
                  name="last_name"
                  className="text-sm mb-4
                    placeholder-gray-500
                    px-4
                    rounded-md
                    border border-gray-400
                    w-full
                    py-2
                    focus:outline-none focus:border-black"
                  value={lastName}
                  onInput={(e) => setLastName((e.target as HTMLInputElement).value)}
                  placeholder="Enter last name"
                />
              </label>
              <label htmlFor="email" className="text-xs tracking-wide text-gray-600">
                Email address:
                <input
                  type="email"
                  name="email"
                  className="text-sm mb-4
                    placeholder-gray-500
                    px-4
                    rounded-md
                    border border-gray-400
                    w-full
                    py-2
                    focus:outline-none focus:border-black"
                  value={custEmail}
                  onInput={(e) => setCustEmail((e.target as HTMLInputElement).value)}
                  placeholder="Enter email address"
                />
              </label>
              <label htmlFor="phone" className="text-xs tracking-wide text-gray-600">
                <span>Mobile number: </span>
                <input
                  type="text"
                  name="phone"
                  className="text-sm mb-4
                    placeholder-gray-500
                    px-4
                    rounded-md
                    border border-gray-400
                    w-full
                    py-2
                    focus:outline-none focus:border-black"
                  value={phone}
                  onInput={(e) => setPhone((e.target as HTMLInputElement).value)}
                  placeholder="Enter mobile number"
                />
              </label>
              <label htmlFor="priority" className="text-xs tracking-wide text-gray-600">
                <span>Priority: </span>
                <select
                  name="priority"
                  className="text-sm mb-4
                    placeholder-gray-500
                    px-4
                    rounded-md
                    border border-gray-400
                    w-full
                    py-2
                    focus:outline-none focus:border-black bg-white"
                  value={priority}
                  onChange={(e) => setPriority((e.target as HTMLSelectElement).value)}
                >
                  <option value="1">Very Low</option>
                  <option value="2">Low</option>
                  <option value="3">Medium</option>
                  <option value="4">High</option>
                  <option value="5">Very High</option>
                </select>
              </label>
              <div className="flex w-full">
                <button
                  type="submit"
                  className="
                  flex
                  mt-2
                  items-center
                  justify-center
                  focus:outline-none
                  text-white text-sm
                  sm:text-base
                  bg-black
                  hover:bg-slate-950
                  rounded-md
                  py-2
                  w-full
                  transition
                  duration-150
                  ease-in
                "
                >
                  <span className="mr-2 uppercase">Create</span>
                </button>
              </div>
            </fieldset>
          </div>
        </form>
      </Layout>
    </>
  );
}
