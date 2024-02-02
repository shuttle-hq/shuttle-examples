import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { UserButton } from "@clerk/clerk-react";
import { Loader } from "lucide-react";
import useSWR from "swr";

export interface UserSchema {
  first_name: string;
  last_name: string;
  username: string;
  email_addresses: { email_address: string; id: string }[];
  profile_image_url: string;
}

export default function UsersTable() {
  const { isLoading, data } = useSWR("/api/users", (url) => fetch(url).then(res => res.json()));

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
            {data.map((user: UserSchema, i: number) => (
              <TableRow key={i}>
                <TableCell>
                  <Avatar>
                    <AvatarImage src={user?.profile_image_url} />
                    <AvatarFallback>{user?.username}</AvatarFallback>
                  </Avatar>
                </TableCell>
                <TableCell>{user?.first_name}</TableCell>
                <TableCell>{user?.last_name}</TableCell>
                <TableCell>{user?.username}</TableCell>
                <TableCell>{user?.email_addresses?.[0].email_address}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      )}
    </>
  );
}
