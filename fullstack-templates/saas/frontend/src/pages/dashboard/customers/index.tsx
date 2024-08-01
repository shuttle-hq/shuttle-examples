import Layout from '../../../components/Layout';
import React from 'react';
import { useRouter } from 'next/router';
import { accountStore } from '@/stores/zustandStore';
import Link from 'next/link';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faPlus, faTrash } from '@fortawesome/free-solid-svg-icons';

interface Customer {
  id: number;
  firstname: string;
  lastname: string;
  email: string;
  phone: string;
}

export default function CustomerIndex() {
  const [data, setData] = React.useState<Customer[]>([]);
  const { email } = accountStore();

  let router = useRouter();

  React.useEffect(() => {
    const fetchData = async () => {
      const url = `//${window.location.host}/api/customers`;

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

        setData(data);
      } catch (e: any) {
        console.log(`Error: ${e}`);
      }
    };
    fetchData();
  }, [email, router]);

  const handleDelete = async (customerId: number) => {
    const url = `//${window.location.host}/api/customers/${customerId}`;

    try {
      const res = await fetch(url, {
        mode: 'cors',
        method: 'DELETE',
        headers: new Headers({
          'Content-Type': 'application/json',
        }),
        body: JSON.stringify({
          email: email,
        }),
      });

      if (res.ok) {
        router.reload();
      }
    } catch (e: any) {
      console.log(`Error: ${e}`);
    }
  };

  return (
    <Layout>
      <div className="py-10 flex flex-col gap-4 w-full px-5 md:px-24 items-start">
        <div>
          <h2 className="text-2xl font-semibold leading-tight my-10">Customers</h2>
        </div>
        <table className="min-w-full leading-normal">
          <thead>
            <tr>
              <th className="px-5 py-3 border-b-2 border-gray-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                Customer
              </th>
              <th className="px-5 py-3 border-b-2 border-gray-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                Email
              </th>
              <th className="px-5 py-3 border-b-2 border-gray-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                Phone Number
              </th>
              <th className="px-5 py-3 border-b-2 border-gray-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                More Info
              </th>
            </tr>
          </thead>
          <tbody>
            {data.map((cust) => (
              <tr key={cust.id}>
                <td className="px-5 py-5 border-b border-gray-200 bg-white text-sm">
                  <div className="flex items-center">
                    <div className="ml-3">
                      <p className="text-gray-900 whitespace-no-wrap">
                        {cust.firstname} {cust.lastname}
                      </p>
                    </div>
                  </div>
                </td>
                <td className="px-5 py-5 border-b border-gray-200 bg-white text-sm">
                  <p className="text-gray-900 whitespace-no-wrap">{cust.email}</p>
                </td>
                <td className="px-5 py-5 border-b border-gray-200 bg-white text-sm">
                  <p className="text-gray-900 whitespace-no-wrap">{cust.phone}</p>
                </td>
                <td className="px-5 py-5 border-b border-gray-200 bg-white text-sm">
                  <button
                    data-id={cust.id}
                    onClick={() => handleDelete(cust.id)}
                    className="px-5 py-2 hover:bg-red-700 transition-all mt-4 rounded text-white bg-red-500"
                  >
                    <FontAwesomeIcon icon={faTrash} color="white" /> Delete Customer
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
        <Link
          href="/dashboard/customers/create"
          className="transition-all mt-4 bg-black hover:bg-slate-950 text-white font-bold py-2 px-4 rounded"
        >
          <FontAwesomeIcon icon={faPlus} color="white" /> Add Customer
        </Link>
      </div>
    </Layout>
  );
}
