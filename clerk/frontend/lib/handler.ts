export const getUsers = async () => {
    return await (await fetch(process.env.NEXT_PUBLIC_BASE_URL + "/api/users")).json()
}