import {
  Box,
  Flex,
  Icon,
  useColorModeValue,
  Link,
  useDisclosure,
  BoxProps,
  FlexProps,
} from '@chakra-ui/react';
import {
  FiCompass,
  FiSettings,
  FiHome
} from 'react-icons/fi';
import { GoSignOut } from 'react-icons/go';
import { IconType } from 'react-icons';

interface LinkItemProps {
  name: string;
  icon: IconType;
  execute: (props?: any) => any | undefined;
}
const LinkItems: Array<LinkItemProps> = [
  { name: 'Home', icon: FiHome, execute: (setTab) => { setTab('Home') } },
  { name: 'Projects', icon: FiCompass, execute: (setTab) => { setTab('Projects') } },
  { name: 'Settings', icon: FiSettings, execute: (setTab) => { setTab('Settings') } },
  {
    name: 'Sign Out', icon: GoSignOut, execute: () => {
      // localforage.removeItem('user');
      // window.location.reload();
    }
  },
];

export const Sidebar = ({
  tab, setTab,
}: { tab: string; setTab: any; }) => {
  return (
    <>
      <SidebarContent
        tab={tab}
        setTab={setTab}
        display={{ base: 'none', md: 'block' }}
      />
    </>
  );
}

interface SidebarProps extends BoxProps {
  tab: string;
  setTab: any;
}

const SidebarContent = ({ tab, setTab, ...rest }: SidebarProps) => {
  return (
    <Box
      transition="ease"
      borderRight="1px"
      borderRightColor={useColorModeValue('gray.200', 'gray.700')}
      w={{ base: 'full', md: 60 }}
      pos="fixed"
      color={"primary.300"}
      h="full"
      {...rest}>
      {LinkItems.map((link) => (
        <NavItem key={link.name} p={6} color={tab === link.name ? "white" : ""} bgGradient={tab === link.name ? "linear(to-r, red.400,pink.400)" : ""} icon={link.icon} onClick={link.execute ? () => link.execute(setTab) : () => {}}>
          {link.name}
        </NavItem>
      ))}
    </Box>
  );
};

interface NavItemProps extends FlexProps {
  icon: IconType;
  children: String;
}
const NavItem = ({ icon, children, ...rest }: NavItemProps) => {
  return (
    <Link href="#" style={{ textDecoration: 'none' }} _focus={{ boxShadow: 'none' }}>
      <Flex
        align="center"
        mx="4"
        borderRadius="lg"
        role="group"
        cursor="pointer"
        _hover={{
          bgGradient: 'linear(to-r, red.500,pink.500)',
          color: 'white',
        }}
        {...rest}>
        {icon && (
          <Icon
            mr="4"
            fontSize="16"
            _groupHover={{
              color: 'white',
            }}
            as={icon}
          />
        )}
        {children}
      </Flex>
    </Link>
  );
};