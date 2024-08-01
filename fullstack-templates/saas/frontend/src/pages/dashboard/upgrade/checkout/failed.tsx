import Layout from "@/components/Layout"
import React from "react"
import {useRouter} from 'next/router'

export default function PaymentSuccess() {

  return (
    <>
      <Layout>
        <h1 className="text-2xl py-20 text-center">Payment has failed :(</h1>
    </Layout>
  </>
  )
}


