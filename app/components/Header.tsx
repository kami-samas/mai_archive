import { Button, Heading, Icon, Stack, useColorMode, Text, useColorModeValue } from '@chakra-ui/react'
import { SunIcon, MoonIcon } from '@chakra-ui/icons'
import { User } from '../helpers/interfaces';
import { useState, useEffect } from 'react';
import { get_am_pm } from '../helpers/api';

export const Header = ({ user }: { user: User }) => {
  const { colorMode, toggleColorMode } = useColorMode();
  const [am_pm, setAmPm] = useState<string>('Good Morning');
  useEffect(() => {
    get_am_pm().then(val => setAmPm((val as any).message as string));
  }
  , []);
  return (
    <Stack>
      <Stack
        flex={{ base: 1, md: 0 }}
        justify="flex-end"
        direction="row"
        spacing={6}
        padding={6}
        borderBlockEnd="1px"
        borderBlockEndColor={useColorModeValue('gray.200', 'gray.700')}
      >
        <Heading as="h1" size="lg" variant={"ghost"}>
          <Text
            bgGradient="linear(to-r, red.400,pink.400)"
            bgClip="text">
              {am_pm}{user.username ? `, ${user.username}` : ''}
          </Text>
        </Heading>
        <Button onClick={toggleColorMode} variant="ghost">
          {colorMode === "light" ? <Icon as={MoonIcon} /> : <Icon as={SunIcon} />}
        </Button>
        {user.id}
      </Stack>
    </Stack>
  )
}
