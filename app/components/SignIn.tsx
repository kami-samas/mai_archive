import {
    FormControl,
    FormLabel,
    FormErrorMessage,
    FormHelperText,
    Stack,
    Input,
    Text,
    Heading,
    Button
} from '@chakra-ui/react'
import { useForm, SubmitHandler } from "react-hook-form";

type Input = {
    email: string,
    password: string,
}
export const Signin = () => {
    const { register, handleSubmit, formState: { errors } } = useForm<Input>();
   // TODO
    const onSubmit: SubmitHandler<Input> = async data => {
        console.log(data);
    };
    
    return (
        <Stack>
            {/* Build a responsive signup form */}
            <Stack
                flex={{ base: 1, md: 0 }}
                justify="flex-centre"
                direction="column"
                spacing={6}
                padding={6}
            >
                <Heading as="h1" size="xl" variant={"ghost"}>
                    <Text
                        bgGradient="linear(to-r, red.400,pink.400)"
                        bgClip="text">
                        Sign Up
                    </Text>
                </Heading>
                <form onSubmit={handleSubmit(onSubmit)}>
                    <FormControl>
                        <FormLabel htmlFor='email'>
                            <Text
                                bgGradient="linear(to-r, red.400,pink.400)"
                                bgClip="text">
                                Email address
                            </Text>
                        </FormLabel>
                        <Input {...register("email", { required: true })} type='email'  />
                        <FormHelperText>We'll never share your email.</FormHelperText>
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
        </Stack>
    )
}