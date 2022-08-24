import {combineReducers} from "@reduxjs/toolkit";
import todoReducer from "./features/todos/todoSlice";

// https://redux.js.org/tutorials/fundamentals/part-3-state-actions-reducers#combinereducers
const reducers = combineReducers(    {
    todos: todoReducer,
} );

export default reducers;