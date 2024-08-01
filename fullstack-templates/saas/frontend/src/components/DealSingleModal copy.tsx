import React from "react"
import { useRouter } from 'next/router'
import { accountStore } from '@/stores/zustandStore'

interface Customer {
    id: number,
    firstname: string,
    lastname: string,
    email: string,
    phone: string
}

type Props = {
    id: number,
    data: Customer[],
    vis: boolean,
    setVis: React.Dispatch<React.SetStateAction<boolean>>
}

export default function CustomerSingle({data, id, vis, setVis}: Props) {

    const {email} = accountStore();

    let router = useRouter();

    const handleDelete = async (e: React.SyntheticEvent) => {
        e.preventDefault()
        const url = `//${window.location.host}/api/customers/${id}`

        try {

            const res = await fetch(url, {
                mode: 'cors',
                method: 'DELETE',
                headers: new Headers({
                    "Content-Type": "application/json"
                }),
                body: JSON.stringify({
                    email: email
                })
            });

            if (res.ok) {
                router.reload()
            }
        } catch (e: any) {
            console.log(`Error: ${e}`)
        }

    }

    return (
        <>
            {vis ?
                <div className="w-screen h-screen backdrop-blur z-50 absolute">
                    <div className="py-10 flex flex-col items-center gap-4">
                        {data ?
                            data.filter(a => a.id == id).map((item) => (
                                <div key={item.id}
                                     className="px-10 py-4 bg-stone-200 flex rounded-md flex-col gap-2 w-4/5 h-[40rem]">
                                    <p className="text-xl"> {item.firstname} {item.lastname} </p>
                                    <p> Email: {item.email} </p>
                                    <p> Phone: {item.phone} </p>
                                    <button onClick={(e) => handleDelete(e)} className="px-5 py-2">Delete Customer
                                    </button>
                                </div>))
                            :
                            <p>Customer does not exist :(</p>}

                    </div>
                </div> : null}
        </>
    )
}



