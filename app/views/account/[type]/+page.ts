import type { EntryGenerator } from './$types';

export const entries: EntryGenerator = () => {
    return [
        { type: "signin" },
        { type: "signup" },
        { type: "password-reset" }
    ];
};

export const prerender = false;