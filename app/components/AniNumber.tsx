// animated number
import { useSpring, animated } from 'react-spring';

export const AniNumber = (props: { val: number; concat: string }) => {
    const { val, concat } = props;
    const num = useSpring({ val, from: { val: 0 } })
    return (
        <animated.div>
            {`${num.val.to(val => Math.floor(val)).get()}${concat}`}
        </animated.div>
    )
}