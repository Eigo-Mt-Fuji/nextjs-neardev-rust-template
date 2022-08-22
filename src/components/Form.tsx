import React from 'react';
import {FormProps } from "../types";

import {jsx, css} from "@emotion/react";

export default function Form({ onSubmit, currentUser }: FormProps) {

  return (
    <form onSubmit={onSubmit}>
      <fieldset id="fieldset">
        <h2>{{ currentUser.accountId }}</h2>
        <p>Put 3 note urls - your awesome study commit!</p>
        <p className="highlight">
          <label htmlFor="url1">Url#1:</label>
          <input
            autoComplete="off"
            autoFocus
            id="url1"
            required
          />
        </p>
        <p className="highlight">
          <label htmlFor="url2">Url#2:</label>
          <input
            autoComplete="off"
            autoFocus
            id="url2"
            required
          />
        </p>
        <p className="highlight">
          <label htmlFor="url3">Url#3:</label>
          <input
            autoComplete="off"
            autoFocus
            id="url3"
            required
          />
        </p>
        <button type="submit">
          Update
        </button>
      </fieldset>
    </form>
  );
}