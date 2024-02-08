import React from 'react';
import { useRouter } from 'next/router';
import { accountStore } from '@/stores/zustandStore';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faMultiply } from '@fortawesome/free-solid-svg-icons';

interface Customer {
    id: number;
    firstname: string;
    lastname: string;
    email: string;
    phone: string;
}

type Props = {
    id: number;
    data: Customer[];
    vis: boolean;
    setVis: React.Dispatch<React.SetStateAction<boolean>>;
};

export default function CustomerSingle({data, id, vis, setVis}: Props) {
    const {email} = accountStore();

    let router = useRouter();

    const handleDelete = async (e: React.SyntheticEvent) => {
        e.preventDefault();
        const url = `//${window.location.host}/api/customers/${id}`;

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

    if (!vis) return null;

    const customer = data.find((a) => a.id == id);

    if (!customer) return <p>Customer does not exist :(</p>;

    return (
        <div className="w-full h-screen backdrop-blur z-50 absolute">
            <div
                className="relative py-8 px-5 md:px-10 bg-white shadow-md rounded border border-gray-400 flex items-center justify-center">
                <div>
                    <h1 className="text-gray-800 font-lg font-bold tracking-normal leading-tight mb-4">Customer
                        details</h1>

                    <button onClick={() => setVis(false)} className="text-right">
                        <FontAwesomeIcon
                            icon={faMultiply}
                            className="text-2xl hover:text-red-500 transition-all"
                            color="rgb(59 130 246)"
                        />
                    </button>
                </div>
                <div className="space-y-2">
                    <p className="text-xl">
                        Name: {customer.firstname} {customer.lastname}
                    </p>
                    <p> Email: {customer.email} </p>
                    <p> Phone: {customer.phone} </p>
                </div>
                <button onClick={(e) => handleDelete(e)} className="bg-[#EF924C] px-5 py-2 text-white">
                    Delete Customer
                </button>
            </div>
        </div>
    );
}
