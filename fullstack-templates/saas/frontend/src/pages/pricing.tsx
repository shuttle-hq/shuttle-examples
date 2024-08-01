import Layout from "@/components/Layout"
import Link from 'next/link'

export default function Home() {
  return (
  <>
      <Layout>
              <section className="my-10 flex flex-col justify-center w-full">
        <h1 className="text-2xl lg:text-5xl text-center">Pricing</h1>
          <div className="grid grid-cols-3 grid-rows-1 gap-10 mx-10 py-10 justify-items-center">
            <div className="col-span-1 w-2/3 grid grid-cols-1 grid-rows-10 justify-center gap-2 justify-items-center text-center bg-sky-200 py-10 rounded-md">
              
              <p className="lg:text-3xl text-xl row-span-2">Basic</p>
              <ul className="row-span-8 text-lg">
              <li>Lorem ipsum</li>
              <li>Lorem ipsum</li>
              <li>Lorem ipsum</li>
              <li>Lorem ipsum</li>
              <li>Lorem ipsum</li>
          </ul>
                  <Link href="/register" className="bg-stone-200 w-max px-10 py-4 self-center row-span-1 hover:bg-stone-100 transition-all">Get Started</Link>
              </div>
            <div className="col-span-1 w-2/3 grid grid-cols-1 grid-rows-10 justify-center gap-2 justify-items-center text-center bg-sky-200 py-10 rounded-md">
              
              <p className="lg:text-3xl text-xl row-span-2">Premium</p>
              <ul className="row-span-8 text-lg">
              <li>Everything in Basic!</li>
              <li>Lorem ipsum</li>
              <li>Lorem ipsum</li>
              <li>Lorem ipsum</li>
              <li>Lorem ipsum</li>
              <li>Lorem ipsum</li>
          </ul>
                  <Link href="/register" className="bg-stone-200 w-max px-10 py-4 self-center row-span-1 hover:bg-stone-100 transition-all">Get Started</Link>
              </div>

            <div className="col-span-1 w-2/3 grid grid-cols-1 grid-rows-10 justify-center gap-2 justify-items-center text-center bg-sky-200 py-10 rounded-md">
              
              <p className="lg:text-3xl text-xl row-span-2">Corporate</p>
              <ul className="row-span-8 text-lg">
                <li>Everything in Premium!</li>
              <li>Lorem ipsum</li>
              <li>Lorem ipsum</li>
              <li>Lorem ipsum</li>
              <li>Lorem ipsum</li>
              <li>Lorem ipsum</li>
          </ul>
                  <Link href="/register" className="bg-stone-200 w-max px-10 py-4 self-center row-span-1 hover:bg-stone-100 transition-all">Get Started</Link>
              </div>

      </div>
        </section>

    </Layout>
  </>
  )
}
