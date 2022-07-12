import {
  Box
} from '@chakra-ui/react';
import { useState } from 'react';
import { Header, User, Sidebar, Home, Settings } from '../components';
import { useLocalForage } from '../helpers/hooks/useLocalForage';
import { User as UserType } from '../helpers/interfaces';

export default () => {
  const [user, setUser, _userLoading] = useLocalForage('user') as [UserType, any, boolean];
  const [tab, setTab] = useState('Home');
  const renderTab = () => {
    switch (tab) {
      case 'Home':
        return <Home />;
      case 'Settings':
        return <Settings />;
    }
  }
  return (
    <Box>
      <Header user={user} />
      {
        user.id ? <>
          <Sidebar tab={tab} setTab={setTab} />
          {renderTab()}
        </> : <User user={user} setUser={setUser} />
      }
    </Box>
  );
}
