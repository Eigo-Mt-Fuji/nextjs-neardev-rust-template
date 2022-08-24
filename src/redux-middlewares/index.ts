import { Middleware } from 'redux'

import { RootState } from '../redux-store';

// why 1st type is empty object? : Most middleware do not modify the dispatch return value, so {}.

// print1
export const print1: Middleware<{}, RootState> = storeApi => next => action => {
    const state = storeApi.getState() // correctly typed as RootState
    console.log("print1");
}

// print2
export const print2: Middleware<{}, RootState> = storeApi => next => action => {
    const state = storeApi.getState() // correctly typed as RootState
    console.log("print1");
}

// print3
export const print3: Middleware<{}, RootState> = storeApi => next => action => {
    const state = storeApi.getState() // correctly typed as RootState
    console.log("print1");
}
