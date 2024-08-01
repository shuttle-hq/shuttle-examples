import Layout from '@/components/Layout';
import React from 'react';
import { useRouter } from 'next/router';

export default function CreateCustomer() {
    const [name, setName] = React.useState<string>('');
    const [email, setEmail] = React.useState<string>('');
    const [card, setCard] = React.useState<string>('');
    const [year, setYear] = React.useState<string>('');
    const [month, setMonth] = React.useState<string>('');
    const [cvc, setCvc] = React.useState<string>('');

    let router = useRouter();

    const handleSubmit = async (e: React.SyntheticEvent) => {
        e.preventDefault();

        const url = `//${window.location.host}/api/payments/pay`;

        try {
            let res = await fetch(url, {
                method: 'POST',
                mode: 'cors',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    name: name,
                    email: email,
                    card: card,
                    expyear: parseInt(year),
                    expmonth: parseInt(month),
                    cvc: cvc,
                }),
            });

            if (res.ok) {
                router.push('/dashboard/upgrade/checkout/success');
            }
        } catch (e: any) {
            console.log(`Error: ${e}`);
        }
    };

    return (
        <Layout>
            <form className="px-5 min-h-screen flex flex-col items-center justify-center bg-gray-100"
                  onSubmit={handleSubmit}>
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
                    <h1 className="lg:text-2xl text-xl text-center font-bold">Checkout</h1>

                    <fieldset className="mt-10">
                        <label htmlFor="name" className="text-xs tracking-wide text-gray-600">
                            Name:{' '}
                        </label>
                        <input
                            type="text"
                            name="name"
                            required
                            className="text-sm
                    placeholder-gray-500
                    px-4
                    rounded-md
                    border border-gray-400
                    w-full
                    py-2
                    focus:outline-none focus:border-black mb-4"
                            value={name}
                            onInput={(e) => setName((e.target as HTMLInputElement).value)}
                            placeholder="Enter your full name"
                        />
                        <label htmlFor="email" className="text-xs tracking-wide text-gray-600">
                            Payment e-mail:
                        </label>
                        <input
                            type="email"
                            name="email"
                            required
                            className="text-sm
                    placeholder-gray-500
                    px-4
                    rounded-md
                    border border-gray-400
                    w-full
                    py-2
                    focus:outline-none focus:border-black mb-4"
                            value={email}
                            onInput={(e) => setEmail((e.target as HTMLInputElement).value)}
                            placeholder="Enter your email"
                        />
                        <label htmlFor="card" className="text-xs tracking-wide text-gray-600">
                            Card number:
                        </label>
                        <input
                            type="text"
                            name="card"
                            required
                            className="text-sm
                    placeholder-gray-500
                    px-4
                    rounded-md
                    border border-gray-400
                    w-full
                    py-2
                    focus:outline-none focus:border-black bg-white mb-4"
                            maxLength={16}
                            minLength={16}
                            value={card}
                            onInput={(e) => setCard((e.target as HTMLInputElement).value)}
                            placeholder="Enter your card number"
                        />
                        <label htmlFor="exp_year" className="text-xs tracking-wide text-gray-600">
                            Expiry year:
                        </label>

                        <select
                            className="text-sm
                    placeholder-gray-500
                    px-4
                    rounded-md
                    border border-gray-400
                    w-full
                    py-2
                    focus:outline-none focus:border-black bg-white mb-4"
                            id="year"
                            name="exp_year"
                            required
                            onChange={(e) => setYear((e.target as HTMLSelectElement).value)}
                        >
                            <option value="" disabled selected>
                                Select year
                            </option>
                            <option value="2023">2023</option>
                            <option value="2024">2024</option>
                            <option value="2025">2025</option>
                            <option value="2026">2026</option>
                            <option value="2027">2027</option>
                            <option value="2028">2028</option>
                            <option value="2029">2029</option>
                            <option value="2030">2030</option>
                            <option value="2031">2031</option>
                            <option value="2032">2032</option>
                            <option value="2033">2033</option>
                            <option value="2034">2034</option>
                            <option value="2035">2035</option>
                            <option value="2036">2036</option>
                            <option value="2037">2037</option>
                            <option value="2038">2038</option>
                            <option value="2039">2039</option>
                            <option value="2040">2040</option>
                            <option value="2041">2041</option>
                            <option value="2042">2042</option>
                            <option value="2043">2043</option>
                        </select>

                        <label htmlFor="exp_month" className="text-xs tracking-wide text-gray-600">
                            Expiry month:
                        </label>

                        <select
                            name="exp_month"
                            required
                            className="text-sm
                    placeholder-gray-500
                    px-4
                    rounded-md
                    border border-gray-400
                    w-full
                    py-2
                    focus:outline-none focus:border-black bg-white mb-4"
                            value={month}
                            onChange={(e) => setMonth((e.target as HTMLSelectElement).value)}
                        >
                            <option value="" disabled selected>
                                Select month
                            </option>
                            <option value="1">January</option>
                            <option value="2">February</option>
                            <option value="3">March</option>
                            <option value="4">April</option>
                            <option value="5">May</option>
                            <option value="6">June</option>
                            <option value="7">July</option>
                            <option value="8">August</option>
                            <option value="9">September</option>
                            <option value="10">October</option>
                            <option value="11">November</option>
                            <option value="12">December</option>
                        </select>
                        <label htmlFor="cvc" className="text-xs tracking-wide text-gray-600">
                            CVC:
                        </label>
                        <input
                            type="password"
                            name="cvc"
                            required
                            maxLength={3}
                            className="text-sm
                    placeholder-gray-500
                    px-4
                    rounded-md
                    border border-gray-400
                    w-full
                    py-2
                    focus:outline-none focus:border-black mb-4"
                            value={cvc}
                            onInput={(e) => setCvc((e.target as HTMLInputElement).value)}
                            placeholder="Enter your card CVC"
                        />

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
                                <span className="mr-2 uppercase">Submit &rarr;</span>
                            </button>
                        </div>
                    </fieldset>
                </div>
            </form>
        </Layout>
    );
}
