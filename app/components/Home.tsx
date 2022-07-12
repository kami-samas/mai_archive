import {
    Flex, Container, Text, SimpleGrid, Box, Heading, Stat, StatHelpText, StatLabel, StatNumber,
} from '@chakra-ui/react'
import { useEffect, useState } from 'react';
import { get_info } from '../helpers/api';
import { Card, CardBody } from './Card'
import { IconBox, DocumentIcon, PersonIcon } from './Icons'
import { AniNumber } from './AniNumber'

export const Home = () => {
    const [info, setInfo] = useState<{ projects: number; users: number }>({ projects: 0, users: 0 });
    useEffect(() => {
        get_info().then(setInfo);
    }, []);

    return (
        <Container display='flex'>
            <Flex flexDirection='column' pt={{ base: "full", md: "50px" }}>
                <SimpleGrid>
                    <Card p="25px" width="400%">
                        <CardBody>
                            <Flex flexDirection='row' align='center' justify='center' w='100%'>
                                <Stat me='auto'>
                                    <StatLabel
                                        fontSize='xl'
                                        color='primary.100' 
                                        fontWeight='bold'
                                        pb='2px'>
                                        Projects
                                    </StatLabel>
                                    <Flex>
                                        <StatNumber fontSize='lg'>
                                            <AniNumber val={info.projects} concat={"+"}/>
                                        </StatNumber>
                                        <StatHelpText
                                            alignSelf='flex-end'
                                            justifySelf='flex-end'
                                            m='0px'
                                            color='green.400'
                                            fontWeight='bold'
                                            ps='3px'
                                            fontSize='md'>
                                            
                                        </StatHelpText>
                                    </Flex>
                                </Stat>
                                <IconBox h={"45px"} w={"45px"} bg='brand.200'>
                                    <DocumentIcon h={"24px"} w={"24px"} color='#fff' />
                                </IconBox>
                            </Flex>
                        </CardBody>
                    </Card>
                    <br />
                    <Card p="25px" width="400%">
                        <CardBody>
                            <Flex flexDirection='row' align='center' justify='center' w='100%'>
                                <Stat me='auto'>
                                    <StatLabel
                                        fontSize='xl'
                                        color='primary.100' 
                                        fontWeight='bold'
                                        pb='2px'>
                                        Users
                                    </StatLabel>
                                    <Flex>
                                        <StatNumber fontSize='lg'>
                                            <AniNumber val={info.users} concat={"+"} />
                                        </StatNumber>
                                        <StatHelpText
                                            alignSelf='flex-end'
                                            justifySelf='flex-end'
                                            m='0px'
                                            color='green.400'
                                            fontWeight='bold'
                                            ps='3px'
                                            fontSize='md'>
                                            
                                        </StatHelpText>
                                    </Flex>
                                </Stat>
                                <IconBox h={"45px"} w={"45px"} bg='brand.200'>
                                    <PersonIcon h={"24px"} w={"24px"} color='#fff' />
                                </IconBox>
                            </Flex>
                        </CardBody>
                    </Card>
                </SimpleGrid>
            </Flex>
        </Container>
    )
}