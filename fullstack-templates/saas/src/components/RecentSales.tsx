import { faker } from '@faker-js/faker';

export default function RecentSales() {
  return (
    <div className="rounded-md shadow-md p-6 flex flex-col gap-4 w-full items-start bg-white">
      <h2 className="text-2xl font-bold leading-tight my-2">Recent Sales</h2>

      <table className="w-full">
        <thead>
          <tr>
            <th className="px-5 py-3 border-b-2 border-gray-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
              Customer
            </th>
            <th className="px-5 py-3 border-b-2 border-gray-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
              Amount Invoiced
            </th>
          </tr>
        </thead>
        <tbody>
          {new Array(7).fill(0).map((_, i) => (
            <tr key={i}>
              <td className="px-5 py-5 border-b border-gray-200 text-sm">
                <div className="flex items-center">
                  <div className="ml-3">
                    <p className="text-gray-900 whitespace-no-wrap">
                      {faker.name.firstName()} {faker.name.lastName()}
                    </p>
                  </div>
                </div>
              </td>
              <td className="px-5 py-5 border-b border-gray-200 text-sm">
                <p className="text-gray-900 whitespace-no-wrap">£{faker.finance.amount()}</p>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
