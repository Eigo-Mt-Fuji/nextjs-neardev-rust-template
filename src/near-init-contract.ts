import * as nearAPI from 'near-api-js';

import getConfig from './near-env-config';
import { ContractMethods } from "near-api-js/lib/contract";

// Initializing contract
export default async function initContract() {
    const nearConfig = getConfig(process.env.NODE_ENV || 'testnet');
  
    // Initializing connection to the NEAR TestNet
    const near = await nearAPI.connect({
      keyStore: new nearAPI.keyStores.BrowserLocalStorageKeyStore(),
      ...nearConfig
    });
  
    // Needed to access wallet
    const walletConnection = new nearAPI.WalletConnection(near, null);
  
    // Load in account data
    let currentUser;
    if(walletConnection.getAccountId()) {
      currentUser = {
        accountId: walletConnection.getAccountId(),
        balance: (await walletConnection.account().state()).amount
      };
    }
  
    // Initializing our contract APIs by contract name and configuration
    const options: ContractMethods = {
      // View methods are read-only – they don't modify the state, but usually return some value
      viewMethods: ['get_status'],
      // Change methods can modify the state, but you don't receive the returned value when called
      changeMethods: ['set_status'],
      // ContractMethods has no sender
      // // Sender is the account ID to initialize transactions.
      // // getAccountId() will return empty string if user is still unauthorized
      // sender: walletConnection.getAccountId()
    };
    const contract = await new nearAPI.Contract(walletConnection.account(), nearConfig.contractName, options);
  
    return { contract, currentUser, nearConfig, walletConnection };
  }
  