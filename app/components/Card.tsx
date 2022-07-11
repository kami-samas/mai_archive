import { BoxProps, Box, useStyleConfig, ThemingProps } from '@chakra-ui/react'

export const CardHeader = (props: BoxProps & ThemingProps) => {
    const { variant, children, ...rest } = props;
    const styles = useStyleConfig("CardHeader", { variant });
    // Pass the computed styles into the `__css` prop
    return (
        <Box __css={styles} {...rest}>
            {children}
        </Box>
    );
}

export const CardBody = (props: BoxProps & ThemingProps) => {
    const { variant, children, ...rest } = props;
    const styles = useStyleConfig("CardBody", { variant });
    // Pass the computed styles into the `__css` prop
    return (
        <Box __css={styles} {...rest}>
            {children}
        </Box>
    );
}

export const Card = (props: BoxProps & ThemingProps) => {
    const { variant, children, ...rest } = props;
    const styles = useStyleConfig("Card", { variant });

    // Pass the computed styles into the `__css` prop
    return (
      <Box __css={styles} {...rest}>
        {children}
      </Box>
    );
}