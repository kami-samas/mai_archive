import localforage from 'localforage';
import { useState, useLayoutEffect } from 'react';
export { localforage };

export function useLocalForage(key: string, defaultValue = {}) {
    // only supports primitives, arrays, and {} objects
    const [state, setState] = useState<any>(defaultValue);
    const [loading, setLoading] = useState<boolean>(true);

    // @ts-ignore
    useLayoutEffect(() => {
        let allow = true;
        localforage.getItem(key).then(value => {
            if (value === null) throw '';
            if (allow) setState(value);
        }).catch(() => localforage.setItem(key, defaultValue)).then(() => {
            if (allow) setLoading(false);
        });
        return () => allow = false;
    }, []);
    useLayoutEffect(() => {
        // do not allow setState to be called before data has even been loaded!
        if (!loading) localforage.setItem(key, state);
    }, [state]);
    return [state, setState, loading];
}