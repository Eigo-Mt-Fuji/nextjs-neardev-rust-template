import React from 'react';
import {FormProps } from "../types";

import {jsx, css} from "@emotion/react";

export default function Form({ onSubmit, onClickHello, currentUser }: FormProps) {

  return (
    <form onSubmit={onSubmit}>
      <fieldset id="fieldset">
        <p>Add or update your status message!</p>
        <p className="highlight">
          <label htmlFor="message">Message:</label>
          <input
            autoComplete="off"
            autoFocus
            id="message"
            required
          />
        </p>
        <button css={css`
          background-color: "#000000";
          font-size: 20px;
          color: "#ffffff";
        `} onClick={onClickHello}>hello</button>
        <button type="submit">
          Update
        </button>
      </fieldset>
    </form>
  );
}