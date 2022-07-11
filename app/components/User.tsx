import { User as UserType } from '../helpers/interfaces'
import { useState } from 'react'
import { Signin } from './SignIn'
import { Signup } from './SignUp'
import { Text, Heading, Stack } from '@chakra-ui/react'

export const User = ({ user, setUser }: { user: UserType; setUser: any }) => {
    const [create, setCreate] = useState<boolean>(true);
    return (
        <Stack
            flex={{ base: 1, md: 0 }}
            justify="flex-centre"
            direction="column"
            spacing={6}
            padding={6}
        >
            <Heading as="h1" size="xl" variant={"ghost"}>
                <Text
                    onClick={() => setCreate(!create)}
                    bgGradient="linear(to-r, red.400,pink.400)"
                    cursor={"pointer"}
                    bgClip="text">
                    {create ? 'Sign Up' : 'Sign In'}
                </Text>
            </Heading>
            {create ? <Signup setUser={setUser} /> : <Signin setUser={setUser} />}
        </Stack>
    )
}