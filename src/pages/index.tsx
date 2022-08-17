import React, { useState, useEffect } from "react";
import type { NextPage } from 'next'
import styles from '../styles/Home.module.css'
import Big from "big.js";
import Form from "../components/Form";
import * as nearAPI from 'near-api-js';

import {NearUserView, NearContractContext, NearContractStatusMessageMethods} from "../types";
import initContract from "../near-init-contract";

const BOATLOAD_OF_GAS = Big(3).times(10 ** 13).toFixed();

const Home: NextPage = () => {

  const [status, setStatus] = useState<string|undefined>(undefined);
  const [nearContext, setNearContext] = useState<NearContractContext| undefined>(undefined);

  useEffect(() => {

    const initContractHandler = async () => {
    
      if (!nearContext) {
        initContract()
        .then(({ contract, currentUser, nearConfig, walletConnection }) => {
          setNearContext({
            contract: contract as nearAPI.Contract & NearContractStatusMessageMethods,
            currentUser,
            nearConfig,
            wallet: walletConnection,
          });
        });
  
      }
  
      if (nearContext && nearContext.contract && nearContext.currentUser) {
        const status = await nearContext.contract.get_status({
          account_id: nearContext.currentUser.accountId
        });
  
        setStatus(status);
      }
    };
    if (nearContext == undefined) {

      initContractHandler();
    }
  });

  if (!nearContext) {

    return (
      <div>loading</div>
    );
  } else if(nearContext.contract == undefined) {

    return (
      <div>loading</div>
    );
  }

  const contract: NearContractStatusMessageMethods = nearContext.contract as NearContractStatusMessageMethods;
  const currentUser: NearUserView = nearContext.currentUser as NearUserView;
  const wallet: nearAPI.WalletConnection = nearContext.wallet as nearAPI.WalletConnection;

  const onClickSignIn = () => {
    wallet.requestSignIn(
      {
        contractId: nearContext.nearConfig.contractName, 
        methodNames: ['set_status']
      },
      "NEAR Status Message"
    );
  };

  const onClickSignOut = () => {
    wallet.signOut();
    window.location.replace(window.location.origin + window.location.pathname);
  };

  const onSubmit = async (event:any) => {
    event.preventDefault();

    const { fieldset, message } = event.target.elements;
    fieldset.disabled = true;

    console.log(process.env.NEXT_PUBLIC_CONTRACT_NAME);

    // call smartcontract set_status method.
    await contract.set_status(
      {
        message: message.value,
        account_id: currentUser.accountId
      },
      BOATLOAD_OF_GAS
    );

    // obtain status after smartcontract set_status success.
    const status: string = await contract.get_status({
      account_id: currentUser.accountId
    });
    setStatus(status);
  
    message.value = "";
    fieldset.disabled = false;
    message.focus();
  };

  const onClickMyButton = (event: any) => {
    alert("Hello");
  }

  return (
    <div className={styles.container}>
      <main>
        <header>
          <h1>NEAR Status Message</h1>

          {nearContext.currentUser ?
            <p>Currently signed in as: <code>{nearContext.currentUser.accountId}</code></p>
          :
            <p>Update or add a status message! Please login to continue.</p>
          }

          { nearContext.currentUser
            ? <button onClick={onClickSignOut}>Log out</button>
            : <button id="signin" onClick={onClickSignIn}>Log in</button>
          }
        </header>

        {nearContext.currentUser &&
          <Form
            onClickHello={onClickMyButton}
            onSubmit={onSubmit}
            currentUser={nearContext.currentUser}
          />
        }
        {status ?
          <>
            <p>Your current status:</p>
            <p>
              <code>
                {status}
              </code>
            </p>
          </> : <p>No status message yet!</p>
        }
      </main>
    </div>
  );
 
  }

export default Home
