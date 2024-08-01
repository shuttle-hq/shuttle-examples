import Layout from '../../components/Layout';
import React from 'react';
import { useRouter } from 'next/router';
import { accountStore } from '@/stores/zustandStore';
import SalesChart from '@/components/SalesChart';
import RecentSales from '@/components/RecentSales';
import TopClients from '@/components/TopClients';

interface dashboardData {
  sales_deals_info: salesDealsInfo;
  sales_per_day_info: salesPerDayInfo;
}

interface salesDealsInfo {
  open: number;
  ready: number;
  awaitingresponse: number;
  closed: number;
  total_amt_closed: number;
}

interface salesPerDayInfo {
  date: String;
  sales_total: number;
}

export default function Dashboard() {
  const [data, setData] = React.useState<dashboardData>();
  const { email } = accountStore();
  const router = useRouter();

  React.useEffect(() => {
    const fetchData = async () => {
      const url = `//${window.location.host}/api/dashboard`;

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

  if (!data) return <p>It looks like something went wrong when retrieving data :(</p>;

  return (
    <Layout>
      <div className="flex flex-col gap-8 pt-10 justify-center bg-slate-50 items-center pb-10 overflow-y-scroll">
        <div className="w-full px-5 sm:px-24 py-10">
          <SalesChart />

          <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
            <RecentSales />
            <TopClients />
          </div>
        </div>
      </div>
    </Layout>
  );
}
