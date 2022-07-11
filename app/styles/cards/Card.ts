const Card = {
    baseStyle: {
        p: "22px",
        display: "flex",
        flexDirection: "column",
        backdropFilter: "blur(120px)",
        width: "100%",
        borderRadius: "20px",
        bgGradient:"linear(to-r, red.400,pink.400)",
        backgroundClip: "border-box",
    },
};

export const CardComponent = {
    components: {
        Card,
    },
};