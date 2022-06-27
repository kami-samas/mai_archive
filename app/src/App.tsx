import {
  Box
} from '@chakra-ui/react';
import { Header, Signup } from '../components';
import { useLocalForage } from '../helpers/hooks/useLocalForage';

export default function App() {
  const [user, setUser, userLoading] = useLocalForage('user');

  return (
    <Box>
      <Header user/>
      <Signup />
    </Box>
  );
}
