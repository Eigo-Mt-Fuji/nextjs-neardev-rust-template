import { useState, useEffect } from "react";
import type { NextPage } from 'next'

import Form from "../components/Form";
import styles from '../styles/Home.module.css'

import * as nearAPI from 'near-api-js';
import initContract from "../near-init-contract";
import Big from "big.js";
import {NearUserView, NearContractContext, SteadyStudyTokenContractMethods} from "../types";
import { useSelector, useDispatch } from 'react-redux'
import {RootState} from "../redux-store";
import { add, update } from '../features/todos/todoSlice';


const BOATLOAD_OF_GAS = Big(3).times(10 ** 13).toFixed();

const Home: NextPage = () => {

  const [balance, setBalance] = useState<string|undefined>(undefined);
  const [nearContext, setNearContext] = useState<NearContractContext| undefined>(undefined);
  const count = useSelector( (state: RootState) => state.todos.value)
  const dispatch = useDispatch()

  useEffect(() => {

    const initContractHandler = async () => {
    
      if (!nearContext) {
        initContract()
        .then(({ contract, currentUser, nearConfig, walletConnection }) => {
          setNearContext({
            contract: contract as nearAPI.Contract & SteadyStudyTokenContractMethods,
            currentUser,
            nearConfig,
            wallet: walletConnection,
          });
        });
      }
  
      if (nearContext && nearContext.contract && nearContext.currentUser) {
        // call NEP-141 FungibleToken::ft_balance_of(&self)
        const balance = await nearContext.contract.ft_balance_of({
          account_id: nearContext.currentUser.accountId
        });
  
        setBalance(balance);
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

  const contract = nearContext.contract as SteadyStudyTokenContractMethods;
  const currentUser: NearUserView = nearContext.currentUser as NearUserView;
  const wallet: nearAPI.WalletConnection = nearContext.wallet as nearAPI.WalletConnection;

  const onClickSignIn = () => {
    wallet.requestSignIn(
      {
        contractId: nearContext.nearConfig.contractName, 
        methodNames: ['ft_balance_of', 'ft_report_study_commit']
      },
      "SteadyStudyToken"
    );
  };

  const onClickSignOut = () => {
    wallet.signOut();
    window.location.replace(window.location.origin + window.location.pathname);
  };

  const onSubmit = async (event:any) => {
    event.preventDefault();

    const { fieldset, url1, url2, url3 } = event.target.elements;
    fieldset.disabled = true;

    console.log(process.env.NEXT_PUBLIC_CONTRACT_NAME);

    // call smartcontract set_balance method.
    await contract.ft_report_study_commit(
      {
        urls: [url1.value, url2.value, url3.value],
        receiver_id: currentUser.accountId
      },
      BOATLOAD_OF_GAS
    );

    // obtain balance after smartcontract set_balance success.
    const balance: string = await contract.ft_balance_of({
      account_id: currentUser.accountId
    });
    setBalance(balance);
  
    url1.value = "";
    url2.value = "";
    url3.value = "";
    fieldset.disabled = false;
    url1.focus();
  };

  return (
    <div className={styles.container}>
      <main>
        <header>
          <h1>NEAR balance Message</h1>
          <button
            aria-label="Add value"
            onClick={(_event) => dispatch(add("hoge"))}
          >
            Add
          </button>

          {nearContext.currentUser ?
            <p>Currently signed in as: <code>{nearContext.currentUser.accountId}</code></p>
          :
            <p>Update or add a balance message! Please login to continue.</p>
          }

          { nearContext.currentUser
            ? <button onClick={onClickSignOut}>Log out</button>
            : <button id="signin" onClick={onClickSignIn}>Log in</button>
          }
        </header>

        {nearContext.currentUser &&
          <Form
            onSubmit={onSubmit}
            currentUser={nearContext.currentUser}
          />
        }
        {balance ?
          <>
            <p>Your token balance:</p>
            <p>
              <code>
                {balance}
              </code>
            </p>
          </> : <p>No STEADYST token yet!</p>
        }
      </main>
    </div>
  );
 
}

export default Home
