import { extendTheme } from '@chakra-ui/react'
import { mode } from '@chakra-ui/theme-tools'
import { CardBodyComponent, CardComponent, CardHeaderComponent } from '../styles'

export default extendTheme({
    initialColorMode: 'dark',
    useSystemColorMode: false,
    fonts: {
        heading: "Greycliff CF, sans-serif",
        body: "Greycliff CF, sans-serif"
    },
    styles: {
        global: (props: any) => ({
            html: {
                bg: "grey.800",
            },
            body: {
                bg: mode("grey.50", "grey.800")(props),
                WebkitTapHighlightColor: "transparent"
            }
        })
    }
}, CardBodyComponent, CardHeaderComponent, CardComponent)