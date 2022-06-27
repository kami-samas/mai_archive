import { Button, Heading, Icon, Stack, useColorMode, Text } from '@chakra-ui/react'
import { SunIcon, MoonIcon } from '@chakra-ui/icons'

export const Header = (props: any) => {
  const { colorMode, toggleColorMode } = useColorMode();
  const user = props.user;
  
  return (
    <Stack>
      <Stack
        flex={{ base: 1, md: 0 }}
        justify="flex-end"
        direction="row"
        spacing={6}
        padding={6}
      >
        <Heading as="h1" size="xl" variant={"ghost"}>
          <Text
            bgGradient="linear(to-r, red.400,pink.400)"
            bgClip="text">
              {user.name ? user.name : ''}
          </Text>
        </Heading>
        <Button onClick={toggleColorMode} variant="ghost">
          {colorMode === "light" ? <Icon as={MoonIcon} /> : <Icon as={SunIcon} />}
        </Button>
      </Stack>
    </Stack>
  )
}
