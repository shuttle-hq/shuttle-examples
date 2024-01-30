import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";

import { UserSchema } from "./page";

interface UsersTableProps {
  users: UserSchema[];
}
export default function UsersTable(props: UsersTableProps) {
  const { users } = props;
  return (
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead>Avatar</TableHead>
          <TableHead>First Name</TableHead>
          <TableHead>Last Name</TableHead>
          <TableHead>username</TableHead>
          <TableHead>email</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {users.map((user, id) => (
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
  );
}
