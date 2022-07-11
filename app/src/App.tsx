import {
  Box
} from '@chakra-ui/react';
import { Header, User, Sidebar, Home } from '../components/index';
import { useLocalForage } from '../helpers/hooks/useLocalForage';
import { User as UserType } from '../helpers/interfaces';

export default function App() {
  const [user, setUser, _userLoading] = useLocalForage('user') as [UserType, any, boolean];
  return (
    <Box>
      <Header user={user} />
      {
        user.id ? <>
          <Sidebar />
          <Home />
        </> : <User user={user} setUser={setUser} />
      }
    </Box>
  );
}
