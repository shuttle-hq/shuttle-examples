import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { useAuth, UserButton } from "@clerk/clerk-react";
import { Loader } from "lucide-react";
import useSWR from "swr";

export interface UserSchema {
  first_name: string;
  last_name: string;
  username: string;
  email_addresses: { email_address: string; id: string }[];
  profile_image_url: string;
  has_image: string;
}

const getUsers = async (token: string) => {
  const res = await (
    await fetch(import.meta.env.VITE_API_BASE_URL + "/users", {
      method: "GET",
      headers: {
        Authorization: "Bearer " + token,
      },
    })
  ).json();

  return res;
};

export default function UsersTable() {
  const { getToken } = useAuth();

  const { isLoading, data } = useSWR("/api/users", async () => {
    const token = await getToken();
    return getUsers(token as string);
  });

  console.log(data);

  return (
    <>
      <div className="fixed top-6 right-6">
        <UserButton afterSignOutUrl="/" />
      </div>
      {isLoading && <Loader className="w-4 h-4 animate-spin" />}
      {!isLoading && (
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Avatar</TableHead>
              <TableHead>First Name</TableHead>
              <TableHead>Last Name</TableHead>
              <TableHead>Username</TableHead>
              <TableHead>Email</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {data.data.map((user: UserSchema, id: string) => (
              <TableRow key={id}>
                <TableCell>
                  <Avatar>
                    <AvatarImage src={user.profile_image_url} />
                    <AvatarFallback>{user.username.slice(0, 2)}</AvatarFallback>
                  </Avatar>
                </TableCell>
                <TableCell>{user.first_name}</TableCell>
                <TableCell>{user.last_name}</TableCell>
                <TableCell>{user.username}</TableCell>
                <TableCell>{user.email_addresses[0].email_address}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      )}
    </>
  );
}
