import { createSlice } from '@reduxjs/toolkit'

interface TodoItem {
    value: string;
    status: string;
}
interface TodoItemState {
    value: TodoItem[];
}
const initialState: TodoItemState = {
    value: []
};

interface TodoReducerAction {
    type: string;
    payload: string;
    status: string;
}

export const todoSlices = createSlice({
    name: 'todos',
    initialState: initialState,
    reducers: {
      add: (state, action) => {
        state.value.push({ value: action.payload, status: status });
        return { ...state };
      },
      update: (state, action) => {

        const target = state.value.find((v, i) => {

            return v.value == action.payload;
        });
        if ( target !== undefined ) { 
            target.value = action.payload;
        }
        return { ...state };
      },
      delete: (state, action) => {
        state.value = state.value.filter( (v, _index) => {

            return v.value != action.payload;
        });
        return { ...state };
      },
    }
});

export const { add, update } = todoSlices.actions;

export default todoSlices.reducer;
  