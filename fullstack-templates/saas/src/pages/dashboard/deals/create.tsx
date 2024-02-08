import Layout from '../../../components/Layout';
import React from 'react';
import { useRouter } from 'next/router';
import { accountStore } from '@/stores/zustandStore';

type Custname = {
  id: number;
  customer_name: string;
};

export default function CreateDeal() {
  const [estimate, setEstimate] = React.useState<string>('');
  const [custId, setCustId] = React.useState<string>('');
  const [custnames, setCustnames] = React.useState<Custname[]>([]);
  const { email } = accountStore();

  let router = useRouter();

  const handleSubmit = async (e: React.SyntheticEvent) => {
    e.preventDefault();
    const url = `//${window.location.host}/api/deals/create`;

    try {
      const res = await fetch(url, {
        method: 'POST',
        mode: 'cors',
        headers: new Headers({
          'Content-Type': 'application/json',
        }),

        body: JSON.stringify({
          estimatedworth: parseInt(estimate),
          cust_id: parseInt(custId),
          useremail: email,
        }),
      });

      if (res.status == 403) {
        return router.push('/login');
      }

      router.push('/dashboard/deals');
    } catch (e: any) {
      console.log(`Error: ${e}`);
    }
  };

  React.useEffect(() => {
    const fetchData = async () => {
      const url = `//${window.location.host}/api/customers/names`;

      try {
        const res = await fetch(url, {
          method: 'POST',
          mode: 'cors',
          headers: new Headers({
            'Content-Type': 'application/json',
          }),

          body: JSON.stringify({
            email: email,
          }),
        });

        if (res.status == 403) {
          return router.push('/login');
        }

        const data = await res.json();

        setCustnames(data);
      } catch (e: any) {
        console.log(`Error: ${e}`);
      }
    };
    fetchData();
  }, [email, router]);

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
            <h1 className="lg:text-2xl text-xl text-center">New Deal</h1>
            <fieldset className="mt-10">
              <label htmlFor="estimated_worth">
                Total to be invoiced:
                <input
                  type="text"
                  name="estimated_worth"
                  className="text-sm mb-4
                    placeholder-gray-500
                    px-4
                    rounded-md
                    border border-gray-400
                    w-full
                    py-2
                    focus:outline-none focus:border-black"
                  value={estimate}
                  onInput={(e) => setEstimate((e.target as HTMLInputElement).value)}
                  placeholder="Enter amount"
                ></input>
              </label>
              <label htmlFor="custId">
                Customer:
                <select
                  value={custId}
                  onChange={(e) => setCustId((e.target as HTMLSelectElement).value)}
                  className="text-sm mb-4
                    placeholder-gray-500
                    px-4
                    rounded-md
                    border border-gray-400
                    w-full
                    py-2
                    focus:outline-none focus:border-black bg-white"
                >
                  <option value=""></option>
                  {custnames
                    ? custnames.map((cust) => (
                        <option value={cust.id} key={cust.id}>
                          {cust.customer_name}
                        </option>
                      ))
                    : null}
                </select>
              </label>
              <button
                type="submit"
                className=" flex
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
                  ease-in"
              >
                <span className="mr-2 uppercase">Create</span>
              </button>
            </fieldset>
          </div>
        </form>
      </Layout>
    </>
  );
}
