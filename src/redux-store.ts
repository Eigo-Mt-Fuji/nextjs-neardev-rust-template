

import {configureStore, EnhancedStore} from "@reduxjs/toolkit";
import rootReducer from './redux-reducer';
import { print1, print2, print3 } from './redux-middlewares/index';

// https://redux-toolkit.js.org/api/configureStore
// recommend using the configureStore method of the @reduxjs/toolkit package, which replaces createStore.
// what configureStore: A friendly abstraction over the standard Redux createStore function that adds good defaults to the store setup for a better development experience
const store: EnhancedStore = configureStore({

    // reducer: Reducer<S, A> | ReducersMapObject<S, A>
    reducer: rootReducer,

    // middleware?: ((getDefaultMiddleware: CurriedGetDefaultMiddleware<S>) => M) | M : An array of Redux middleware to install. If not supplied, defaults to the set of middleware returned by `getDefaultMiddleware()`.
    middleware: () => {

        return [print1, print2, print3];
    },
    // devTools?: boolean | DevToolsOptions : Whether to enable Redux DevTools integration. Defaults to `true`. Additional configuration can be done by passing Redux DevTools options
    devTools: true
    // preloadedState?: DeepPartial<S extends any ? S : S> : The initial state, same as Redux's createStore. You may optionally specify it to hydrate the state
    // enhancers : The store enhancers to apply. See Redux's `createStore()`.
});

// Infer the `RootState` and `AppDispatch` types from the store itself
export type RootState = ReturnType<typeof store.getState>
// Inferred type: {posts: PostsState, comments: CommentsState, users: UsersState}
export type AppDispatch = typeof store.dispatch

export default store;
