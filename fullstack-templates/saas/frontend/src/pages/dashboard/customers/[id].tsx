import Layout from "@/components/Layout"
import React from "react"
import { useRouter } from 'next/router'
import { accountStore } from "@/stores/zustandStore"

interface Customer {
    id: number,
    firstname: string,
    lastname: string,
    email: string,
    phone: string
}

export default function CustomerSingle() {

    const [data, setData] = React.useState<Customer>();
    const {email} = accountStore();

    let router = useRouter();
    const {id} = router.query;

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
                router.push("/dashboard/customers");
            }
        } catch (e: any) {
            console.log(`Error: ${e}`)
        }

    }

    React.useEffect(() => {
        const fetchData = async () => {

            const url = `//${window.location.host}/api/customers/${id}`

            try {
                const res = await fetch(url,
                    {
                        method: "POST",
                        mode: "cors",
                        headers: new Headers({
                            "Content-Type": "application/json"
                        }),

                        body: JSON.stringify({
                            email: email
                        })
                    },
                );

                if (res.status == 403) {
                    return router.push("/login");
                }

                const data: Customer = await res.json()

                setData(data);

            } catch (e: any) {
                console.log(`Error: ${e}`);
            }
        };
        fetchData()
    }, [email, router, id]);


    return (
        <Layout>
            <div className="py-10 flex flex-col items-center gap-4">
                {data ?
                    <div className="px-10 py-4 bg-stone-200 flex flex-col gap-2 w-full">
                        <p className="text-xl"> {data.firstname} {data.lastname} </p>
                        <p> Email: {data.email} </p>
                        <p> Phone: {data.phone} </p>
                        <button onClick={(e) => handleDelete(e)} className="px-5 py-2">Delete Customer</button>
                    </div>
                    :
                    <p>User does not exist :(</p>}

            </div>

        </Layout>
    )
}



