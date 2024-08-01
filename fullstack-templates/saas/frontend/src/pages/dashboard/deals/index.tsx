import Layout from '../../../components/Layout';
import React from 'react';
import { useRouter } from 'next/router';
import { accountStore } from '@/stores/zustandStore';
import Link from 'next/link';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faPlus } from '@fortawesome/free-solid-svg-icons';

interface Deal {
  id: number;
  estimate_worth: number;
  actual_worth: number;
  status: string;
  closed: string;
  customer_id: number;
  customer_name: String;
}

export default function DealIndex() {
  const [data, setData] = React.useState<Deal[]>([]);
  const { email } = accountStore();

  let router = useRouter();

  const handleStatus = async (e: React.SyntheticEvent) => {
    e.preventDefault();

    let element = e.target as HTMLSelectElement;

    let id = element.getAttribute('data-id');
    let status = element.value;

    const url = `//${window.location.host}/api/deals/${id}`;

    try {
      const res = await fetch(url, {
        method: 'PUT',
        mode: 'cors',
        headers: new Headers({
          'Content-Type': 'application/json',
        }),
        body: JSON.stringify({
          new_value: status,
          email: email,
        }),
      });

      if (res.ok) {
        element.value = status;
      }
    } catch (e: any) {
      console.log(e.message);
    }
  };

  React.useEffect(() => {
    const fetchData = async () => {
      const url = `//${window.location.host}/api/deals`;

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

  return (
    <Layout>
      <div className="px-5 py-10 flex flex-col gap-4 w-full md:px-24 items-start">
        <div>
          <h2 className="text-2xl font-semibold leading-tight my-10">Deals</h2>
        </div>

        <table className="min-w-full leading-normal">
          <thead>
            <tr>
              <th className="px-5 py-3 border-b-2 border-gray-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                Customer
              </th>
              <th className="px-5 py-3 border-b-2 border-gray-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                Invoice Amount
              </th>
              <th className="px-5 py-3 border-b-2 border-gray-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                Status
              </th>
            </tr>
          </thead>

          <tbody>
            {data.map((deal) => (
              <tr key={deal.id}>
                <td className="px-5 py-5 border-b border-gray-200 bg-white text-sm">
                  <div className="flex items-center">
                    <div className="ml-3">
                      <p className="text-gray-900 whitespace-no-wrap">{deal.customer_name}</p>
                    </div>
                  </div>
                </td>

                <td className="px-5 py-5 border-b border-gray-200 bg-white text-sm">
                  <p className="text-gray-900 whitespace-no-wrap">£{deal.estimate_worth}.00</p>
                </td>
                <td className="px-5 py-5 border-b border-gray-200 bg-white text-sm">
                  <select
                    className="text-gray-900 whitespace-no-wrap bg-white"
                    data-id={deal.id}
                    value={deal.status}
                    onChange={(e: React.SyntheticEvent) => handleStatus(e)}
                  >
                    <option value="open">Open</option>
                    <option value="awaitingresponse">Awaiting Response</option>
                    <option value="ready">Ready to close</option>
                    <option value="closed">Closed</option>
                  </select>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
        <Link
          href="/dashboard/deals/create"
          className="transition-all mt-4 bg-black hover:bg-slate-950 text-white font-bold py-2 px-4 rounded"
        >
          <FontAwesomeIcon icon={faPlus} color="white" /> Create Deal
        </Link>
      </div>
    </Layout>
  );
}
