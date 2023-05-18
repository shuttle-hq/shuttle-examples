import { faker } from '@faker-js/faker';

export default function Metrics() {
  return (
    <div className="flex flex-col gap-4 justify-between w-10">
      <div className="flex flex-col justify-center items-center shadow-md rounded-md transition-all px-10 py-4">
        <p className="text-xl font-semibold text-slate-900">{faker.finance.amount()}</p>
        <h2 className="text-sm text-slate-400">Leads</h2>
      </div>
      <div className="flex flex-col justify-center items-center shadow-md rounded-md transition-all px-10 py-4">
        <p className="text-xl font-semibold text-slate-900">{faker.finance.amount()}</p>
        <h2 className="text-sm text-slate-400">Open Sales Deals</h2>
      </div>
      <div className="flex flex-col justify-center items-center shadow-md rounded-md transition-all px-10 py-4 ">
        <p className="text-xl font-semibold text-slate-900">£{faker.finance.amount()}</p>
        <h2 className="text-sm text-slate-400">Closed this week:</h2>
      </div>
      <div className="flex flex-col justify-center items-center shadow-md rounded-md transition-all px-10 py-4">
        <p className="text-xl font-semibold text-slate-900">£{faker.finance.amount()}</p>
        <h2 className="text-sm text-slate-400">Closed this month:</h2>
      </div>
    </div>
  );
}
