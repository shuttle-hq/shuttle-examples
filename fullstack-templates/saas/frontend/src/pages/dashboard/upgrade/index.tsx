import Layout from '@/components/Layout';
import Link from 'next/link';

export default function Home() {
  return (
    <>
      <Layout>
        <section className="py-10 flex flex-col justify-center items-center w-full">
          <h1 className="text-xl lg:text-3xl text-center my-10">Pricing</h1>

          <div className="">
            <div className="container px-6 py-8 mx-auto">
              <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
                <div className="flex flex-col w-full max-w-sm p-8 space-y-8 text-center bg-white border border-gray-200 rounded-md ">
                  <div className="flex-shrink-0">
                    <h2 className="inline-flex items-center justify-center px-2 font-semibold tracking-tight text-black uppercase rounded bg-gray-50">
                      Casual
                    </h2>
                  </div>
                  <div className="flex-shrink-0">
                    <span className="pt-2 text-4xl font-bold text-gray-800 uppercase">Free</span>
                  </div>
                  <ul className="flex-1 space-y-4">
                    <li className="text-gray-500">Up to 5 projects</li>
                    <li className="text-gray-500">Up to 10 collaborators</li>
                    <li className="text-gray-500">2Gb of storage</li>
                  </ul>

                  <span className="h-10 flex justify-center items-center">Your plan</span>
                </div>

                <div className="flex flex-col w-full max-w-sm p-8 space-y-8 text-center bg-white border-2 border-gray-200 rounded-md  ">
                  <div className="flex-shrink-0">
                    <h2 className="inline-flex items-center justify-center px-2 font-semibold tracking-tight text-black uppercase rounded bg-gray-50">
                      Professional
                    </h2>
                  </div>
                  <div className="flex-shrink-0">
                    <span className="pt-2 text-4xl font-bold text-gray-800 uppercase">$24.90</span>
                    <span className="text-gray-500">/month</span>
                  </div>
                  <ul className="flex-1 space-y-4">
                    <li className="text-gray-500">Up to 10 projects</li>
                    <li className="text-gray-500">Up to 20 collaborators</li>
                    <li className="text-gray-500">10Gb of storage</li>
                    <li className="text-gray-500">Real-time collaborations</li>
                  </ul>

                  <Link
                    href="/dashboard/upgrade/checkout"
                    className="inline-flex items-center justify-center px-4 py-2 font-semibold text-white uppercase transition-colors bg-black rounded hover:bg-slate-950 focus:outline-none"
                  >
                    Upgrade
                  </Link>
                </div>

                <div className="flex flex-col w-full max-w-sm p-8 space-y-8 text-center bg-white border-2 border-gray-200 rounded-md  ">
                  <div className="flex-shrink-0">
                    <h2 className="inline-flex items-center justify-center px-2 font-semibold tracking-tight text-black uppercase rounded bg-gray-50 ">
                      Expert
                    </h2>
                  </div>
                  <div className="flex-shrink-0">
                    <span className="pt-2 text-4xl font-bold text-gray-800 uppercase ">$49.90</span>
                    <span className="text-gray-500 ">/month</span>
                  </div>
                  <ul className="flex-1 space-y-4">
                    <li className="text-gray-500">Unlimited projects</li>
                    <li className="text-gray-500">Unlimited collaborators</li>
                    <li className="text-gray-500">Unlimited storage</li>
                    <li className="text-gray-500">Real-time collaborations</li>
                    <li className="text-gray-500">24x7 Support</li>
                  </ul>

                  <Link
                    href="/dashboard/upgrade/checkout"
                    className="inline-flex items-center justify-center px-4 py-2 font-semibold text-white uppercase transition-colors bg-black rounded hover:bg-slate-950 focus:outline-none"
                  >
                    Upgrade
                  </Link>
                </div>
              </div>
            </div>
          </div>
        </section>
      </Layout>
    </>
  );
}
