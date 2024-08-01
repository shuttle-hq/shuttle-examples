import { faker } from '@faker-js/faker';
import Image from 'next/image';

export default function TopClients() {
  return (
    <div className="rounded-md shadow-md p-6 flex flex-col gap-4 w-full items-start bg-white">
      <h2 className="text-2xl font-bold leading-tight my-2">Top Clients</h2>
      <div className="justify-center items-center w-full">
        {new Array(7).fill(0).map((_, i) => (
          <div key={i} className="px-2 py-4 w-full flex justify-between items-center">
            <div className="flex gap-4 items-center">
              <Image
                src={faker.image.people(40, 40, true)}
                width={40}
                height={40}
                alt="Avatar"
                className="rounded-full"
              />

              <p className="flex flex-col gap-1">
                <span className="text-slate-900 text-xs">
                  {faker.name.firstName()} {faker.name.lastName()}
                </span>
                <span className="text-slate-500 text-xs">{faker.internet.email()}</span>
              </p>
            </div>

            <p className="text-slate-900 text-md font-semibold">£{faker.finance.amount()}</p>
          </div>
        ))}
      </div>
    </div>
  );
}
