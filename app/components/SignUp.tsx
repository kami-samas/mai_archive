import {
    FormControl,
    FormLabel,
    FormHelperText,
    Stack,
    Input,
    Text,
    Button,
    useToast,
} from '@chakra-ui/react'
import { useForm, SubmitHandler } from "react-hook-form";
import { create, UserResponse, Response } from '../helpers/api'

type Input = {
    username: string,
    email: string,
    password: string,
}
export const Signup = ({ setUser }: { setUser: any }) => {
    const toast = useToast();
    const { register, handleSubmit } = useForm<Input>();
    const onSubmit: SubmitHandler<Input> = async data => {
        const user: UserResponse & Response = await create(data).catch(val => val);
        if (!user) return;
        if (user.message) {
            toast({
                title: user.message,
                status: 'error',
                duration: 9000,
                variant: 'top-accent',
                position: 'bottom-right',
                isClosable: false
            })
            return;
        }
        toast({
            title: 'Account created! ' + user.username,
            status: 'success',
            duration: 9000,
            variant: 'top-accent',
            position: 'bottom-right',
            isClosable: false
        })
        setUser(user);
    };

    return (
        <Stack>
            {/* Build a responsive signup form */}
            <form onSubmit={handleSubmit(onSubmit)}>
                <FormControl>
                    <FormLabel htmlFor='email'>
                        <Text
                            bgGradient="linear(to-r, red.400,pink.400)"
                            bgClip="text">
                            Email address
                        </Text>
                    </FormLabel>
                    <Input {...register("email", { required: true })} type='email' />
                    <FormHelperText>We'll never share your email.</FormHelperText>
                    <br />
                    <FormLabel htmlFor='username'>
                        <Text
                            bgGradient="linear(to-r, red.400,pink.400)"
                            bgClip="text">
                            Username
                        </Text>
                    </FormLabel>
                    <Input {...register("username", { required: true })} type='text' />
                    <FormHelperText>Pick out a unique one.</FormHelperText>
                    <br />
                    <FormLabel htmlFor='password'>
                        <Text
                            bgGradient="linear(to-r, red.400,pink.400)"
                            bgClip="text">
                            Password
                        </Text>
                    </FormLabel>
                    <Input {...register("password", { required: true })} type='password' />
                    <FormHelperText>At least 8 characters.</FormHelperText>
                    <br />
                    <Button
                        mt={4}
                        bgGradient="linear(to-r, red.400,pink.400)"
                        type='submit'
                    >
                        Submit
                    </Button>
                </FormControl>
            </form>
        </Stack>
    )
}